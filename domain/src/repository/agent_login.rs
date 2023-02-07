use crate::model::{AgentAccessToken, AgentId};
use chrono::Duration;

pub trait AgentLoginRepository: Send + Sync + 'static {
    type Err;
    type Transaction;
    fn is_blocked(&self, username: &str, ip_address: &str) -> Result<bool, Self::Err>;
    /// record login failed count
    /// return (failed count by username, failed count by ip address) tuple.
    fn record_login_failed(
        &self,
        username: &str,
        ip_address: &str,
    ) -> Result<(i32, i32), Self::Err>;
    fn clear_failed_count(&self, username: &str, ip_address: &str) -> Result<(), Self::Err>;
    fn set_login_blocked_by_username(&self, username: &str) -> Result<(), Self::Err>;
    fn set_login_blocked_by_ip(&self, ip_address: &str) -> Result<(), Self::Err>;
    fn set_access_token(
        &self,
        agent_id: &AgentId,
        token: &AgentAccessToken,
        ttl: Duration,
    ) -> Result<(), Self::Err>;
}
