use chrono::Duration;

pub const AGENT_ACCESS_TOKEN_EXPIRE_DAYS: i64 = 14;
pub const AGENT_LOGIN_USERNAME_FAILURE_LIMIT: i32 = 10;
pub const AGENT_LOGIN_IP_FAILURE_LIMIT: i32 = 10;

pub fn get_agent_access_token_expire_duration() -> Duration {
    Duration::days(AGENT_ACCESS_TOKEN_EXPIRE_DAYS)
}
