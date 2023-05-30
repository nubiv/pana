// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use llm_chain::traits::Executor;
use llm_chain_llama::Executor as LlamaExecutor;
use std::{
    error::Error,
    process::Command,
    sync::{Arc, Mutex, MutexGuard},
};
use tauri::{async_runtime::Sender, generate_handler, Manager, State};
use tokio::{
    runtime::Handle,
    sync::{
        mpsc::{self, error::SendError},
        oneshot,
    },
};

mod commands;
mod services;

use services::{error::LLMError, llama::LLMCtx};

// #[derive(Default)]
// struct LLMConnection {
//     llm: Mutex<Option<Arc<Mutex<LLMCtx<LlamaExecutor>>>>>,
// }

// #[tauri::command]
// async fn connect_llm(
//     connection: State<'_, LLMConnection>,
// ) -> Result<String, LLMError> {
//     match &*connection.llm.lock().unwrap() {
//         Some(llm) => {
//             // let res = match llm.try_lock() {
//             //     Ok(mut llm_guard) => llm_guard.feed_input().await,
//             //     Err(_) => {
//             //         Err(LLMError::Custom("LLM is processing.".to_string()))
//             //     }
//             // };

//             // match res {
//             //     Ok(s) => Ok(s),
//             //     Err(e) => Err(e),
//             // }
//             todo!()
//         }
//         None => {
//             *connection.llm.lock().unwrap() =
//                 Some(Arc::new(Mutex::new(LLMCtx::spawn_llama(
//                     "/Users/horus/dev/lobot/llm/ggml-alpaca-7b-q4.bin",
//                 )?)));

//             Ok("Connection established.".to_string())
//         }
//     }
// }

// fn main() {
//     tauri::Builder::default()
//         .manage(LLMConnection {
//             llm: Default::default(),
//         })
//         .invoke_handler(tauri::generate_handler!(connect_llm))
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
// struct AppState {
//     llama_in_use: bool,
//     llm_ctx: Mutex<Option<LLMCtx<dyn Executor>>>,
// }
#[derive(Default)]
struct AppState {
    pub tx: Mutex<Option<Sender<String>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let llm = Arc::new(Mutex::new(LLMCtx::spawn_llama(
    //     "/Users/horus/dev/lobot/llm/ggml-alpaca-7b-q4.bin",
    // )?));

    // let (tx, rx) = oneshot::channel();

    // tokio::spawn(async move {
    //     // let res = run_llama(Arc::clone(&llm));
    //     let res = tokio::task::block_in_place(|| {
    //         futures::executor::block_on(run_llama(llm))
    //     });

    //     if let Err(_) = tx.send(res) {
    //         println!("the receiver droppped.")
    //     }
    // });

    // match rx.await {
    //     Ok(res) => match res {
    //         Ok(v) => println!("Llama: {}", v),
    //         Err(e) => println!("Error: {}", e),
    //     },
    //     Err(_) => println!("the sender dropped"),
    // }

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(generate_handler![run_llama, send_message])
        // .setup(|app| {
        //     let app_handle = app.handle();
        //     let (tx, mut rx) = mpsc::channel(2);
        //     tauri::async_runtime::spawn(async move {
        //         let res = tokio::task::block_in_place(|| {
        //             let llm = Arc::new(Mutex::new(LLMCtx::spawn_llama()?));
        //             // let guard = match llm.try_lock() {
        //             //     Ok(mut llm_guard) => llm_guard,
        //             //     Err(_) => (),
        //             // };
        //             // match llm.lock() {
        //             //     Ok(executor) => {
        //             //         executor.exec.with_callback(move |output| {
        //             //             tx.blocking_send("Hello world")
        //             //         })
        //             //     }
        //             //     _ => LLMError::Custom("Nothing happens"),
        //             // };
        //             // if let Ok(executor) = llm.lock() {
        //             //     executor
        //             //         .exec
        //             //         .with_callback(|output| println!("{}", output))
        //             // }
        //             futures::executor::block_on(run_llama_test(
        //                 llm.try_lock().unwrap(),
        //             ))
        //         });
        //         if let Err(_) = tx.send(res).await {
        //             println!("the receiver droppped.")
        //         }
        //     });
        //     tauri::async_runtime::spawn(async move {
        //         match rx.recv().await {
        //             Some(res) => match res {
        //                 Ok(v) => {}
        //                 Err(e) => println!("Error: {}", e),
        //             },
        //             None => println!("the sender dropped"),
        //         }
        //     });
        //     // tauri::async_runtime::spawn(async move {
        //     //     // read events such as stdout
        //     // });
        //     Ok(())
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // run_llama().await?;
    Ok(())
}

// async fn run_llama_test(
//     mut llm: MutexGuard<'_, LLMCtx<LlamaExecutor>>,
// ) -> Result<String, LLMError> {
//     // let mut llm = LLMCtx::spawn_llama("/Users/horus/dev/lobot/llm/ggml-alpaca-7b-q4.bin")?;
//     // let res = llm.feed_input().await?;

//     // println!("Llama: {}", res);
//     // Ok(res.to_string())
//     // let res = match llm.try_lock() {
//     //     Ok(mut llm_guard) => llm_guard.feed_input().await,
//     //     Err(_) => Err(LLMError::Custom("LLM is processing.".to_string())),
//     // };
//     let res = llm.feed_input().await;

//     match res {
//         Ok(s) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

async fn llama_loop(
    llm: Arc<Mutex<LLMCtx<LlamaExecutor>>>,
) -> Result<String, LLMError> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(_) = tx.send(3) {
            println!("the receiver dropped");
        }
    });

    match rx.await {
        Ok(v) => println!("got = {:?}", v),
        Err(_) => println!("the sender dropped"),
    }

    Ok("Helllo".to_string())
}

// #[tauri::command]
// async fn send_message(message: &str) -> Result<String, String> {
//     let llm = Arc::new(Mutex::new(LLMCtx::spawn_llama().unwrap()));
//     tauri::async_runtime::spawn(async move {
//         let res = tokio::task::block_in_place(|| {
//             // match llm.lock() {
//             //     Ok(executor) => {
//             //         executor.exec.with_callback(move |output| {
//             //             tx.blocking_send("Hello world")
//             //         })
//             //     }
//             //     _ => LLMError::Custom("Nothing happens"),
//             // };
//             // if let Ok(executor) = llm.lock() {
//             //     executor
//             //         .exec
//             //         .with_callback(|output| println!("{}", output))
//             // }
//             futures::executor::block_on(run_llama(llm))
//         });
//     });

//     // let res = match llm.try_lock() {
//     //     Ok(mut llm_guard) => llm_guard.feed_input().await,
//     //     Err(_) => Err(LLMError::Custom("LLM is processing.".to_string())),
//     // };

//     match res {
//         Ok(s) => Ok(s),
//         Err(e) => Err(e.to_string()),
//     }
// }

#[tauri::command]
fn run_llama(window: tauri::Window, state: tauri::State<AppState>) {
    let (tx, mut rx) = mpsc::channel(32);
    let tx_guard = &mut *state.tx.lock().unwrap();
    *tx_guard = Some(tx.clone());

    tauri::async_runtime::spawn(async move {
        tokio::task::block_in_place(|| {
            let llm =
                Arc::new(Mutex::new(LLMCtx::spawn_llama(tx.clone()).unwrap()));

            Handle::current().block_on(async move {
                loop {
                    match rx.recv().await {
                        Some(input) => {
                            // futures::executor::block_on(feed_input(
                            //     llm.try_lock().unwrap(),
                            //     input,
                            // ));

                            // if let Err(e) =
                            //     feed_input(llm.try_lock().unwrap(), input).await
                            // {
                            //     println!("run llama block {}", e)
                            // }

                            match feed_input(llm.try_lock().unwrap(), input)
                                .await
                            {
                                Ok(res) => {
                                    window
                                        .emit(
                                            "incoming_response",
                                            Payload {
                                                message: res.to_string(),
                                            },
                                        )
                                        .unwrap();
                                }
                                Err(e) => println!("run llama block {}", e),
                            }
                        }
                        None => println!("the sender dropped"),
                    }
                }
            })
        });
    });
}

#[tauri::command]
fn send_message(
    state: tauri::State<AppState>,
    message: String,
) -> Result<(), String> {
    let tx_guard = &*state.tx.lock().unwrap();

    if let Some(tx) = tx_guard {
        // tx.blocking_send(message).map_err(|e| (e));

        let tmp_tx = tx.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(_) = tmp_tx.send(message).await {
                println!("send message block..")
            }
        });
    } else {
        // println!("something went wrong...")
        ()
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

// fn send_response(window: &tauri::Window, payload: Payload) {
//     std::thread::spawn(move || loop {
//         window.emit("incoming_response", &payload).unwrap();
//     });
// }
