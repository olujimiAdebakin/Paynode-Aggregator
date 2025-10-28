
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypesError {
    #[error("Invalid address format: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Invalid tier: {0}")]
    InvalidTier(String),
    
    #[error("Invalid status: {0}")]
    InvalidStatus(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, TypesError>;