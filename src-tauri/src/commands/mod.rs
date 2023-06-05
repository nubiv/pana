use std::sync::{Arc, Mutex, MutexGuard};

use llm_chain_llama::Executor as LlamaExecutor;
use tokio::sync::mpsc;

use crate::services::downloader::download;
use crate::services::error::LLMError;
use crate::services::llama::LLM;
use crate::services::models::find_local_models;
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
            window
                .emit(
                    "system_message",
                    Payload {
                        message: "Llama has been running already..."
                            .to_string(),
                    },
                )
                .unwrap();
        }
        None => {
            *tx_guard = Some(tx);

            tauri::async_runtime::spawn(async move {
                let mut llm = LLM::spawn_llama(&app_handle).unwrap();

                window
                    .emit(
                        "system_message",
                        Payload {
                            message: "Llama activated...".to_string(),
                        },
                    )
                    .unwrap();

                loop {
                    match rx.recv().await {
                        Some(input) => {
                            match llm.feed_input(&input).await {
                                Ok(res) => {
                                    // TODO:
                                    // setup status code inside input
                                    // shut down gracefully if status code matches
                                    window
                                        .emit(
                                            "incoming_response",
                                            Payload {
                                                message: res.to_string(),
                                            },
                                        )
                                        .unwrap();
                                }
                                Err(e) => {
                                    println!("run llama block {}", e)
                                }
                            }
                        }
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
) -> Result<(), String> {
    let tx_guard = &*state.tx.lock().unwrap();

    if let Some(tx) = tx_guard {
        let tmp_tx = tx.clone();
        tauri::async_runtime::spawn(async move {
            if tmp_tx.send(message).await.is_err() {
                println!("send message block..")
            }
        });
    } else {
        window
            .emit(
                "system_message",
                Payload {
                    message: "Wake Llama up first...".to_string(),
                },
            )
            .unwrap();
    };

    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub fn update_llm_models(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
) -> Result<Vec<String>, String> {
    let updated_models = find_local_models(&app_handle).unwrap();
    let models = &mut *state.local_models.try_lock().unwrap();
    *models = updated_models.clone();

    println!("models in state>>> {:?}", models);

    Ok(updated_models)
}

#[tauri::command]
pub fn download_model(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        download(&app_handle).await;
    });
}
