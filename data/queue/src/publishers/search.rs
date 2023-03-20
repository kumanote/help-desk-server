use super::{publish, publish_async};
use crate::entities::Search;
use crate::{NatsSubject, QueueAsyncConnection, QueueConnection, Result};

pub fn publish_search_engine_task(conn: &mut QueueConnection, msg: Search) -> Result<()> {
    let subject = NatsSubject::Search;
    let msg = serde_json::to_vec(&msg)?;
    publish(conn, &subject, msg)
}

pub async fn publish_email_task_async(conn: &mut QueueAsyncConnection, msg: Search) -> Result<()> {
    let subject = NatsSubject::Search;
    let msg = serde_json::to_vec(&msg)?;
    publish_async(conn, &subject, msg).await
}
