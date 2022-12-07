use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IOError: {cause:?}")]
    SerdeJsonError { cause: serde_json::Error },

    #[error("MeilisearchError: {cause:?}")]
    MeilisearchError {
        cause: meilisearch_sdk::errors::Error,
    },
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Self {
        Self::SerdeJsonError { cause }
    }
}

impl From<meilisearch_sdk::errors::Error> for Error {
    fn from(cause: meilisearch_sdk::errors::Error) -> Self {
        Self::MeilisearchError { cause }
    }
}
