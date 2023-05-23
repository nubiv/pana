// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;

use std::error::Error;

use services::{error::LLMError, llama::LLMCtx};

// fn main() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    run_llama().await?;

    Ok(())
}

async fn run_llama() -> Result<String, LLMError> {
    let mut llm = LLMCtx::spawn_llama(
        "/Users/horus/dev/lobot/src-tauri/target/debug/llm/ggml-alpaca-7b-q4.bin",
    )?;
    let res = llm.feed_input().await?;

    println!("Llama: {}", res);
    Ok(res.to_string())
}
