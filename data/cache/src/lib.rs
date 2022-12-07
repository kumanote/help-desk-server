extern crate redis;

pub mod adapters;
pub mod keys;

mod conn;
mod error;
mod pool;

pub use conn::*;
pub use error::*;
pub use pool::*;

pub mod prelude {
    pub use redis::{ConnectionLike, FromRedisValue, RedisWrite, ToRedisArgs};
}
pub type R2D2Error = r2d2::Error;
pub type CacheValue = redis::Value;
pub type CacheConnection = redis::Connection;
pub type CacheClient = redis::Client;
pub type CacheConnectionPool = r2d2::Pool<CacheClient>;
pub type Result<T> = std::result::Result<T, Error>;
