pub mod inquiry_incoming_event;
pub mod search;

use crate::{NatsSubject, QueueAsyncConnection, QueueConnection, Result};

fn publish(conn: &mut QueueConnection, subject: &NatsSubject, msg: impl AsRef<[u8]>) -> Result<()> {
    let subject = subject.to_string();
    conn.publish(&subject, msg).map_err(Into::into)
}

async fn publish_async(
    conn: &mut QueueAsyncConnection,
    subject: &NatsSubject,
    msg: impl AsRef<[u8]>,
) -> Result<()> {
    let subject = subject.to_string();
    conn.publish(&subject, msg).await.map_err(Into::into)
}
