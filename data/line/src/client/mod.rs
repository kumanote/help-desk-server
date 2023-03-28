mod client;
mod url;

mod get_profile;
mod get_webhook_endpoint;
mod send_push_message;
mod test_webhook_endpoint;

pub use get_profile::*;
pub use get_webhook_endpoint::*;
pub use send_push_message::*;
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
