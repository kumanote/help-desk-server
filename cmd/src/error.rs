use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid configuration... {cause}")]
    ImproperConfigError { cause: String },
    #[error("AgentRestApiServerError: {cause}")]
    AgentRestApiServerError {
        cause: agent_rest_server::ServerError,
    },
}

impl From<agent_rest_config::Error> for Error {
    fn from(cause: agent_rest_config::Error) -> Self {
        Self::ImproperConfigError {
            cause: format!("{}", cause),
        }
    }
}

impl From<agent_rest_server::ServerError> for Error {
    fn from(cause: agent_rest_server::ServerError) -> Self {
        Self::AgentRestApiServerError { cause }
    }
}
