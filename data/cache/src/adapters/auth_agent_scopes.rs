use super::commands;
use crate::{
    keys::{Key, AUTH_PREFIX},
    CacheConnection, Result,
};
use chrono::Duration;

pub fn clear_all(conn: &mut CacheConnection) -> Result<()> {
    let keys = commands::keys(conn, format!("{}:*", AUTH_PREFIX))?;
    for key in &keys {
        commands::del(conn, &key)?;
    }
    Ok(())
}

pub fn delete_by_agent_id(conn: &mut CacheConnection, agent_id: &str) -> Result<()> {
    let key = Key::AuthAgentScopes {
        agent_id: agent_id.to_string(),
    };
    let key = key.to_string();
    commands::del(conn, &key)
}

pub fn add_scope_to_agent<D: Into<Duration>>(
    conn: &mut CacheConnection,
    agent_id: &str,
    scope: &str,
    ttl: D,
) -> Result<()> {
    let key = Key::AuthAgentScopes {
        agent_id: agent_id.to_string(),
    };
    let key = key.to_string();
    let ttl = ttl.into();
    commands::sadd(conn, &key, scope)?;
    commands::expire(conn, &key, ttl)?;
    Ok(())
}

pub fn add_scopes_to_agent<D: Into<Duration>>(
    conn: &mut CacheConnection,
    agent_id: &str,
    scopes: Vec<&str>,
    ttl: D,
) -> Result<()> {
    let key = Key::AuthAgentScopes {
        agent_id: agent_id.to_string(),
    };
    let key = key.to_string();
    let ttl = ttl.into();
    commands::sadd(conn, &key, &scopes)?;
    commands::expire(conn, &key, ttl)?;
    Ok(())
}

pub fn has_scopes(conn: &mut CacheConnection, agent_id: &str) -> Result<bool> {
    let key = Key::AuthAgentScopes {
        agent_id: agent_id.to_string(),
    };
    let key = key.to_string();
    let saved = commands::keys(conn, &key)?;
    Ok(!saved.is_empty())
}

pub fn check_agent_has_scope(
    conn: &mut CacheConnection,
    agent_id: &str,
    scope: &str,
) -> Result<bool> {
    let key = Key::AuthAgentScopes {
        agent_id: agent_id.to_string(),
    };
    let key = key.to_string();
    commands::sismember(conn, &key, scope)
}

pub fn check_agent_has_all_scopes(
    conn: &mut CacheConnection,
    agent_id: &str,
    scopes: Vec<&str>,
) -> Result<bool> {
    let key = Key::AuthAgentScopes {
        agent_id: agent_id.to_string(),
    };
    let key = key.to_string();
    let results: Vec<bool> = commands::smismember(conn, &key, &scopes)?;
    let missed = results.contains(&false);
    Ok(!missed)
}

pub fn get_all_scopes(conn: &mut CacheConnection, agent_id: &str) -> Result<Vec<String>> {
    let key = Key::AuthAgentScopes {
        agent_id: agent_id.to_string(),
    };
    let key = key.to_string();
    commands::smembers(conn, &key)
}
