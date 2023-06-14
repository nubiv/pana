use std::sync::{Arc, Mutex};

pub trait LLMState {}

#[derive(Default)]
pub struct Unspawned;
impl LLMState for Unspawned {}
pub struct Idle;
impl LLMState for Idle {}
pub struct Processing;
impl LLMState for Processing {}
#[derive(Default)]
pub struct LLM<S: LLMState> {
    model: Arc<Mutex<Option<Box<dyn llm::Model>>>>,
    message_handle: Arc<Mutex<Option<tauri::async_runtime::Sender<u8>>>>,
    abort_handle: Arc<Mutex<Option<tauri::async_runtime::JoinHandle<()>>>>,
    state: std::marker::PhantomData<S>,
}

impl LLM<Unspawned> {
    pub fn set_model(self, model_path: &std::path::Path) -> Self {
        let architecture = llm::ModelArchitecture::Llama;
        let params = llm::ModelParameters {
            ..Default::default()
        };
        let model = llm::load_dynamic(
            architecture,
            model_path,
            params,
            llm::load_progress_callback_stdout,
        )
        .unwrap_or_else(|e| panic!("Failed to load model: {}", e));

        Self {
            model: Arc::new(Mutex::new(Some(model))),
            message_handle: Arc::new(Mutex::new(None)),
            abort_handle: Arc::new(Mutex::new(None)),
            state: std::marker::PhantomData::<Unspawned>,
        }
    }

    pub fn new_session(&self, message: &str) -> llm::InferenceSession {
        let message_handle_clone = Arc::clone(&self.message_handle);
        let mut message_handle_guard = message_handle_clone.lock().unwrap();

        let character_name = "### Assistant";
        let user_name = "### Human";
        let persona = "A chat between a human and an assistant.";
        let history = format!(
            "{character_name}: Hello - How may I help you today?\n\
                 {user_name}: What is the capital of France?\n\
                 {character_name}:  Paris is the capital of France."
        );

        let model_ptr = Arc::clone(&self.model);
        let model_guard = model_ptr.lock().unwrap();
        let model = match &*model_guard {
            Some(model) => model,
            None => panic!("model is not loaded"),
        };

        let mut session = model.start_session(Default::default());

        session
            .feed_prompt(
                model.as_ref(),
                &llm::InferenceParameters::default(),
                format!("{}\n{}", persona, history).as_str(),
                &mut Default::default(),
                |r| match String::from_utf8(r.to_vec()) {
                    Ok(r) => {
                        println!("{}", r);
                        Ok(())
                    }
                    Err(e) => Err(e),
                },
            )
            .expect("Failed to ingest initial prompt.");

        // let model_clone = Arc::clone(&self.model);
        // let model_guard = model_clone.lock().unwrap();
        // let model = match &*model_guard {
        //     Some(model) => model,
        //     None => panic!("model is not loaded"),
        // };

        // let mut session = model.start_session(Default::default());

        // session
        //     .feed_prompt(
        //         model.as_ref(),
        //         &llm::InferenceParameters::default(),
        //         format!("{}\n{}", persona, history).as_str(),
        //         &mut Default::default(),
        //         |r| match String::from_utf8(r.to_vec()) {
        //             Ok(r) => {
        //                 println!("{}", r);
        //                 Ok(())
        //             }
        //             Err(e) => Err(e),
        //         },
        //     )
        //     .expect("Failed to ingest initial prompt.");

        session
    }

    pub fn start_inference(
        self,
        window: tauri::Window,
        mut session: llm::InferenceSession,
    ) -> tauri::async_runtime::JoinHandle<()> {
        let (tx, response_handle) = inference_callback(window);

        let handle = tauri::async_runtime::spawn_blocking(move || {
            let model_clone = Arc::clone(&self.model);
            let model_guard = model_clone.lock().unwrap();
            let model = match &*model_guard {
                Some(model) => model,
                None => panic!("model is not loaded"),
            };

            // let message_handle_clone = Arc::clone(&self.message_handle);
            // let message_handle_guard = message_handle_clone.lock().unwrap();
            // let message_handle = match &*message_handle_guard {
            //     Some(message_handle) => message_handle,
            //     None => panic!("message handle is not loaded"),
            // };

            let mut buf = vec![];

            while let Ok(token) = session.infer_next_token(
                model.as_ref(),
                &llm::InferenceParameters::default(),
                &mut Default::default(),
                &mut rand::thread_rng(),
            ) {
                buf.push(token);

                tx.send(token.to_vec()[1]);
            }

            response_handle.abort();
        });

        handle
    }
}

pub fn inference_callback(
    window: tauri::Window,
) -> (
    tokio::sync::mpsc::Sender<u8>,
    tauri::async_runtime::JoinHandle<()>,
) {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(10);

    let handle = tauri::async_runtime::spawn(async move {
        if let Some(token) = rx.recv().await {
            // feed prompt here
            // AppEvent::<Response>::new(ResponsePayload {
            //     message: String::from("hello"),
            // })
            // .emit(&window);
        }
    });

    (tx, handle)
}

impl LLM<Idle> {
    pub fn start_inference(self) -> LLM<Processing> {
        todo!()
    }

    pub fn stop_session() -> LLM<Unspawned> {
        todo!()
    }
}

impl LLM<Processing> {
    pub fn finish_inference(self) -> LLM<Idle> {
        todo!()
    }

    pub fn stop_session(self) -> LLM<Unspawned> {
        todo!()
    }
}

fn test() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<u8>(10);
}

pub struct Actor {
    receiver: tokio::sync::mpsc::Receiver<u8>,
}

pub struct ActorHandle {
    sender: tokio::sync::mpsc::Sender<u8>,
}

impl Actor {
    fn new(receiver: tokio::sync::mpsc::Receiver<u8>) -> Self {
        Self { receiver }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            println!("received: {}", msg);
        }
    }

    fn handle_message(&mut self) {}
}

impl ActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(8);
        let mut actor = Actor::new(receiver);
        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }
}
