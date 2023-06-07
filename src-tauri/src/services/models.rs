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
) -> Result<Vec<String>, AppError> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or(AppError::Custom(anyhow!("Failed to get app data dir.")))?;
    let llm_dir = app_data_dir.join("llm");

    let mut models = vec![];
    let files = match fs::read_dir(llm_dir) {
        Ok(files) => files,
        Err(_) => {
            return Err(AppError::Custom(anyhow!("Failed to read llm dir.")))
        }
    };
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
            models.push(path.to_str().unwrap().to_string());
        }
    }

    Ok(models)
}
