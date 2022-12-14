use anyhow::anyhow;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("UnsupportedLocale: {value}")]
    UnsupportedLocale { value: String },
    #[error("UnsupportedScope: {value}")]
    UnsupportedScope { value: String },
    #[error("InvalidId: {cause}")]
    InvalidId { cause: ulid::DecodeError },
    /// in case inputted string is provided in invalid format.
    /// format check must be done by client side, thus the server dose not hold error details.
    #[error("InvalidFormat")]
    InvalidFormat,
    #[error("UnsupportedRteValue: {value}")]
    UnsupportedRteValue { value: String },
    #[error("SystemError: {cause}")]
    SystemError { cause: anyhow::Error },
}

impl From<ulid::DecodeError> for Error {
    fn from(cause: ulid::DecodeError) -> Self {
        Self::InvalidId { cause }
    }
}

impl From<pbkdf2::password_hash::Error> for Error {
    fn from(cause: pbkdf2::password_hash::Error) -> Self {
        Self::SystemError {
            cause: anyhow!(cause),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(cause: jsonwebtoken::errors::Error) -> Self {
        Self::SystemError {
            cause: anyhow!(cause),
        }
    }
}

impl From<cache::Error> for Error {
    fn from(cause: cache::Error) -> Self {
        Self::SystemError {
            cause: cause.into(),
        }
    }
}

impl From<database::Error> for Error {
    fn from(cause: database::Error) -> Self {
        Self::SystemError {
            cause: cause.into(),
        }
    }
}
