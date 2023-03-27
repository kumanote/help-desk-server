use super::{client::Client, url::LineBotApiUrl};
use crate::{LineClient, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetWebhookEndpointResult {
    pub endpoint: String,
    pub active: bool,
}

impl LineClient {
    pub async fn get_webhook_endpoint(&self) -> Result<GetWebhookEndpointResult> {
        let url = LineBotApiUrl::GetWebhookEndpoint.build_url();
        println!("url: {}", url);
        let client = Client::new(&url)?;
        client.get(&self.channel_access_token).await
    }
}
