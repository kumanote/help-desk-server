mod names;

pub mod emails;
pub mod search;

use crate::{NatsSubject, QueueConnection, Result};
use names::ConsumerName;
use nats::jetstream::PullSubscription;

fn pull_subscribe(
    conn: QueueConnection,
    subject: &NatsSubject,
    consumer_name: &ConsumerName,
) -> Result<PullSubscription> {
    let jet_stream = nats::jetstream::new(conn);
    let subject = subject.to_string();
    let consumer_name = consumer_name.to_string();
    let option = nats::jetstream::PullSubscribeOptions::new().durable_name(consumer_name);
    jet_stream
        .pull_subscribe_with_options(&subject, &option)
        .map_err(Into::into)
}
