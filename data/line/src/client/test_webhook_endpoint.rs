use super::{client::Client, url::LineBotApiUrl};
use crate::{LineClient, Result};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct TestWebhookEndpointResult {
    pub success: bool,
    pub timestamp: String,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
    pub reason: String,
    pub detail: String,
}

impl LineClient {
    pub async fn test_webhook_endpoint(&self, endpoint: &str) -> Result<TestWebhookEndpointResult> {
        let url = LineBotApiUrl::TestWebhookEndpoint.build_url();
        let client = Client::new(&url)?;
        let params: serde_json::Value = json!(
            {
                "endpoint": endpoint,
            }
        );
        client.post(params, &self.channel_access_token).await
    }
}
