use crate::{CacheClient, CacheConnectionPool, Result};

pub fn new_pool<S: Into<String>>(redis_url: S, max_size: u32) -> Result<CacheConnectionPool> {
    let redis_url = redis_url.into();
    let client = CacheClient::open(redis_url)?;
    CacheConnectionPool::builder()
        .max_size(max_size)
        .build(client)
        .map_err(Into::into)
}
