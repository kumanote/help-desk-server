use super::commands;
use crate::{keys::Key, CacheConnection, Result};
use chrono::Duration;

const VALUE: &'static str = "1";

pub fn is_blocked(conn: &mut CacheConnection, ip_address: &str) -> Result<bool> {
    let key = Key::LoginBlockedByIp {
        ip_address: ip_address.to_owned(),
    }
    .to_string();
    let result: Option<String> = commands::get(conn, &key)?;
    Ok(result.is_some())
}

pub fn set_blocked<D: Into<Duration>>(
    conn: &mut CacheConnection,
    ip_address: &str,
    ttl: D,
) -> Result<()> {
    let key = Key::LoginBlockedByIp {
        ip_address: ip_address.to_string(),
    }
    .to_string();
    let ttl = ttl.into();
    commands::set(conn, &key, VALUE)?;
    commands::expire(conn, &key, ttl)?;
    Ok(())
}
