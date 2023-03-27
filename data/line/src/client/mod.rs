mod client;
mod url;

mod get_webhook_endpoint;
mod test_webhook_endpoint;

pub use get_webhook_endpoint::*;
pub use test_webhook_endpoint::*;

#[derive(Debug)]
pub struct LineClient {
    channel_access_token: String,
}

impl LineClient {
    pub fn new(channel_access_token: &str) -> Self {
        Self {
            channel_access_token: channel_access_token.to_owned(),
        }
    }
}
