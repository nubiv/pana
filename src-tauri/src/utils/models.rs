use anyhow::anyhow;
use std::fs;

use crate::app_event;
use crate::utils::errors::IOError;
use crate::utils::events::*;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub filename: String,
    pub architecture: String,
    pub total_size: u64,
    pub download_url: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ModelList {
    pub models: Vec<ModelInfo>,
}

pub fn read_model_list(
    app_handle: &tauri::AppHandle,
    window: &tauri::Window,
) -> Result<(), IOError> {
    let model_config_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .ok_or(IOError::Custom(anyhow!("Failed to resolve resource path.")))?;

    let config_file = model_config_path.join("models.json");
    let model_config = fs::File::open(config_file)?;
    let model_list: ModelList = serde_json::from_reader(model_config)
        .map_err(|e| IOError::Custom(anyhow!("Failed to read configs: {e}")))?;

    let bin_path = model_config_path.join("bin");

    if fs::read_dir(&bin_path).is_err() {
        fs::create_dir(&bin_path)?;
    }

    model_list
        .models
        .iter()
        .try_for_each(|model| -> Result<(), IOError> {
            let current_model_path = bin_path.join(&model.filename);

            let size = match fs::File::open(current_model_path) {
                Ok(model_info) => {
                    let metadata = model_info.metadata().map_err(|e| {
                        IOError::Custom(anyhow!(
                            "Failed to read model metadata: {e}"
                        ))
                    })?;

                    metadata.len()
                }
                Err(_) => 0,
            };

            let model = model.clone();

            app_event!(
                window,
                Model,
                ModelPayload {
                    name: model.name.clone(),
                    size,
                    total_size: model.total_size,
                }
            );

            Ok(())
        })?;

    Ok(())
}

pub fn get_model_info(
    app_handle: &tauri::AppHandle,
    model_name: &str,
) -> Result<ModelInfo, IOError> {
    let config_dir_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .ok_or(IOError::Custom(anyhow!("Failed to resolve resource path.")))?;

    let config_file_path = config_dir_path.join("models.json");
    let model_config = fs::File::open(config_file_path)?;
    let model_list: ModelList = serde_json::from_reader(model_config)
        .map_err(|e| IOError::Custom(anyhow!("Failed to read configs: {e}")))?;

    let target_model = match model_list
        .models
        .iter()
        .find(|model| model.name == model_name)
    {
        Some(model) => model,
        None => return Err(IOError::Custom(anyhow!("Model not found."))),
    };

    Ok(target_model.to_owned())
}

pub fn delete_model(
    app_handle: &tauri::AppHandle,
    window: &tauri::Window,
    model_name: &str,
) -> Result<(), IOError> {
    let model_info = get_model_info(app_handle, model_name)?;
    let model_path = app_handle
        .path_resolver()
        .resolve_resource(format!("./models/bin/{}", model_info.filename))
        .ok_or(IOError::Custom(anyhow!("Failed to resolve resource path.")))?;

    match fs::remove_file(model_path) {
        Ok(_) => {
            app_event!(
                window,
                Noticification,
                NoticificationPayload {
                    message: String::from("Model deleted.")
                }
            );

            Ok(())
        }
        Err(e) => Err(IOError::Custom(anyhow!("Failed to delete model: {e}"))),
    }
}
