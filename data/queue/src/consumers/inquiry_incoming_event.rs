use super::{names::ConsumerName, pull_subscribe};
use crate::{QueueConnection, Result};
use nats::jetstream::PullSubscription;

pub fn subscribe_inquiry_incoming_event_tasks(conn: QueueConnection) -> Result<PullSubscription> {
    let consumer_name = ConsumerName::AllInquiryIncomingEvents;
    pull_subscribe(conn, &consumer_name)
}
