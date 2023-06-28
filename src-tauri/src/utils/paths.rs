use anyhow::anyhow;

use crate::app_event;
use crate::utils::events::*;

pub fn get_models_path(
    app_handle: &tauri::AppHandle,
    window: &tauri::Window,
) -> Result<std::path::PathBuf, anyhow::Error> {
    let path = match app_handle
        .path_resolver()
        .resolve_resource("./models")
    {
        Some(path) => path,
        None => {
            app_event!(
                window,
                Error,
                ErrorPayload {
                    message: String::from(
                        "Failed to resolve resource path."
                    )
                }
            );

            return Err(anyhow!(
                "Failed to resolve resource path."
            ));
        }
    };

    Ok(path)
}

pub fn get_db_path(
    app_handle: &tauri::AppHandle,
    window: &tauri::Window,
) -> Result<std::path::PathBuf, anyhow::Error> {
    let path =
        match app_handle.path_resolver().app_data_dir() {
            Some(path) => path.join("db"),
            None => {
                app_event!(
                    window,
                    Error,
                    ErrorPayload {
                        message: String::from(
                            "Failed to resolve db path."
                        )
                    }
                );

                return Err(anyhow!(
                    "Failed to resolve db path."
                ));
            }
        };

    Ok(path)
}
