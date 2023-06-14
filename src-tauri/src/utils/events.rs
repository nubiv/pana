use serde::Serialize;

pub trait EventType<'a> {
    const NAME: &'a str;
    type Payload: EventPayload;
}
pub struct Noticification;
impl<'a> EventType<'a> for Noticification {
    const NAME: &'a str = "notification";
    type Payload = NoticificationPayload;
}
pub struct Error;
impl<'a> EventType<'a> for Error {
    const NAME: &'a str = "error";
    type Payload = ErrorPayload;
}
pub struct Response;
impl<'a> EventType<'a> for Response {
    const NAME: &'a str = "response";
    type Payload = ResponsePayload;
}
pub struct Model;
impl<'a> EventType<'a> for Model {
    const NAME: &'a str = "model";
    type Payload = ModelPayload;
}
pub struct Download;
impl<'a> EventType<'a> for Download {
    const NAME: &'a str = "download";
    type Payload = DownloadPayload;
}

pub trait EventPayload {}

#[derive(Clone, serde::Serialize)]
pub struct NoticificationPayload {
    pub message: String,
}
impl EventPayload for NoticificationPayload {}
#[derive(Clone, serde::Serialize)]
pub struct ErrorPayload {
    pub message: String,
}
impl EventPayload for ErrorPayload {}
#[derive(Clone, serde::Serialize)]
pub struct ResponsePayload {
    pub message: String,
}
impl EventPayload for ResponsePayload {}
#[derive(Clone, serde::Serialize)]
pub struct ModelPayload {
    pub running_model: String,
    pub local_models: Vec<std::path::PathBuf>,
}
impl EventPayload for ModelPayload {}
#[derive(Clone, serde::Serialize)]
pub struct DownloadPayload {
    pub progress: String,
}
impl EventPayload for DownloadPayload {}

pub struct AppEvent<'a, T>
where
    T: 'a + EventType<'a>,
    T::Payload: Serialize + Clone,
{
    name: &'a str,
    payload: T::Payload,
}

impl<'a, T> AppEvent<'a, T>
where
    T: 'a + EventType<'a>,
    T::Payload: Serialize + Clone,
{
    pub fn new(payload: T::Payload) -> Self {
        Self {
            name: T::NAME,
            payload,
        }
    }

    pub fn emit(self, window: &tauri::Window) {
        window
            .emit(self.name, self.payload)
            .map_err(|e| println!("Error {:?}", e))
            .unwrap();
    }
}

#[macro_export]
macro_rules! app_event {
    ($window:expr, $event:ty, $payload:expr) => {
        AppEvent::<$event>::new($payload).emit($window);
    };
}
