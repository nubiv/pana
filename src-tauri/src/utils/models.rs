use anyhow::anyhow;
use std::fs;

use crate::app_event;
use crate::utils::errors::AppError;
use crate::utils::events::*;

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

pub fn read_model_list(
    app_handle: &tauri::AppHandle,
    window: &tauri::Window,
    tx: tokio::sync::mpsc::Sender<ModelPayload>,
) {
    let model_config_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .expect("failed to resolve resource");

    let config_file = model_config_path.join("models.json");
    let model_config = fs::File::open(config_file).unwrap();
    let model_list: ModelList = serde_json::from_reader(model_config).unwrap();

    let bin_path = model_config_path.join("bin");

    match fs::read_dir(&bin_path) {
        Ok(_) => {}
        Err(_) => {
            if let Err(e) = fs::create_dir(&bin_path) {
                println!("failed to create bin dir: {}", e);

                app_event!(
                    window,
                    Error,
                    ErrorPayload {
                        message: String::from("Failed to create bin folder.")
                    }
                );

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
            let model_payload_info = ModelPayload {
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
    let model_config = fs::File::open(config_file_path).unwrap();
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

pub fn delete_model(
    window: &tauri::Window,
    model_path: &std::path::Path,
) -> Result<(), AppError> {
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
        Err(e) => {
            println!("{}", e);

            app_event!(
                window,
                Error,
                ErrorPayload {
                    message: String::from("Failed to delete model.")
                }
            );

            Err(AppError::Custom(anyhow!("Failed to delete model.")))
        }
    }
}
