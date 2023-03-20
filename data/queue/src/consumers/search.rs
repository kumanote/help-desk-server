use super::{names::ConsumerName, pull_subscribe};
use crate::{NatsSubject, QueueConnection, Result};
use nats::jetstream::PullSubscription;

pub fn subscribe_search_engine_tasks(conn: QueueConnection) -> Result<PullSubscription> {
    let subject = NatsSubject::Search;
    let consumer_name = ConsumerName::Search;
    pull_subscribe(conn, &subject, &consumer_name)
}
