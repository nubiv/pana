// TODO: need to refactor error types
#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("LLM is processing, wait a sec.")]
    IsProcessing,
    #[error("Initing LLM failed.")]
    InitingLLMFailed,
    #[error("Feeding input to LLM failed: {0}")]
    FeedingInputFailed(String),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DownloadError {
    #[error("File error {0}")]
    FileError(#[from] std::io::Error),
    #[error("Network error {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("DownloadError {0}")]
    DownloadError(#[from] DownloadError),
    #[error("LLMError {0}")]
    LLMError(#[from] LLMError),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}
