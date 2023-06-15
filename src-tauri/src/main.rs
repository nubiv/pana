// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{async_runtime::Sender, generate_handler, Manager};

mod commands;
use commands::{
    delete_model, download_model, load_model, load_model_v2, send_message,
    send_message_v2, stop_download, stop_inference, stop_model,
    update_llm_models,
};
mod services;
mod utils;

#[derive(Default)]
pub struct LLMState {
    pub model: Arc<Mutex<Option<Box<dyn llm::Model>>>>,
    pub abort_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
}

#[derive(Default)]
pub struct DownloadState {
    abort_handlers: Mutex<Option<tauri::async_runtime::JoinHandle<()>>>,
}

#[derive(Default)]
pub struct Channel {
    pub tx: Mutex<Option<Sender<String>>>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            update_llm_models,
            load_model,
            load_model_v2,
            stop_model,
            send_message,
            send_message_v2,
            download_model,
            stop_download,
            stop_inference,
            delete_model,
        ])
        .setup(|app| {
            let app_handle = app.app_handle();

            app.manage(LLMState::default());
            app.manage(DownloadState::default());
            app.manage(Channel::default());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
