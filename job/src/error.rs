use anyhow::anyhow;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("DatabaseConnectionPoolError: {cause}")]
    DatabaseConnectionPoolError { cause: database::Error },
    #[error("CacheConnectionPoolError: {cause}")]
    CacheConnectionPoolError { cause: cache::Error },
    #[error("QueueConnectionPoolError: {cause}")]
    QueueConnectionPoolError { cause: queue::Error },
    #[error("SystemError: {cause}")]
    SystemError { cause: anyhow::Error },
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Self::SystemError {
            cause: anyhow!(cause),
        }
    }
}

impl From<database::Error> for Error {
    fn from(cause: database::Error) -> Self {
        Self::DatabaseConnectionPoolError { cause }
    }
}

impl From<database::R2D2Error> for Error {
    fn from(cause: database::R2D2Error) -> Self {
        Self::SystemError {
            cause: anyhow!(cause),
        }
    }
}
impl From<cache::Error> for Error {
    fn from(cause: cache::Error) -> Self {
        Self::CacheConnectionPoolError { cause }
    }
}

impl From<queue::Error> for Error {
    fn from(cause: queue::Error) -> Self {
        Self::QueueConnectionPoolError { cause }
    }
}

impl From<domain::Error> for Error {
    fn from(cause: domain::Error) -> Self {
        Self::SystemError {
            cause: cause.into(),
        }
    }
}
