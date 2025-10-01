use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn plugin(msg: impl Into<String>) -> Self {
        Self::Plugin(msg.into())
    }
    
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
}
