// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use llm_chain_llama::Executor as LlamaExecutor;
use std::{
    error::Error,
    sync::{Arc, Mutex, MutexGuard},
};
use tauri::{async_runtime::Sender, generate_handler};
use tokio::{runtime::Handle, sync::mpsc};

mod commands;
mod services;

use services::{error::LLMError, llama::LLMCtx};

#[derive(Default)]
struct Channel {
    pub tx: Mutex<Option<Sender<String>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .manage(Channel::default())
        .invoke_handler(generate_handler![run_llama, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // run_llama().await?;
    Ok(())
}

#[tauri::command]
fn run_llama(window: tauri::Window, state: tauri::State<Channel>) {
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
            *tx_guard = Some(tx.clone());

            tauri::async_runtime::spawn(async move {
                tokio::task::block_in_place(|| {
                    let llm =
                        Arc::new(Mutex::new(LLMCtx::spawn_llama().unwrap()));

                    window
                        .emit(
                            "system_message",
                            Payload {
                                message: "Llama activated...".to_string(),
                            },
                        )
                        .unwrap();

                    Handle::current().block_on(async move {
                        loop {
                            match rx.recv().await {
                                Some(input) => {
                                    match feed_input(
                                        llm.try_lock().unwrap(),
                                        input,
                                    )
                                    .await
                                    {
                                        Ok(res) => {
                                            // TODO:
                                            // setup status code inside input
                                            // shut down gracefully if status code matches
                                            window
                                                .emit(
                                                    "incoming_response",
                                                    Payload {
                                                        message: res
                                                            .to_string(),
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
                    })
                });
            });

            // *tx_guard = None;
        }
    }
}

#[tauri::command]
fn send_message(
    window: tauri::Window,
    state: tauri::State<Channel>,
    message: String,
) -> Result<(), String> {
    let tx_guard = &*state.tx.lock().unwrap();

    if let Some(tx) = tx_guard {
        let tmp_tx = tx.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(_) = tmp_tx.send(message).await {
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
    mut llm: MutexGuard<'_, LLMCtx<LlamaExecutor>>,
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
