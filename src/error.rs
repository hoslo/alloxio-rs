use thiserror::Error;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Init(#[from] reqwest::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("from_utf8 error: {0}")]
    FromUtf8(#[from] std::str::Utf8Error),
    #[error("server error: {0}")]
    Server(String),
}