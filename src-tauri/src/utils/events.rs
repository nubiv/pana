use serde::Serialize;

pub trait EventType<'a> {
    const NAME: &'a str;
    type Payload: EventPayload + Serialize + Clone;

    // fn new(payload: Self::Payload) -> Self;

    // fn emit(
    //     window: &tauri::Window,
    //     payload: Self::Payload,
    // ) {
    //     window
    //         .emit(Self::NAME, payload)
    //         .expect("failed to emit event");
    // }
}

macro_rules! event_type {
    ($name:ident, $name_str:expr, $payload:ty) => {
        pub struct $name;

        impl<'a> EventType<'a> for $name {
            const NAME: &'a str = $name_str;
            type Payload = $payload;
        }
    };
}
event_type!(
    Noticification,
    "notification",
    NoticificationPayload
);
event_type!(Error, "error", ErrorPayload);
event_type!(Response, "response", ResponsePayload);
event_type!(Model, "model", ModelPayload);
event_type!(Download, "download", DownloadPayload);
event_type!(History, "history", HistoryPayload);

pub trait EventPayload {}

#[derive(Debug, Clone, serde::Serialize)]
pub struct NoticificationPayload {
    pub message: String,
}
impl EventPayload for NoticificationPayload {}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ErrorPayload {
    pub message: String,
}
impl EventPayload for ErrorPayload {}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResponsePayload {
    pub is_streaming: bool,
    pub is_feeding_prompt: bool,
    pub token: String,
}
impl EventPayload for ResponsePayload {}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModelPayload {
    pub name: String,
    pub size: u64,
    pub total_size: u64,
}
impl EventPayload for ModelPayload {}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DownloadPayload {
    pub size: u64,
}
impl EventPayload for DownloadPayload {}

#[derive(Debug, Clone, serde::Serialize)]
pub struct HistoryPayload {
    pub history: Vec<(u8, String)>,
}
impl EventPayload for HistoryPayload {}

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
            .expect("failed to emit event");
    }
}

/// Macro to emit an event to the frontend.
/// *Arguments*:
/// - `$window`: the reference to the target window.
/// - `$event`: the event type to emit. Available events are: `Notification`, `Error`, `Response`, `Model`, `Download`.
/// - `$payload`: the corresponding payload to send with the event.
#[macro_export]
macro_rules! app_event {
    ($window:expr, $event:ty, $payload:expr) => {
        AppEvent::<$event>::new($payload).emit($window);
    };
}
