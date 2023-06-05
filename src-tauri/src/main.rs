// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use llm_chain::traits::Executor;
use serde::Serialize;
use services::llama::LLM;
use std::sync::Mutex;
use tauri::{async_runtime::Sender, generate_handler, App, Manager};

mod commands;
use commands::{download_model, run_llama, send_message, update_llm_models};
mod services;
use services::models::{create_llm_dir, find_local_models, is_llm_dir_existed};

#[derive(Default)]
pub struct Channel {
    pub tx: Mutex<Option<Sender<String>>>,
}

#[derive(Debug)]
pub struct Online;

#[derive(Debug)]
pub struct Offline;

#[derive(Debug)]
pub struct AppState<AppMode = Offline> {
    app_mode: std::marker::PhantomData<AppMode>,
    local_models: Mutex<Vec<String>>,
    model_in_use: Mutex<Option<String>>,
}

impl AppState {}

impl AppState<Offline> {
    fn turn_online(self) -> AppState<Online> {
        AppState {
            app_mode: std::marker::PhantomData::<Online>,
            local_models: self.local_models,
            model_in_use: self.model_in_use,
        }
    }
}

impl AppState<Online> {
    fn turn_offline(self) -> AppState<Offline> {
        AppState {
            app_mode: std::marker::PhantomData::<Offline>,
            local_models: self.local_models,
            model_in_use: self.model_in_use,
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![
            run_llama,
            send_message,
            update_llm_models,
            download_model
        ])
        .setup(|app| {
            let app_handle = app.app_handle();

            // tauri::api::dialog::message(
            //     Some(&main_window),
            //     "Hello",
            //     "Welcome back!",
            // );

            match is_llm_dir_existed(&app_handle) {
                true => {
                    let models = find_local_models(&app_handle).unwrap();

                    app.manage(AppState {
                        app_mode: std::marker::PhantomData::<Offline>,
                        local_models: Mutex::from(models),
                        model_in_use: Mutex::from(None),
                    });
                }
                false => match create_llm_dir(&app_handle) {
                    Ok(_) => {
                        let models = find_local_models(&app_handle).unwrap();

                        app.manage(AppState {
                            app_mode: std::marker::PhantomData::<Offline>,
                            local_models: Mutex::from(models),
                            model_in_use: Mutex::from(None),
                        });
                    }
                    Err(e) => println!("{}", e),
                },
            }

            app.manage(Channel::default());

            Ok(())
        })
        // .on_page_load(|window, payload| {
        //     let app_handler = window.app_handle();
        //     let app_state = app_handler.state::<AppState>();
        //     let local_models = &app_state.local_models;
        //     let models_guard = local_models.try_lock().unwrap();
        //     let models = &*models_guard;
        //     println!("payload: {:?}", payload);
        //     println!("models: {:?}", models);
        //     if window.emit("system_message", "test").is_err() {
        //         println!("something went wrrong while sending the models");
        //     };
        //     if window.emit("system_message", models).is_err() {
        //         println!("something went wrrong while sending the models");
        //     };
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
