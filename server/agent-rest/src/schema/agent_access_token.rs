use domain::model;
use serde::Serialize;

pub const AGENT_ACCESS_TOKEN_SCHEME: &'static str = "Bearer";

#[derive(Debug, Clone, Serialize)]
pub struct AgentAccessToken {
    pub access_token: String,
    pub token_type: &'static str,
}

impl From<model::AgentAccessToken> for AgentAccessToken {
    fn from(value: model::AgentAccessToken) -> Self {
        Self {
            access_token: value.into(),
            token_type: AGENT_ACCESS_TOKEN_SCHEME,
        }
    }
}
