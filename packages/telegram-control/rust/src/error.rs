use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Telegram API error: {0}")]
    Telegram(#[from] teloxide::RequestError),
    
    #[error("Authentication failed: {0}")]
    Auth(String),
    
    #[error("User not authorized: {0}")]
    Unauthorized(i64),
    
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
