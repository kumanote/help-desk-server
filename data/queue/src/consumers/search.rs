use super::{names::ConsumerName, pull_subscribe};
use crate::{QueueConnection, Result};
use nats::jetstream::PullSubscription;

pub fn subscribe_search_engine_tasks(conn: QueueConnection) -> Result<PullSubscription> {
    let consumer_name = ConsumerName::AllSearch;
    pull_subscribe(conn, &consumer_name)
}
