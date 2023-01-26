use anyhow::anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use internationalization::t;
use serde_json::json;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ServerError {
    #[error("DatabaseConnectionPoolError: {cause}")]
    DatabaseConnectionPoolError { cause: database::Error },
    #[error("CacheConnectionPoolError: {cause}")]
    CacheConnectionPoolError { cause: cache::Error },
    #[error("QueueConnectionPoolError: {cause}")]
    QueueConnectionPoolError { cause: queue::Error },
    #[error("Invalid configuration... {cause}")]
    ImproperConfigError { cause: String },
    #[error("ServerError: {cause}")]
    ServerError { cause: anyhow::Error },
}

impl From<database::Error> for ServerError {
    fn from(cause: database::Error) -> Self {
        Self::DatabaseConnectionPoolError { cause }
    }
}

impl From<cache::Error> for ServerError {
    fn from(cause: cache::Error) -> Self {
        Self::CacheConnectionPoolError { cause }
    }
}

impl From<queue::Error> for ServerError {
    fn from(cause: queue::Error) -> Self {
        Self::QueueConnectionPoolError { cause }
    }
}

#[derive(ThisError, Debug)]
pub enum HttpError {
    #[error("BadRequest: {reasons:?}")]
    BadRequest {
        reasons: Vec<serde_json::Value>,
        code: Option<String>,
    },
    #[error("Unauthorized: {detail}")]
    Unauthorized { detail: serde_json::Value },
    #[error("ServiceUnavailable: {detail}")]
    ServiceUnavailable { detail: serde_json::Value },
    #[error("InternalServerError: {cause}")]
    InternalServerError { cause: anyhow::Error },
}

impl HttpError {
    pub fn new_bad_request<S: std::fmt::Display>(reason: S) -> Self {
        Self::BadRequest {
            reasons: vec![json!(reason.to_string())],
            code: None,
        }
    }

    pub fn new_bad_request_with_code<S: std::fmt::Display>(reason: S, code: S) -> Self {
        Self::BadRequest {
            reasons: vec![json!(reason.to_string())],
            code: Some(code.to_string()),
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest { reasons, code } => {
                let body = if let Some(code) = code {
                    json!({
                        "error": {
                            "code": code,
                            "reasons": reasons,
                        }
                    })
                } else {
                    json!({
                        "error": {
                            "reasons": reasons,
                        }
                    })
                };
                (StatusCode::BAD_REQUEST, Json(body))
            }
            Self::Unauthorized { detail } => (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": {
                        "reasons": vec![detail],
                    }
                })),
            ),
            Self::ServiceUnavailable { detail } => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "error": {
                        "reasons": vec![detail],
                    }
                })),
            ),
            Self::InternalServerError { cause: _ } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": {
                        "reasons": vec!["oops...unknown error occurred..."],
                    }
                })),
            ),
        }
        .into_response()
    }
}

impl From<database::R2D2Error> for HttpError {
    fn from(cause: database::R2D2Error) -> Self {
        Self::InternalServerError {
            cause: anyhow!(cause),
        }
    }
}

impl From<database::DieselError> for HttpError {
    fn from(cause: database::DieselError) -> Self {
        Self::InternalServerError {
            cause: anyhow!(cause),
        }
    }
}

impl From<(domain::Error, &domain::model::Locale)> for HttpError {
    fn from((cause, locale): (domain::Error, &domain::model::Locale)) -> Self {
        match cause {
            domain::Error::UnsupportedLocale { value: _ } => {
                Self::new_bad_request(t!("validations.invalid_request", locale.as_str()))
            }
            domain::Error::UnsupportedScope { value: _ } => {
                Self::new_bad_request(t!("validations.invalid_request", locale.as_str()))
            }
            domain::Error::InvalidId { cause } => Self::InternalServerError {
                cause: anyhow!(cause),
            },
            domain::Error::InvalidFormat => {
                Self::new_bad_request(t!("validations.input_error", locale.as_str()))
            }
            domain::Error::InvalidRequest => {
                Self::new_bad_request(t!("validations.invalid_request", locale.as_str()))
            }
            domain::Error::UnsupportedRteValue { value: _ } => {
                Self::new_bad_request(t!("validations.input_error", locale.as_str()))
            }
            domain::Error::SystemError { cause } => Self::InternalServerError { cause },
        }
    }
}
