use super::error::LLMError;
use llm_chain::{parameters, prompt, traits::Executor};
use llm_chain_llama::{Executor as LlamaExecutor, PerExecutor, PerInvocation};
use llm_chain_openai::chatgpt::Executor as ChatGPTExecutor;
use std::error::Error;

pub struct LLMCtx<T: Executor> {
    exec: T,
    processing: bool,
}

impl LLMCtx<LlamaExecutor> {
    pub fn spawn_llama(model_file_name: &str) -> Result<Self, LLMError> {
        // TODO: custom model path
        // let model_path: &str;
        // let exe_path = std::env::current_exe()?;
        // if let Some(debug_path) =
        //     exe_path.parent()
        // {
        //     // println!(
        //     //     "Current path: {:?}",
        //     //     debug_path.to_str()
        //     // );
        //     if let Some(mut model_path) =
        //         debug_path.to_str()
        //     {
        //         let tmp = format!(
        //             "{}{}",
        //             model_path, model_file_name
        //         );

        //         model_path = &tmp;
        //     }
        // }

        let exec_options = PerExecutor::new().with_model_path(model_file_name);
        let mut inv_options = PerInvocation::new();
        inv_options.n_threads = Some(1);

        let executor = LlamaExecutor::new_with_options(Some(exec_options), Some(inv_options))
            .map_err(|_| LLMError::InitingLLMFailed)?;

        Ok(Self {
            exec: executor,
            processing: false,
        })
    }

    pub async fn feed_input(&mut self) -> Result<String, LLMError> {
        if !self.processing {
            self.processing = true;

            let res =
                prompt!("Write a hypothetical weather report for {{season}} in {{location}}.")
                    .run(
                        &parameters!("season" => "winter", "location" => "Canada"),
                        &self.exec,
                    )
                    .await
                    .map_err(|_| LLMError::FeedingInputFailed)?;

            let res_string = res.to_string();

            self.processing = false;
            return Ok(res_string);
        }

        Err(LLMError::IsProcessing)
    }
}

impl LLMCtx<ChatGPTExecutor> {
    pub fn spawn_chatgpt() -> Result<Self, Box<dyn Error>> {
        let executor = ChatGPTExecutor::new()?;

        Ok(Self {
            exec: executor,
            processing: false,
        })
    }
}
