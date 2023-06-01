// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::{async_runtime::Sender, generate_handler};

mod commands;
use commands::{run_llama, send_message};
mod services;

#[derive(Default)]
pub struct Channel {
    pub tx: Mutex<Option<Sender<String>>>,
}

fn main() {
    tauri::Builder::default()
        .manage(Channel::default())
        .invoke_handler(generate_handler![run_llama, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
