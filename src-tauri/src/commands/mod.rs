use crate::app_event;
use crate::services::downloader::download;
use crate::services::llm::set_model;
use crate::utils::events::*;
use crate::utils::models::{get_model_info, read_model_list};

#[tauri::command]
pub fn update_llm_models(app_handle: tauri::AppHandle, window: tauri::Window) {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ModelPayload>(10);

    read_model_list(&app_handle, &window, tx);

    tauri::async_runtime::spawn(async move {
        while let Some(model_payload_info) = rx.recv().await {
            app_event!(&window, Model, model_payload_info);
        }
    });
}

#[tauri::command]
pub fn download_model(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    model_name: String,
    download_state: tauri::State<crate::DownloadState>,
) {
    use crate::utils::models::ModelList;

    let config_dir_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .expect("failed to resolve resource");

    let config_file_path = config_dir_path.join("models.json");
    let model_config = std::fs::File::open(config_file_path).unwrap();
    let model_list: ModelList = serde_json::from_reader(model_config).unwrap();

    let bin_dir_path = config_dir_path.join("bin");

    let download_handle = tauri::async_runtime::spawn(async move {
        let target_model = match model_list
            .models
            .iter()
            .find(|model| model.name == model_name)
        {
            Some(model) => model,
            None => panic!("model not found"),
        };

        download(&window, &bin_dir_path, target_model)
            .await
            .expect("failed to download");
    });

    let mut abort_handlers_guard =
        download_state.abort_handlers.lock().unwrap();
    *abort_handlers_guard = Some(download_handle);
}

#[tauri::command]
pub fn stop_download(download_state: tauri::State<crate::DownloadState>) {
    let mut abort_handlers_guard =
        download_state.abort_handlers.lock().unwrap();

    if let Some(handle) = abort_handlers_guard.take() {
        handle.abort();
    }
}

#[tauri::command]
pub fn delete_model(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    model_name: String,
) {
    use crate::utils::models::{delete_model, get_model_info};

    let model_info = get_model_info(&app_handle, &model_name);
    let model_path = app_handle
        .path_resolver()
        .resolve_resource(format!("./models/bin/{}", model_info.filename))
        .expect("failed to resolve resource");
    delete_model(&window, &model_path).expect("failed to delete model");
}

#[tauri::command]
pub fn load_model(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    llm_state: tauri::State<crate::LLMState>,
    model_name: String,
) {
    let model_info = get_model_info(&app_handle, &model_name);
    set_model(&llm_state, &app_handle, &model_info);

    app_event!(
        &window,
        Noticification,
        NoticificationPayload {
            message: String::from("Model loaded.")
        }
    );
}

#[tauri::command]
pub fn stop_model(
    window: tauri::Window,
    llm_state: tauri::State<crate::LLMState>,
) {
    let mut model_guard = llm_state.model.lock().unwrap();
    *model_guard = None;

    app_event!(
        &window,
        Noticification,
        NoticificationPayload {
            message: String::from("Model unloaded.")
        }
    );
}

#[tauri::command]
pub fn start_inference(
    llm_state: tauri::State<crate::LLMState>,
    window: tauri::Window,
    message: String,
) {
    let model = llm_state.model.clone();
    let abort_handle = llm_state.abort_handle.clone();

    // app_event!(
    //     &window,
    //     Response,
    //     ResponsePayload {
    //         is_streaming: true,
    //         token: String::from("")
    //     }
    // );

    let window = std::sync::Arc::new(window);
    crate::services::llm::infer(window, message, model, abort_handle);

    // let mut abort_handle_guard = llm_state
    //     .abort_handle
    //     .lock()
    //     .expect("failed to get abort_handle lock");
    // *abort_handle_guard = Some(handle);
}

#[tauri::command]
pub fn stop_inference(llm_state: tauri::State<crate::LLMState>) {
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
}

#[tauri::command]
pub fn open_model_folder(path: String) {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match std::fs::metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = std::path::PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            Command::new("xdg-open").arg(&new_path).spawn().unwrap();
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
                .unwrap();
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}
