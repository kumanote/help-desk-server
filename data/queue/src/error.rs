use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IOError: {cause:?}")]
    IOError { cause: std::io::Error },

    #[error("IOError: {cause:?}")]
    SerdeJsonError { cause: serde_json::Error },

    #[error("R2D2Error: {cause:?}")]
    R2D2Error { cause: r2d2::Error },

    #[error("NatsJetStreamError: {cause:?}")]
    NatsJetStreamError { cause: nats::jetstream::Error },
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Self::IOError { cause }
    }
}

impl From<r2d2::Error> for Error {
    fn from(cause: r2d2::Error) -> Self {
        Self::R2D2Error { cause }
    }
}

impl From<nats::jetstream::Error> for Error {
    fn from(cause: nats::jetstream::Error) -> Self {
        Self::NatsJetStreamError { cause }
    }
}

impl From<serde_json::Error> for Error {
    fn from(cause: serde_json::Error) -> Self {
        Self::SerdeJsonError { cause }
    }
}
