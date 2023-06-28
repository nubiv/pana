use anyhow::anyhow;
use std::{
    str::FromStr,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use crate::utils::errors::{InitingError, LLMError};
use crate::utils::events::*;
use crate::{app_event, utils::errors::InferenceError};

pub fn set_model(
    llm_state: &tauri::State<crate::LLMState>,
    app_handle: &tauri::AppHandle,
    model_info: &crate::utils::models::ModelInfo,
) -> Result<(), LLMError> {
    let model_config_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .expect("failed to resolve resource");

    let bin_path = model_config_path.join("bin");
    let model_path = bin_path.join(&model_info.filename);

    let architecture = llm::ModelArchitecture::from_str(
        model_info.architecture.as_str(),
    )
    .map_err(InitingError::UnsupportedArch)?;
    let params = llm::ModelParameters {
        ..Default::default()
    };
    let model = llm::load_dynamic(
        architecture,
        &model_path,
        llm::VocabularySource::Model,
        params,
        llm::load_progress_callback_stdout,
    )
    .map_err(InitingError::LoadError)?;

    let mut model_guard =
        llm_state.model.lock().map_err(|_| {
            LLMError::Custom(anyhow!(
                "Failed to setup model."
            ))
        })?;
    *model_guard = Some(model);

    Ok(())
}

pub fn new_session(
    model: &dyn llm::Model,
    message: &str,
    tree: &sled::Tree,
) -> Result<llm::InferenceSession, LLMError> {
    let character_name = "### Pana";
    let user_name = "### Human";
    let persona =
        "A chat between a human and an assistant called Pana.";

    let latest_adjacency_pairs =
        crate::db::get_latest_adjacency_pairs(tree)
            .unwrap();
    let history = latest_adjacency_pairs
        .iter()
        .map(|(k, v)| {
            format!(
                "{}: {}\n\
        ",
                k, v
            )
        })
        .collect::<String>();

    // let prompt = format!(
    //     "{persona}\n\
    //      {user_name}: {message}\n\
    //      {character_name}:",
    // );

    let prompt = format!(
        "{persona}\n\
         {history}\
         {user_name}: {message}\n\
         {character_name}: "
    );
    // println!("prompt: {:?}", prompt);

    let mut session =
        model.start_session(Default::default());

    session
        .feed_prompt::<std::convert::Infallible, llm::Prompt>(
            model,
            &llm::InferenceParameters::default(),
            llm::Prompt::Text(&prompt),
            &mut Default::default(),
            |_| Ok(llm::InferenceFeedback::Continue),
        )
        .map_err(InferenceError::FeedingPromptError)?;

    Ok(session)
}

pub fn infer(
    window: Arc<tauri::Window>,
    message: String,
    model: Arc<Mutex<Option<Box<dyn llm::Model>>>>,
    abort_handle: Arc<AtomicBool>,
    tree: Arc<sled::Tree>,
) {
    tauri::async_runtime::spawn_blocking(move || {
        let user_message_time = chrono::Local::now();
        let model_guard = match model.lock() {
            Ok(model_guard) => model_guard,
            Err(_) => {
                app_event!(
                    &window,
                    Error,
                    ErrorPayload {
                        message: String::from(
                            "Failed to read model."
                        )
                    }
                );

                return;
            }
        };
        let model = match &*model_guard {
            Some(model) => model,
            None => {
                app_event!(
                    &window,
                    Error,
                    ErrorPayload {
                        message: String::from(
                            "Model not loaded."
                        )
                    }
                );

                return;
            }
        };

        let parameters =
            llm::InferenceParameters::default();
        let mut rng = rand::thread_rng();

        let mut stats = llm::InferenceStats::default();
        let start_at = std::time::SystemTime::now();

        app_event!(
            &window,
            Response,
            ResponsePayload {
                is_streaming: false,
                is_feeding_prompt: true,
                token: String::from("")
            }
        );

        let mut session =
            match crate::services::llm::new_session(
                model.as_ref(),
                &message,
                &tree,
            ) {
                Ok(session) => session,
                Err(e) => {
                    app_event!(
                        &window,
                        Error,
                        ErrorPayload {
                            message: format!(
                                "Failed to process with inference: {}",
                                e
                            )
                        }
                    );

                    app_event!(
                        &window,
                        Response,
                        ResponsePayload {
                            is_streaming: false,
                            is_feeding_prompt: false,
                            token: String::from("")
                        }
                    );

                    return;
                }
            };

        app_event!(
            &window,
            Response,
            ResponsePayload {
                is_streaming: true,
                is_feeding_prompt: false,
                token: String::from("")
            }
        );

        stats.feed_prompt_duration =
            start_at.elapsed().unwrap();
        stats.prompt_tokens = session.n_past;

        let max_token_count = usize::MAX;
        let mut tokens_processed = 0;
        let mut response = String::from("");
        let mut utf8_buf = llm::TokenUtf8Buffer::new();
        while tokens_processed < max_token_count {
            if abort_handle
                .load(std::sync::atomic::Ordering::Relaxed)
            {
                break;
            }

            let token = match session.infer_next_token(
                model.as_ref(),
                &parameters,
                &mut Default::default(),
                &mut rng,
            ) {
                Ok(token) => token,
                Err(llm::InferenceError::EndOfText) => {
                    break
                }
                Err(e) => {
                    println!(
                        "Failed to infer next token: {}",
                        e
                    );
                    break;
                }
            };

            if let Some(token) = utf8_buf.push(&token) {
                response.push_str(&token);
                app_event!(
                    &window,
                    Response,
                    ResponsePayload {
                        is_streaming: true,
                        is_feeding_prompt: false,
                        token
                    }
                );
            }

            tokens_processed += 1;
        }
        stats.predict_duration =
            start_at.elapsed().unwrap();
        stats.predict_tokens = session.n_past;

        let window_clone = window.clone();
        tauri::async_runtime::spawn(async move {
            // create id based on local time, format - %H:%M:%S
            // so messages will be automatically ordered by time

            // this atomic u64 id only increments and persists
            // only use it for ordering the messages in the timeline
            // let batch_id = db.generate_id().unwrap();
            // let user_format =
            //     format!("{}-0-%Y-%m-%d %H:%M:%S", batch_id);
            // let pana_format =
            //     format!("{}-1-%Y-%m-%d %H:%M:%S", batch_id);

            let user_formatted_time = user_message_time
                .format("%H:%M:%S-0")
                .to_string();
            let pana_message_time = chrono::Local::now();
            let pana_formatted_time = pana_message_time
                .format("%H:%M:%S-1")
                .to_string();
            let user_message = &message;
            let pana_message = &response;

            if let Err(e) = crate::db::insert_adjacency_pair(
                &user_formatted_time,
                user_message,
                &pana_formatted_time,
                pana_message,
                &tree,
            ) {
                app_event!(
                    &window_clone,
                    Error,
                    ErrorPayload {
                        message: format!(
                            "Failed to save chat messages: {}",
                            e
                        )
                    }
                );
            }

            // if let Some(first) = tree.first().unwrap() {
            //     crate::db::print_kv(&first.0, &first.1);
            // };

            // println!("Inference Stats: {:?}", stats);
        });

        abort_handle.store(
            false,
            std::sync::atomic::Ordering::Relaxed,
        );

        app_event!(
            &window,
            Response,
            ResponsePayload {
                is_streaming: false,
                is_feeding_prompt: false,
                token: String::from("")
            }
        );
    });
}
