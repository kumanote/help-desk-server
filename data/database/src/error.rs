use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("R2D2Error: {cause:?}")]
    R2D2Error { cause: r2d2::Error },
    #[error("ConnectionError: {cause:?}")]
    ConnectionError { cause: diesel::ConnectionError },
    #[error("DieselError: {cause:?}")]
    DieselError { cause: diesel::result::Error },
}

impl From<r2d2::Error> for Error {
    fn from(cause: r2d2::Error) -> Self {
        Self::R2D2Error { cause }
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(cause: diesel::ConnectionError) -> Self {
        Self::ConnectionError { cause }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(cause: diesel::result::Error) -> Self {
        Self::DieselError { cause }
    }
}
