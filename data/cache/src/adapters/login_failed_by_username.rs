use super::commands;
use crate::{keys::Key, CacheConnection, Result};
use chrono::Duration;

pub fn increment<D: Into<Duration>>(
    conn: &mut CacheConnection,
    username: &str,
    ttl: D,
) -> Result<i32> {
    let key = Key::LoginFailedByUsername {
        username: username.to_owned(),
    }
    .to_string();
    let ttl = ttl.into();
    let result: i32 = commands::increment(conn, &key, 1)?;
    commands::expire(conn, &key, ttl)?;
    Ok(result)
}

pub fn delete(conn: &mut CacheConnection, username: &str) -> Result<()> {
    let key = Key::LoginFailedByUsername {
        username: username.to_string(),
    }
    .to_string();
    commands::del(conn, &key)?;
    Ok(())
}
