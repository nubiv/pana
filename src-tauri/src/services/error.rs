use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LLMError {
    IsProcessing,
    InitingLLMFailed,
    FeedingInputFailed,
    Custom(String),
}

impl fmt::Display for LLMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LLMError::IsProcessing => {
                write!(f, "LLM is processing, wait a sec.")
            }
            LLMError::InitingLLMFailed => {
                write!(f, "Initing LLM failed.")
            }
            LLMError::FeedingInputFailed => {
                write!(f, "Feeding input to LLM failed.")
            }
            LLMError::Custom(s) => {
                write!(f, "{s}")
            }
        }
    }
}

impl Error for LLMError {}
