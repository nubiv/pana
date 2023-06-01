use std::sync::{Arc, Mutex, MutexGuard};

use llm_chain_llama::Executor as LlamaExecutor;
use tokio::sync::mpsc;

use crate::services::error::LLMError;
use crate::services::llama::LLMCtx;
use crate::Channel;

#[tauri::command]
pub fn run_llama(window: tauri::Window, state: tauri::State<Channel>) {
    let (tx, mut rx) = mpsc::channel(32);
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
                let mut llm = LLMCtx::spawn_llama().unwrap();

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
                            match feed_input(&mut llm, input).await {
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

async fn feed_input(
    llm: &mut LLMCtx<LlamaExecutor>,
    input: String,
) -> Result<String, LLMError> {
    let res = llm.feed_input(&input).await;

    match res {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
