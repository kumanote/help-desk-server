use crate::{QueueAsyncConnection, QueueConnection, Result};

pub fn establish_connection<S: Into<String>>(nats_url: S) -> Result<QueueConnection> {
    let url_string: String = nats_url.into();
    nats::connect(&url_string).map_err(Into::into)
}

pub async fn establish_connection_async<S: Into<String>>(
    nats_url: S,
) -> Result<QueueAsyncConnection> {
    let url_string: String = nats_url.into();
    nats::asynk::connect(&url_string).await.map_err(Into::into)
}
