#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppArgs {
    pub line_webhook: Option<LineWebhookArgs>,
    pub queue: Option<QueueArgs>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineWebhookArgs {
    pub channel_secret: Option<String>,
    pub channel_access_token: Option<String>,
    pub bind_address: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QueueArgs {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}
