use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShindenError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Filesystem error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Corrupted cookie file: {0}")]
    CookieParse(String),

    #[error("Header configuration failed: {0}")]
    HeaderConfig(String),

    #[error("Thread safety violation (Mutex Poisoned)")]
    LockPoisoned,

    #[error("System config error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, ShindenError>;