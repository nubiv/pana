use std::sync::{Arc, Mutex, MutexGuard};

use tokio::sync::mpsc;

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
                let mut llm = LLM::spawn_llama(&app_handle).unwrap();

                let notification_payload = NoticificationPayload {
                    message: String::from("Lobot activated..."),
                };

                AppEvent::<Noticification>::new(notification_payload)
                    .emit(&window);

                loop {
                    match rx.recv().await {
                        Some(input) => match llm.feed_input(&input).await {
                            Ok(res) => {
                                let response_payload = ResponsePayload {
                                    message: res.to_string(),
                                };

                                AppEvent::<Response>::new(response_payload)
                                    .emit(&window);
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
pub fn download_model(app_handle: tauri::AppHandle, window: tauri::Window) {
    tauri::async_runtime::spawn(async move {
        download(&app_handle, &window).await;
    });
}
