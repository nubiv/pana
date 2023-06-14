use llm_chain::output::StreamExt;
use tokio::sync::mpsc;

use crate::app_event;
use crate::services::downloader::download;
use crate::services::llama::LLM;
use crate::services::models::find_local_models;
use crate::utils::events::*;
use crate::{AppState, Channel};

#[tauri::command]
pub fn run_llama(
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
                                        message: token.to_string(),
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

            // *tx_guard = None;
        }
    }
}

#[tauri::command]
pub fn send_message(
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

#[derive(Clone, serde::Serialize)]
struct Models {
    is_running: bool,
    running_model: String,
    local_models: Vec<String>,
}

#[tauri::command]
pub fn update_llm_models(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    state: tauri::State<AppState>,
) {
    let updated_models = find_local_models(&app_handle).unwrap();
    let models = &mut *state.local_models.try_lock().unwrap();
    *models = updated_models;

    let model_payload = ModelPayload {
        running_model: String::from(""),
        local_models: models.clone(),
    };

    AppEvent::<Model>::new(model_payload).emit(&window);
}

#[tauri::command]
pub fn update_llm_models_v2(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
) {
    use crate::services::models::{read_model_list, ModelPayloadInfo};

    let (tx, mut rx) = tokio::sync::mpsc::channel::<ModelPayloadInfo>(10);

    read_model_list(&app_handle, tx);

    tauri::async_runtime::spawn(async move {
        while let Some(model_payload_info) = rx.recv().await {
            window
                .emit("model_v2", model_payload_info)
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
    use crate::services::models::ModelList;

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
pub fn stop_download(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    download_state: tauri::State<crate::DownloadState>,
) {
    let mut abort_handlers_guard =
        download_state.abort_handlers.lock().unwrap();

    if let Some(handle) = abort_handlers_guard.take() {
        handle.abort();
    }
}

#[tauri::command]
pub fn llm_test(
    app_handle: tauri::AppHandle,
    window: tauri::Window,
    message: String,
) {
    use crate::services::llm::{Unspawned, LLM};

    let llm = LLM::<Unspawned>::default();
    let model = find_local_models(&app_handle).unwrap();
    let llm = llm.set_model(&model[2]);
    let session = llm.new_session(&message);
    // llm.start_inference(window, session);
}
