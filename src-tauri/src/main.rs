// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{generate_handler, Manager};

mod commands;
mod services;
mod utils;
use commands::{
    delete_model, download_model, load_model, open_model_folder,
    start_inference, stop_download, stop_inference, stop_model,
    update_llm_models,
};

#[derive(Default)]
pub struct LLMState {
    pub model: Arc<Mutex<Option<Box<dyn llm::Model>>>>,
    pub abort_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
}

#[derive(Default)]
pub struct DownloadState {
    abort_handlers: Mutex<Option<tauri::async_runtime::JoinHandle<()>>>,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            update_llm_models,
            load_model,
            stop_model,
            start_inference,
            download_model,
            stop_download,
            stop_inference,
            delete_model,
            open_model_folder
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            window_vibrancy::apply_vibrancy(&window, window_vibrancy::NSVisualEffectMaterial::HudWindow, None, None)
              .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
              .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            app.manage(LLMState::default());
            app.manage(DownloadState::default());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
