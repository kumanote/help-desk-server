use super::commands;
use crate::{keys::Key, CacheConnection, Result};
use chrono::Duration;

pub fn is_valid(conn: &mut CacheConnection, agent_id: &str, token: &str) -> Result<bool> {
    let key = Key::AgentAccessToken {
        agent_id: agent_id.to_string(),
        token: token.to_string(),
    }
    .to_string();
    let result: Option<String> = commands::get(conn, &key)?;
    Ok(result.is_some())
}

pub fn set<D: Into<Duration>>(
    conn: &mut CacheConnection,
    agent_id: &str,
    token: &str,
    ttl: D,
) -> Result<()> {
    let key = Key::AgentAccessToken {
        agent_id: agent_id.to_string(),
        token: token.to_string(),
    };
    let key = key.to_string();
    let ttl = ttl.into();
    commands::set(conn, &key, "1")?;
    commands::expire(conn, &key, ttl)?;
    Ok(())
}

pub fn delete(conn: &mut CacheConnection, agent_id: &str, token: &str) -> Result<()> {
    let key = Key::AgentAccessToken {
        agent_id: agent_id.to_string(),
        token: token.to_string(),
    }
    .to_string();
    commands::del(conn, &key)
}
