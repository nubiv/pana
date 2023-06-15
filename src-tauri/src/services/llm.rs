// This is the implementation for llm crate.
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

pub fn set_model(
    llm_state: tauri::State<crate::LLMState>,
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
    model: &Box<dyn llm::Model>,
    message: &str,
) -> llm::InferenceSession {
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant.";
    let history = format!(
        "{character_name}: Hello - How may I help you today?\n\
                 {user_name}: What is the capital of France?\n\
                 {character_name}:  Paris is the capital of France."
    );
    let new_message = format!("{}: {}", user_name, message);

    let mut session = model.start_session(Default::default());

    session
        .feed_prompt::<std::convert::Infallible, llm::Prompt>(
            model.as_ref(),
            &llm::InferenceParameters::default(),
            llm::Prompt::Text(
                format!("{}\n{}\n{}", persona, history, new_message).as_str(),
            ),
            &mut Default::default(),
            |_| Ok(llm::InferenceFeedback::Continue),
        )
        .expect("Failed to ingest initial prompt.");

    session
}
