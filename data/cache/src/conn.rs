use crate::{CacheClient, CacheConnection, Result};

pub fn establish_connection<S: Into<String>>(redis_url: S) -> Result<CacheConnection> {
    let redis_url = redis_url.into();
    let client = CacheClient::open(redis_url)?;
    Ok(client.get_connection()?)
}
