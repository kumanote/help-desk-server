use super::{client::Client, url::LineBotApiUrl};
use crate::{messages::SendMessageType, LineClient, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SendPushMessageParams {
    to: String,
    messages: Vec<SendMessageType>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    notification_disabled: Option<bool>,
    #[serde(
        rename = "customAggregationUnits",
        skip_serializing_if = "Option::is_none"
    )]
    custom_aggregation_units: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendPushMessageResult {}

impl LineClient {
    pub async fn send_push_message(
        &self,
        params: SendPushMessageParams,
    ) -> Result<SendPushMessageResult> {
        let url = LineBotApiUrl::SendPushMessage.build_url();
        let client = Client::new(&url)?;
        client.post(params, &self.channel_access_token).await
    }
}
