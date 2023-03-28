mod names;

pub mod inquiry_incoming_event;
pub mod search;

use crate::{QueueConnection, Result};
use names::ConsumerName;
use nats::jetstream::PullSubscription;

fn pull_subscribe(conn: QueueConnection, consumer_name: &ConsumerName) -> Result<PullSubscription> {
    let jet_stream = nats::jetstream::new(conn);
    let stream = consumer_name.get_stream();
    let subject = consumer_name.get_subject();
    let stream_name = stream.to_string();
    let consumer_name = consumer_name.to_string();
    let option = nats::jetstream::PullSubscribeOptions::new()
        .bind_stream(stream_name)
        .durable_name(consumer_name);
    jet_stream
        .pull_subscribe_with_options(&subject, &option)
        .map_err(Into::into)
}
