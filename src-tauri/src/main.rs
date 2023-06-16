// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{async_runtime::Sender, generate_handler, Manager};

mod commands;
use commands::{
    delete_model, download_model, load_model, load_model_v2, open_model_folder,
    send_message_v2, start_inference, stop_download, stop_inference,
    stop_model, update_llm_models,
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
            start_inference,
            send_message_v2,
            download_model,
            stop_download,
            stop_inference,
            delete_model,
            open_model_folder
        ])
        .setup(|app| {
            let app_handle = app.app_handle();
            let window = app.get_window("main").unwrap();
            use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
              .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
              .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            app.manage(LLMState::default());
            app.manage(DownloadState::default());
            app.manage(Channel::default());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
