use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ServerError {
    #[error("QueueConnectionPoolError: {cause}")]
    QueueConnectionPoolError { cause: queue::Error },
    #[error("Invalid configuration... {cause}")]
    ImproperConfigError { cause: String },
    #[error("ServerError: {cause}")]
    ServerError { cause: anyhow::Error },
}

impl From<queue::Error> for ServerError {
    fn from(cause: queue::Error) -> Self {
        Self::QueueConnectionPoolError { cause }
    }
}

#[derive(ThisError, Debug)]
pub enum HttpError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("InternalServerError: {cause}")]
    InternalServerError { cause: anyhow::Error },
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid signature"),
            Self::InternalServerError { cause } => {
                eprintln!("{:?}", cause);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            },
        }
        .into_response()
    }
}

impl From<domain::Error> for HttpError {
    fn from(value: domain::Error) -> Self {
        Self::InternalServerError {
            cause: value.into(),
        }
    }
}
