// use crate::utils::errors::LLMError;
// use anyhow::anyhow;

// use crate::utils::models::find_local_models;
// use llm_chain::{
//     executor, options, options::ModelRef, parameters, prompt, traits::Executor,
// };
// use llm_chain_llama::Executor as LlamaExecutor;

// pub struct LLM<T: Executor> {
//     pub exec: T,
//     processing: bool,
// }

// impl LLM<LlamaExecutor> {
//     pub fn spawn_llama(
//         app_handle: &tauri::AppHandle,
//     ) -> Result<Self, LLMError> {
//         let model_path = find_local_models(app_handle).unwrap();
//         let model_path = model_path[2].to_str().unwrap();

//         let opts = options!(
//             Model: ModelRef::from_path(model_path),
//             ModelType: "llama",
//             MaxContextSize: 512_usize,
//             NThreads: 4_usize,
//             MaxTokens: 0_usize,
//             TopK: 40_i32,
//             TopP: 0.95,
//             TfsZ: 1.0,
//             TypicalP: 1.0,
//             Temperature: 0.8,
//             RepeatPenalty: 1.1,
//             RepeatPenaltyLastN: 64_usize,
//             FrequencyPenalty: 0.0,
//             PresencePenalty: 0.0,
//             Mirostat: 0_i32,
//             MirostatTau: 5.0,
//             MirostatEta: 0.1,
//             PenalizeNl: true,
//             StopSequence: vec!["\n".to_string()]
//         );

//         let executor = executor!(llama, opts).map_err(|e| {
//             LLMError::Custom(anyhow!("init exec failed: {}", e))
//         })?;

//         Ok(Self {
//             exec: executor,
//             processing: false,
//         })
//     }

//     pub async fn feed_input(
//         &mut self,
//         input: &str,
//     ) -> Result<llm_chain::output::OutputStream, LLMError> {
//         if !self.processing {
//             self.processing = true;

//             let prompt = "A dialog, where User interacts with AI. AI is helpful, kind, obedient, honest, and knows its own limits.
//             User: Hello, AI.
//             AI: Hello! How can I assist you today?";

//             // need to find a proper prompt
//             let res = prompt!(input)
//                 .run(&parameters!(), &self.exec)
//                 .await
//                 .map_err(|e| LLMError::FeedingInputFailed(e.to_string()))?;

//             let stream = res.as_stream().await.unwrap();

//             // let res_string = res.to_string();

//             self.processing = false;
//             return Ok(stream);
//         }

//         Err(LLMError::IsProcessing)
//     }
// }
