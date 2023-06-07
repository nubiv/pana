use crate::utils::errors::LLMError;

use super::models::find_local_models;
use llm_chain::{parameters, prompt, traits::Executor};
use llm_chain_llama::{Executor as LlamaExecutor, PerExecutor, PerInvocation};
use std::io::Write;

pub struct LLM<T: Executor> {
    pub exec: T,
    processing: bool,
}

impl LLM<LlamaExecutor> {
    pub fn spawn_llama(
        app_handle: &tauri::AppHandle,
    ) -> Result<Self, LLMError> {
        let model_path = find_local_models(app_handle).unwrap();
        let exec_options = PerExecutor::new().with_model_path(&model_path[2]);
        let mut inv_options = PerInvocation::new();
        inv_options.n_threads = Some(1);

        let executor = LlamaExecutor::new_with_options(
            Some(exec_options),
            Some(inv_options),
        )
        .map_err(|_| LLMError::InitingLLMFailed)?;

        let executor = executor.with_callback(|output| {
            print!("{}", output);
            std::io::stdout().flush().unwrap();
        });

        Ok(Self {
            exec: executor,
            processing: false,
        })
    }

    pub async fn feed_input(
        &mut self,
        input: &str,
    ) -> Result<String, LLMError> {
        if !self.processing {
            self.processing = true;

            let prompt = "A dialog, where User interacts with AI. AI is helpful, kind, obedient, honest, and knows its own limits.
            User: Hello, AI.
            AI: Hello! How can I assist you today?";

            let res = prompt!(input)
                .run(&parameters!(), &self.exec)
                .await
                .map_err(|_| LLMError::FeedingInputFailed)?;

            let res_string = res.to_string();

            self.processing = false;
            return Ok(res_string);
        }

        Err(LLMError::IsProcessing)
    }
}
