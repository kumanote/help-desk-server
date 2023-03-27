use serde::Deserialize;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("UnsupportedUrl: {url}")]
    UnsupportedUrl { url: String },

    #[error("HyperHttpError: {cause}")]
    HyperHttpError { cause: hyper::http::Error },

    #[error("HyperError: {cause}")]
    HyperError { cause: hyper::Error },

    #[error("IOError: {cause:?}")]
    IOError { cause: std::io::Error },

    #[error("SerializationError: {cause:?}")]
    SerializationError { cause: serde_json::Error },

    #[error("UnexpectedErrorResponse: code: {status_code}, body: {response_body}")]
    UnexpectedErrorResponse {
        status_code: u16,
        response_body: String,
    },

    #[error("ErrorResponse: {0:?}")]
    ErrorResponse(ErrorResponse),
}

impl From<hyper::http::Error> for Error {
    fn from(cause: hyper::http::Error) -> Self {
        Self::HyperHttpError { cause }
    }
}

impl From<hyper::Error> for Error {
    fn from(cause: hyper::Error) -> Self {
        Self::HyperError { cause }
    }
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Self::IOError { cause }
    }
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Self {
        Self::SerializationError { cause }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub details: Option<Vec<ErrorResponseDetail>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponseDetail {
    pub message: Option<String>,
    pub property: Option<String>,
}
