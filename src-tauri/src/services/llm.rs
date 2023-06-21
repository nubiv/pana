use std::{
    str::FromStr,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use crate::app_event;
use crate::utils::events::*;

pub fn set_model(
    llm_state: &tauri::State<crate::LLMState>,
    app_handle: &tauri::AppHandle,
    model_info: &crate::utils::models::ModelInfo,
) {
    let model_config_path = app_handle
        .path_resolver()
        .resolve_resource("./models")
        .expect("failed to resolve resource");

    let bin_path = model_config_path.join("bin");
    let model_path = bin_path.join(&model_info.filename);

    let architecture =
        llm::ModelArchitecture::from_str(model_info.architecture.as_str())
            .expect("failed to parse model architecture");
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
    .unwrap_or_else(|e| panic!("Failed to load model: {}", e));

    let mut model_guard = llm_state.model.lock().unwrap();
    *model_guard = Some(model);
}

pub fn new_session(
    model: &dyn llm::Model,
    message: &str,
) -> llm::InferenceSession {
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant.";
    // let history = format!(
    //     "{character_name}: Hello - How may I help you today?\n\
    //              {user_name}: What is the capital of France?\n\
    //              {character_name}:  Paris is the capital of France."
    // );
    let new_message =
        format!("\n{}: {}\n{}:", user_name, message, character_name);

    let mut session = model.start_session(Default::default());

    session
        .feed_prompt::<std::convert::Infallible, llm::Prompt>(
            model,
            &llm::InferenceParameters::default(),
            llm::Prompt::Text(format!("{}\n{}", persona, new_message).as_str()),
            &mut Default::default(),
            |_| Ok(llm::InferenceFeedback::Continue),
        )
        .expect("Failed to ingest initial prompt.");

    session
}

pub fn infer(
    window: Arc<tauri::Window>,
    message: String,
    model: Arc<Mutex<Option<Box<dyn llm::Model>>>>,
    abort_handle: Arc<AtomicBool>,
) {
    tauri::async_runtime::spawn_blocking(move || {
        let model_guard = model.lock().unwrap();
        let model = match &*model_guard {
            Some(model) => model,
            None => panic!("model is not loaded"),
        };
        let parameters = llm::InferenceParameters::default();
        let mut rng = rand::thread_rng();

        let mut stats = llm::InferenceStats::default();
        let start_at = std::time::SystemTime::now();

        let mut session =
            crate::services::llm::new_session(model.as_ref(), &message);

        app_event!(
            &window,
            Response,
            ResponsePayload {
                is_streaming: true,
                token: String::from("")
            }
        );

        stats.feed_prompt_duration = start_at.elapsed().unwrap();
        stats.prompt_tokens = session.n_past;

        let max_token_count = usize::MAX;
        let mut tokens_processed = 0;
        let mut response = String::from("");
        let mut utf8_buf = llm::TokenUtf8Buffer::new();
        while tokens_processed < max_token_count {
            if abort_handle.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            let token = match session.infer_next_token(
                model.as_ref(),
                &parameters,
                &mut Default::default(),
                &mut rng,
            ) {
                Ok(token) => token,
                Err(llm::InferenceError::EndOfText) => break,
                Err(e) => panic!("Failed to infer next token: {}", e),
            };

            if let Some(token) = utf8_buf.push(&token) {
                response.push_str(&token);
                app_event!(
                    &window,
                    Response,
                    ResponsePayload {
                        is_streaming: true,
                        token
                    }
                );
            }

            tokens_processed += 1;
        }
        stats.predict_duration = start_at.elapsed().unwrap();
        stats.predict_tokens = session.n_past;

        tauri::async_runtime::spawn(async move {
            println!("start_at: {:?}", start_at);
            println!("Response: {}", response);
            println!("Stats: {:?}", stats);
        });

        abort_handle.store(false, std::sync::atomic::Ordering::Relaxed);

        app_event!(
            &window,
            Response,
            ResponsePayload {
                is_streaming: false,
                token: String::from("")
            }
        );
    });
}
