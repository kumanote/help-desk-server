use super::{publish, publish_async};
use crate::entities::InquiryIncomingEvent;
use crate::{QueueAsyncConnection, QueueConnection, Result};

pub fn publish_inquiry_incoming_event_task(
    conn: &mut QueueConnection,
    msg: InquiryIncomingEvent,
) -> Result<()> {
    let subject = msg.get_subject();
    let msg = serde_json::to_vec(&msg)?;
    publish(conn, &subject, msg)
}

pub async fn publish_inquiry_incoming_event_task_async(
    conn: &mut QueueAsyncConnection,
    msg: InquiryIncomingEvent,
) -> Result<()> {
    let subject = msg.get_subject();
    let msg = serde_json::to_vec(&msg)?;
    publish_async(conn, &subject, msg).await
}
