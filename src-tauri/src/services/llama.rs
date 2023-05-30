use super::{error::LLMError, model_path::find_local_model};
use llm_chain::{parameters, prompt, traits::Executor};
use llm_chain_llama::{Executor as LlamaExecutor, PerExecutor, PerInvocation};
use llm_chain_openai::chatgpt::Executor as ChatGPTExecutor;
use std::{error::Error, io::Write};
use tauri::async_runtime::Sender;

pub struct LLMCtx<T: Executor> {
    pub exec: T,
    tx: Sender<String>,
    processing: bool,
}

impl LLMCtx<LlamaExecutor> {
    // pub fn with_cb(&mut self, tx: String) -> Result<Self, LLMError> {
    //     let executor = self.exec;

    //     fn run_with_cb(exec: dyn Executor, tx: String) {}

    //     Ok(Self {
    //         exec: executor,
    //         processing: false,
    //     })
    // }

    pub fn spawn_llama(tx: Sender<String>) -> Result<Self, LLMError> {
        let model_path = find_local_model().unwrap();
        let exec_options = PerExecutor::new().with_model_path(&model_path);
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
            tx,
            processing: false,
        })
    }

    pub async fn feed_input(
        &mut self,
        input: &str,
    ) -> Result<String, LLMError> {
        if !self.processing {
            self.processing = true;

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

// impl LLMCtx<ChatGPTExecutor> {
//     pub fn spawn_chatgpt() -> Result<Self, Box<dyn Error>> {
//         let executor = ChatGPTExecutor::new()?;

//         Ok(Self {
//             exec: executor,
//             processing: false,
//         })
//     }
// }
