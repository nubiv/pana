use crate::app_event;
use crate::services::downloader::download;
use crate::services::llm::set_model;
use crate::utils::events::*;
use crate::utils::models::{
    get_model_info, read_model_list,
};

#[tauri::command]
pub fn update_llm_models(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
) -> Result<(), String> {
    read_model_list(&app_handle, &window)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn download_model(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    model_name: String,
    download_state: tauri::State<crate::DownloadState>,
) -> Result<(), String> {
    use crate::utils::models::ModelList;

    let config_dir_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .ok_or(String::from(
            "Failed to resolve resource path.",
        ))?;

    let config_file_path =
        config_dir_path.join("models.json");
    let model_config =
        std::fs::File::open(config_file_path).map_err(
            |e| format!("Failed to read configs: {}", e),
        )?;
    let model_list: ModelList =
        serde_json::from_reader(model_config).map_err(
            |e| format!("Failed to read configs: {}", e),
        )?;

    let bin_dir_path = config_dir_path.join("bin");

    let download_handle =
        tauri::async_runtime::spawn(async move {
            let target_model = match model_list
                .models
                .iter()
                .find(|model| model.name == model_name)
            {
                Some(model) => model,
                None => {
                    app_event!(
                        &window,
                        Error,
                        ErrorPayload {
                            message: String::from(
                                "Model not found."
                            )
                        }
                    );

                    return;
                }
            };

            if let Err(e) = download(
                &window,
                &bin_dir_path,
                target_model,
            )
            .await
            {
                app_event!(
                    &window,
                    Error,
                    ErrorPayload {
                        message: format!(
                            "Failed to download model: {}",
                            e
                        )
                    }
                );
            }
        });

    let mut abort_handlers_guard =
        download_state.abort_handlers.lock().unwrap();
    *abort_handlers_guard = Some(download_handle);

    Ok(())
}

#[tauri::command]
pub fn stop_download(
    download_state: tauri::State<crate::DownloadState>,
) -> Result<(), String> {
    let mut abort_handlers_guard =
        download_state.abort_handlers.lock().map_err(|_| {
            String::from(
                "Something went wrong while attempting to stop downloading.",
            )
        })?;

    if let Some(handle) = abort_handlers_guard.take() {
        handle.abort();
    }

    Ok(())
}

#[tauri::command]
pub fn delete_model(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    model_name: String,
) -> Result<(), String> {
    use crate::utils::models::delete_model;

    delete_model(&app_handle, &window, &model_name)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn load_model(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    llm_state: tauri::State<crate::LLMState>,
    model_name: String,
) -> Result<(), String> {
    let model_info =
        get_model_info(&app_handle, &model_name)
            .map_err(|e| e.to_string())?;
    set_model(&llm_state, &app_handle, &model_info)
        .map_err(|e| e.to_string())?;

    app_event!(
        &window,
        Noticification,
        NoticificationPayload {
            message: String::from("Model loaded.")
        }
    );

    Ok(())
}

#[tauri::command]
pub fn unload_model(
    window: tauri::Window,
    llm_state: tauri::State<crate::LLMState>,
) -> Result<(), String> {
    let mut model_guard = llm_state.model.lock().map_err(|_| {
        String::from("Something went wrong while attempting to unload model.")
    })?;
    *model_guard = None;

    app_event!(
        &window,
        Noticification,
        NoticificationPayload {
            message: String::from("Model unloaded.")
        }
    );

    Ok(())
}

#[tauri::command]
pub fn start_inference(
    db_state: tauri::State<crate::DBState>,
    llm_state: tauri::State<crate::LLMState>,
    window: tauri::Window,
    message: String,
) -> Result<(), String> {
    let model = llm_state.model.clone();
    let abort_handle = llm_state.abort_handle.clone();
    let db = db_state.db.clone();

    // for kv in db.iter() {
    //     let kv = kv.unwrap();
    //     crate::db::print_kv(&kv.0, &kv.1);
    // }

    let tree = crate::db::setup_tree(&db)
        .map_err(|e| e.to_string())?;

    // for kv in tree.iter() {
    //     let kv = kv.unwrap();
    //     crate::db::print_kv(&kv.0, &kv.1);
    // }

    let window = std::sync::Arc::new(window);
    crate::services::llm::infer(
        window,
        message,
        model,
        abort_handle,
        std::sync::Arc::new(tree),
    );

    // let mut abort_handle_guard = llm_state
    //     .abort_handle
    //     .lock()
    //     .expect("failed to get abort_handle lock");
    // *abort_handle_guard = Some(handle);
    Ok(())
}

#[tauri::command]
pub fn stop_inference(
    llm_state: tauri::State<crate::LLMState>,
) -> Result<(), String> {
    // Since inference session will consistantly check the abort_handle
    // to see if it should stop, relaxed ordering is fine here.
    llm_state
        .abort_handle
        .store(true, std::sync::atomic::Ordering::Relaxed);

    // Closures spawned using spawn_blocking cannot be cancelled abruptly using handle abort;
    // there is no standard low level API to cause a thread to stop running.

    // let mut abort_handle_guard = llm_state
    //     .abort_handle
    //     .lock()
    //     .expect("failed to get abort handle lock");

    // if let Some(handle) = abort_handle_guard.take() {
    //     handle.abort();
    // }
    Ok(())
}

#[tauri::command]
pub fn open_model_folder(
    path: String,
) -> Result<(), String> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match std::fs::metadata(&path)
                .unwrap()
                .is_dir()
            {
                true => path,
                false => {
                    let mut path2 =
                        std::path::PathBuf::from(path);
                    path2.pop();
                    path2
                        .into_os_string()
                        .into_string()
                        .unwrap()
                }
            };
            Command::new("xdg-open")
                .arg(&new_path)
                .spawn()
                .map_err(|e| e.to_string())?;
        } else {
            Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:\"file://{path}\"").as_str(),
                    "string:\"\"",
                ])
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-R", &path])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
