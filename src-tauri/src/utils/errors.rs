#[derive(thiserror::Error, Debug)]
pub enum LLMError {
    #[error("Initing LLM failed.")]
    InitingError(#[from] InitingError),
    #[error("Inference failed.")]
    InferenceError(#[from] InferenceError),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum InitingError {
    #[error("Unsupported architecture.")]
    UnsupportedArch(#[from] llm::UnsupportedModelArchitecture),
    #[error("Failed to load model.")]
    LoadError(#[from] llm::LoadError),
}

#[derive(thiserror::Error, Debug)]
pub enum InferenceError {
    #[error("Feeding prompt failed.")]
    FeedingPromptError(#[from] llm::InferenceError),
    // #[error("LLM is processing, wait a sec.")]
    // IsProcessing,
}

#[derive(thiserror::Error, Debug)]
pub enum IOError {
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
    IOError(#[from] IOError),
    #[error("LLMError {0}")]
    LLMError(#[from] LLMError),
    #[error(transparent)]
    Custom(#[from] anyhow::Error),
}
