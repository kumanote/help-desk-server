use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("IOError: {cause:?}")]
    IOError { cause: std::io::Error },
    #[error("TomlError: {cause:?}")]
    TomlError { cause: toml::de::Error },
}

impl From<std::io::Error> for Error {
    fn from(cause: std::io::Error) -> Self {
        Self::IOError { cause }
    }
}

impl From<toml::de::Error> for Error {
    fn from(cause: toml::de::Error) -> Self {
        Self::TomlError { cause }
    }
}
