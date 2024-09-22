use super::KeySequence;

#[derive(Debug, thiserror::Error)]
pub enum KeymapError {
    #[error("Key not found")]
    KeyNotFound(KeySequence, String),
    #[error("Key({0}) for mode {1} is already bound")]
    KeyAlreadyBound(KeySequence, String),
    #[error("No such mode: {0}")]
    NoSuchMode(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type KeymapResult<T> = Result<T, KeymapError>;
