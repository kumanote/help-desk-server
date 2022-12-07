mod conn;
mod error;
mod pool;
mod subjects;

pub use conn::*;
pub use error::*;
pub use pool::*;
use subjects::*;

pub mod consumers;
pub mod entities;
pub mod publishers;

pub mod prelude {
    pub use nats::jetstream::PullSubscription;
}
pub type R2D2Error = r2d2::Error;
pub type QueueConnection = nats::Connection;
pub type QueueAsyncConnection = nats::asynk::Connection;
pub type QueueConnectionPool = r2d2::Pool<NatsConnectionManager>;
pub type Result<T> = core::result::Result<T, Error>;
