use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Invalid configuration... {cause}")]
    ImproperConfigError { cause: String },
    #[error("AgentRestApiServerError: {cause}")]
    AgentRestApiServerError {
        cause: agent_rest_server::ServerError,
    },
    #[error("JobExecutionError: {cause}")]
    JobExecutionError { cause: job::Error },
}

impl From<agent_rest_config::Error> for Error {
    fn from(cause: agent_rest_config::Error) -> Self {
        Self::ImproperConfigError {
            cause: format!("{}", cause),
        }
    }
}

impl From<job_config::Error> for Error {
    fn from(cause: job_config::Error) -> Self {
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

impl From<job::Error> for Error {
    fn from(cause: job::Error) -> Self {
        Self::JobExecutionError { cause }
    }
}
