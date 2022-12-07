use super::{publish, publish_async};
use crate::entities::Emails;
use crate::{NatsSubject, QueueAsyncConnection, QueueConnection, Result};

pub fn publish_email_task(conn: &mut QueueConnection, msg: Emails) -> Result<()> {
    let subject = NatsSubject::Emails;
    let msg = serde_json::to_vec(&msg)?;
    publish(conn, &subject, msg)
}

pub async fn publish_email_task_async(conn: &mut QueueAsyncConnection, msg: Emails) -> Result<()> {
    let subject = NatsSubject::Emails;
    let msg = serde_json::to_vec(&msg)?;
    publish_async(conn, &subject, msg).await
}
