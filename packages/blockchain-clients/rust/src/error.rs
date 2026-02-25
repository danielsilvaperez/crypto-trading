use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("RPC error: {0}")]
    Rpc(String),
    
    #[error("Authentication failed: {0}")]
    Authentication(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Insufficient balance: required {required}, have {available}")]
    InsufficientBalance { required: String, available: String },
    
    #[error("Market not found: {0}")]
    MarketNotFound(String),
    
    #[error("Order rejected: {0}")]
    OrderRejected(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl Error {
    /// Create a new error with a message
    pub fn msg<S: Into<String>>(msg: S) -> Self {
        Error::Other(msg.into())
    }
    
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(self,
            Error::Network(_) |
            Error::Rpc(_) |
            Error::RateLimit
        )
    }
}
