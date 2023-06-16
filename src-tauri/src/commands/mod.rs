use std::sync::Arc;

use llm_chain::output::StreamExt;
use tokio::sync::mpsc;

use crate::app_event;
use crate::services::downloader::download;
use crate::services::llm_chain::LLM;
use crate::utils::events::*;
use crate::Channel;

#[tauri::command]
pub fn update_llm_models(app_handle: tauri::AppHandle, window: tauri::Window) {
    use crate::utils::models::{read_model_list, ModelPayloadInfo};

    let (tx, mut rx) = tokio::sync::mpsc::channel::<ModelPayloadInfo>(10);

    read_model_list(&app_handle, tx);

    tauri::async_runtime::spawn(async move {
        while let Some(model_payload_info) = rx.recv().await {
            window
                .emit("model", model_payload_info)
                .expect("failed to emit");
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

    // put download handle somewhere in case user wants to cancel/pause the download
    // let mut download_state_guard = &download_state.abort_handle;
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
pub fn delete_model(app_handle: tauri::AppHandle, model_name: String) {
    use crate::utils::models::{delete_model, get_model_info};

    let model_info = get_model_info(&app_handle, &model_name);
    let model_path = app_handle
        .path_resolver()
        .resolve_resource(format!("./models/bin/{}", model_info.filename))
        .expect("failed to resolve resource");
    delete_model(&model_path).expect("failed to delete model");
}

#[tauri::command]
pub fn load_model(
    llm_state: tauri::State<crate::LLMState>,
    app_handle: tauri::AppHandle,
    model_name: String,
) {
    use crate::utils::models::get_model_info;

    let model_info = get_model_info(&app_handle, &model_name);
    crate::services::llm::set_model(llm_state, &app_handle, &model_info);
}

#[tauri::command]
pub fn stop_model(llm_state: tauri::State<crate::LLMState>) {
    let mut model_guard = llm_state.model.lock().unwrap();
    *model_guard = None;
}

#[tauri::command]
pub fn start_inference(
    llm_state: tauri::State<crate::LLMState>,
    window: tauri::Window,
    message: String,
) {
    let model = llm_state.model.clone();

    app_event!(
        &window,
        Response,
        ResponsePayload {
            is_streaming: true,
            token: String::from("")
        }
    );

    let handle = tauri::async_runtime::spawn_blocking(move || {
        let model_guard = model.lock().unwrap();
        let model = match &*model_guard {
            Some(model) => model,
            None => panic!("model is not loaded"),
        };
        let mut session = crate::services::llm::new_session(model, &message);

        let mut utf8_buf = llm::TokenUtf8Buffer::new();

        while let Ok(token) = session.infer_next_token(
            model.as_ref(),
            &llm::InferenceParameters::default(),
            &mut Default::default(),
            &mut rand::thread_rng(),
        ) {
            if let Some(token) = utf8_buf.push(&token) {
                app_event!(
                    &window,
                    Response,
                    ResponsePayload {
                        is_streaming: true,
                        token
                    }
                );
            }
        }

        app_event!(
            &window,
            Response,
            ResponsePayload {
                is_streaming: false,
                token: String::from("")
            }
        );
    });

    let mut abort_handle_guard = llm_state
        .abort_handle
        .lock()
        .expect("failed to get abort_handle lock");
    *abort_handle_guard = Some(handle);
}

#[tauri::command]
pub fn stop_inference(llm_state: tauri::State<crate::LLMState>) {
    let mut abort_handle_guard = llm_state
        .abort_handle
        .lock()
        .expect("failed to get abort handle lock");

    // TODO: aborting the handle will not stop the inference
    if let Some(handle) = abort_handle_guard.take() {
        handle.abort();
    }
}

#[tauri::command]
pub fn load_model_v2(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    state: tauri::State<Channel>,
) {
    let (tx, mut rx) = mpsc::channel(10);
    let tx_guard = &mut *state.tx.lock().unwrap();

    match tx_guard {
        Some(_) => {
            let notification_payload = NoticificationPayload {
                message: String::from("Lobot has been running already..."),
            };

            AppEvent::<Noticification>::new(notification_payload).emit(&window);
        }
        None => {
            *tx_guard = Some(tx);

            tauri::async_runtime::spawn(async move {
                let mut llm = match LLM::spawn_llama(&app_handle) {
                    Ok(llm) => llm,
                    Err(e) => {
                        panic!("spawn llama failed {}", e);
                    }
                };

                let notification_payload = NoticificationPayload {
                    message: String::from("Lobot activated..."),
                };

                AppEvent::<Noticification>::new(notification_payload)
                    .emit(&window);

                loop {
                    match rx.recv().await {
                        Some(input) => match llm.feed_input(&input).await {
                            Ok(mut res) => {
                                while let Some(token) = res.next().await {
                                    let response_payload = ResponsePayload {
                                        is_streaming: true,
                                        token: token.to_string(),
                                    };

                                    AppEvent::<Response>::new(response_payload)
                                        .emit(&window);
                                }

                                // let response_payload = ResponsePayload {
                                //     message: res.to_string(),
                                // };

                                // AppEvent::<Response>::new(response_payload)
                                //     .emit(&window);
                            }
                            Err(e) => {
                                println!("run llama block {}", e)
                            }
                        },
                        None => println!("the sender dropped"),
                    }
                }
            });
        }
    }
}

#[tauri::command]
pub fn send_message_v2(
    window: tauri::Window,
    state: tauri::State<Channel>,
    message: String,
) {
    let tx_guard = &*state.tx.lock().unwrap();

    if let Some(tx) = tx_guard {
        let tmp_tx = tx.clone();
        tauri::async_runtime::spawn(async move {
            if tmp_tx.send(message).await.is_err() {
                println!("send message block..")
            }
        });
    } else {
        let notification_payload = NoticificationPayload {
            message: String::from("Wake Lobot up first..."),
        };

        AppEvent::<Noticification>::new(notification_payload).emit(&window);
    };
}

#[tauri::command]
pub fn open_model_folder(path: String) {
    use std::fs;
    use std::fs::metadata;
    use std::path::PathBuf;
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
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
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
