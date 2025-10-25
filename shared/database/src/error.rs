
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;