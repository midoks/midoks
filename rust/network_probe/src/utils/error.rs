use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("DNS resolution failed: {0}")]
    Dns(String),
    
    #[error("Ping failed: {0}")]
    Ping(String),
    
    #[error("TCP connection failed: {0}")]
    Tcp(String),
    
    #[error("HTTP request failed: {0}")]
    Http(String),
    
    #[error("Traceroute failed: {0}")]
    Traceroute(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Permission denied: {0}")]
    Permission(String),
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, NetworkError>;