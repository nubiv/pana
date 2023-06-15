use anyhow::anyhow;
use std::{error::Error, fs};

use crate::utils::errors::AppError;

pub fn is_llm_dir_existed(
    app_handle: &tauri::AppHandle,
) -> Result<bool, AppError> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(AppError::Custom(anyhow!("Failed to get app data dir.")))?;
    let llm_dir = app_data_dir.join("llm");

    match fs::read_dir(llm_dir) {
        Ok(_) => Ok(true),
        Err(e) => Ok(false),
    }
}

pub fn create_llm_dir(app_handle: &tauri::AppHandle) -> Result<(), AppError> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(AppError::Custom(anyhow!("Failed to get app data dir.")))?;
    let llm_dir = app_data_dir.join("llm");

    match fs::create_dir(llm_dir) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(AppError::Custom(anyhow!("Failed to create llm dir: {}", e)))
        }
    }
}

pub fn find_local_models(
    app_handle: &tauri::AppHandle,
) -> Result<Vec<std::path::PathBuf>, AppError> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(AppError::Custom(anyhow!("Failed to get app data dir.")))?;
    let llm_dir = app_data_dir.join("llm");

    let files = match fs::read_dir(llm_dir) {
        Ok(files) => files,
        Err(_) => {
            return Err(AppError::Custom(anyhow!("Failed to read llm dir.")))
        }
    };
    let mut models = vec![];
    // Iterate over the files and return the first `.bin` file found
    for file in files {
        let path = match file {
            Ok(file) => file.path(),
            Err(_) => continue,
        };
        let extension = match path.extension() {
            Some(extension) => extension.to_string_lossy(),
            None => continue,
        };
        if extension == "bin" {
            models.push(path);
        }
    }

    Ok(models)
}

pub enum Models {
    Wizard7B,
    Mpt7B,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub filename: String,
    pub architecture: String,
    pub download_url: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ModelList {
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModelPayloadInfo {
    pub name: String,
    pub size: u64,
    pub total_size: u64,
}

pub fn read_model_list(
    app_handle: &tauri::AppHandle,
    tx: tokio::sync::mpsc::Sender<ModelPayloadInfo>,
) {
    let model_config_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .expect("failed to resolve resource");

    let config_file = model_config_path.join("models.json");
    let model_config = std::fs::File::open(config_file).unwrap();
    let model_list: ModelList = serde_json::from_reader(model_config).unwrap();

    let bin_path = model_config_path.join("bin");

    match fs::read_dir(&bin_path) {
        Ok(_) => {}
        Err(_) => {
            println!("bin dir not existed creating...");

            if let Err(e) = fs::create_dir(&bin_path) {
                println!("failed to create bin dir: {}", e);
                return;
            }
        }
    }

    model_list.models.iter().for_each(|model| {
        let current_model_path = bin_path.join(&model.filename);

        let size = match fs::File::open(current_model_path) {
            Ok(model_info) => model_info.metadata().unwrap().len(),
            Err(_) => 0,
        };

        let model = model.clone();
        let tx = tx.clone();
        tauri::async_runtime::spawn(async move {
            let client = reqwest::Client::new();
            let res = client.get(&model.download_url).send().await;

            let total_size = res.unwrap().content_length().unwrap();
            let model_payload_info = ModelPayloadInfo {
                name: model.name.clone(),
                size,
                total_size,
            };

            tx.send(model_payload_info).await.expect("failed to send");
        });
    });
}

pub fn get_model_info(
    app_handle: &tauri::AppHandle,
    model_name: &str,
) -> ModelInfo {
    let config_dir_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .expect("failed to resolve resource");

    let config_file_path = config_dir_path.join("models.json");
    let model_config = std::fs::File::open(config_file_path).unwrap();
    let model_list: ModelList = serde_json::from_reader(model_config).unwrap();

    let target_model = match model_list
        .models
        .iter()
        .find(|model| model.name == model_name)
    {
        Some(model) => model,
        None => panic!("model not found"),
    };

    target_model.to_owned()
}

pub fn delete_model(model_path: &std::path::Path) -> Result<(), AppError> {
    fs::remove_file(model_path).expect("failed to delete model file");

    Ok(())
}
