use std::fmt;

const AGENT_ACCESS_TOKEN_PREFIX: &'static str = "agent_access_token";
pub(crate) const AUTH_PREFIX: &'static str = "auth";

pub enum Key {
    LoginBlockedByIp { ip_address: String },
    LoginBlockedByUsername { username: String },
    LoginFailedByIp { ip_address: String },
    LoginFailedByUsername { username: String },
    AgentAccessToken { agent_id: String, token: String },
    ResetPasswordToken { token: String },
    AuthAgentScopes { agent_id: String },
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoginBlockedByIp { ip_address } => write!(f, "login:blocked:ip:{}", ip_address),
            Self::LoginBlockedByUsername { username } => {
                write!(f, "login:blocked:username:{}", username)
            }
            Self::LoginFailedByIp { ip_address } => write!(f, "login:failed:ip:{}", ip_address),
            Self::LoginFailedByUsername { username } => {
                write!(f, "login:failed:username:{}", username)
            }
            Self::AgentAccessToken { agent_id, token } => {
                write!(f, "{}:{}:{}", AGENT_ACCESS_TOKEN_PREFIX, agent_id, token)
            }
            Self::ResetPasswordToken { token } => {
                write!(f, "reset_password_token:{}", token)
            }
            Self::AuthAgentScopes { agent_id } => {
                write!(f, "{}:agents:{}:scopes", AUTH_PREFIX, agent_id)
            }
        }
    }
}
