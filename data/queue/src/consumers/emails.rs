use super::{names::ConsumerName, pull_subscribe};
use crate::{NatsSubject, QueueConnection, Result};
use nats::jetstream::PullSubscription;

pub fn subscribe_emails(conn: QueueConnection) -> Result<PullSubscription> {
    let subject = NatsSubject::Emails;
    let consumer_name = ConsumerName::Emails;
    pull_subscribe(conn, &subject, &consumer_name)
}
