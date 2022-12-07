use crate::{CacheConnection, Result};
use redis::{Commands, FromRedisValue, ToRedisArgs};

#[allow(dead_code)]
pub(crate) fn getset<K: ToRedisArgs, V: ToRedisArgs + FromRedisValue>(
    conn: &mut CacheConnection,
    key: K,
    value: V,
) -> Result<Option<V>> {
    conn.getset(key, value).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn get<K: ToRedisArgs, T: FromRedisValue>(
    conn: &mut CacheConnection,
    key: K,
) -> Result<T> {
    conn.get(key).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn set<K: ToRedisArgs, V: ToRedisArgs>(
    conn: &mut CacheConnection,
    key: K,
    value: V,
) -> Result<()> {
    conn.set(key, value).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn expire<K: ToRedisArgs>(
    conn: &mut CacheConnection,
    key: K,
    seconds: chrono::Duration,
) -> Result<()> {
    conn.expire(key, seconds.num_seconds() as usize)
        .map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn del<K: ToRedisArgs>(conn: &mut CacheConnection, key: K) -> Result<()> {
    conn.del(key).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn increment<K: ToRedisArgs, V: ToRedisArgs + FromRedisValue>(
    conn: &mut CacheConnection,
    key: K,
    delta: V,
) -> Result<V> {
    conn.incr(key, delta).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn sadd<K: ToRedisArgs, V: ToRedisArgs>(
    conn: &mut CacheConnection,
    key: K,
    value: V,
) -> Result<()> {
    conn.sadd(key, value).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn srem<K: ToRedisArgs, V: ToRedisArgs>(
    conn: &mut CacheConnection,
    key: K,
    value: V,
) -> Result<()> {
    conn.srem(key, value).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn sismember<K: ToRedisArgs, V: ToRedisArgs>(
    conn: &mut CacheConnection,
    key: K,
    value: V,
) -> Result<bool> {
    conn.sismember(key, value).map_err(Into::into)
}

#[allow(dead_code)]
pub(crate) fn smembers<K: ToRedisArgs, V: FromRedisValue>(
    conn: &mut CacheConnection,
    key: K,
) -> Result<Vec<V>> {
    conn.smembers(key).map_err(Into::into)
}
