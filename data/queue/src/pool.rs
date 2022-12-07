use crate::{establish_connection, Error, QueueConnection, QueueConnectionPool, Result};

#[derive(Debug)]
pub struct NatsConnectionManager {
    nats_url: String,
}

impl NatsConnectionManager {
    fn new<S: Into<String>>(nats_url: S) -> Self {
        Self {
            nats_url: nats_url.into(),
        }
    }
}

impl r2d2::ManageConnection for NatsConnectionManager {
    type Connection = QueueConnection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection> {
        establish_connection(&self.nats_url)
    }

    /// Flush a NATS connection by sending a PING protocol and waiting for the responding PONG.
    /// Will fail with TimedOut if the server does not respond with in 10 seconds.
    /// Will fail with NotConnected if the server is not currently connected.
    /// Will fail with BrokenPipe if the connection to the server is lost.
    fn is_valid(&self, conn: &mut Self::Connection) -> Result<()> {
        conn.flush().map_err(Into::into)
    }

    fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
        false
    }
}

pub fn new_pool<S: Into<String>>(nats_url: S, max_size: u32) -> Result<QueueConnectionPool> {
    let manager = NatsConnectionManager::new(nats_url);
    QueueConnectionPool::builder()
        .max_size(max_size)
        .build(manager)
        .map_err(Into::into)
}
