use crate::{args::*, toml::*, Result};
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppConfig {
    pub line_webhook: LineWebhookConfig,
    pub queue: QueueConfig,
}

impl AppConfig {
    pub fn build<P: AsRef<Path>>(
        config_file_path: Option<P>,
        app_args: AppArgs,
    ) -> Result<AppConfig> {
        let mut config = AppConfig::default();
        let app_toml = if let Some(path) = config_file_path {
            AppToml::load_from_file(path)?
        } else {
            AppToml::default()
        };
        config.line_webhook =
            LineWebhookConfig::build(app_toml.line_webhook, app_args.line_webhook);
        config.queue = QueueConfig::build(app_toml.queue, app_args.queue);
        Ok(config)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineWebhookConfig {
    pub channel_secret: String,
    pub channel_access_token: String,
    pub bind_address: String,
}

impl LineWebhookConfig {
    pub fn build(app_toml: Option<LineWebhookToml>, app_args: Option<LineWebhookArgs>) -> Self {
        let mut result = Self::default();
        if let Some(toml) = app_toml {
            if let Some(channel_secret) = toml.channel_secret {
                result.channel_secret = channel_secret;
            }
            if let Some(channel_access_token) = toml.channel_access_token {
                result.channel_access_token = channel_access_token;
            }
            if let Some(bind_address) = toml.bind_address {
                result.bind_address = bind_address;
            }
        }
        if let Some(args) = app_args {
            if let Some(channel_secret) = args.channel_secret {
                result.channel_secret = channel_secret;
            }
            if let Some(channel_access_token) = args.channel_access_token {
                result.channel_access_token = channel_access_token;
            }
            if let Some(bind_address) = args.bind_address {
                result.bind_address = bind_address;
            }
        }
        result
    }
}

impl Default for LineWebhookConfig {
    fn default() -> Self {
        Self {
            channel_secret: String::default(),
            channel_access_token: String::default(),
            bind_address: "0.0.0.0:8001".to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QueueConfig {
    pub url: String,
    pub max_connection_pool_size: u32,
}

impl QueueConfig {
    pub fn build(app_toml: Option<QueueToml>, app_args: Option<QueueArgs>) -> Self {
        let mut result = Self::default();
        if let Some(toml) = app_toml {
            if let Some(url) = toml.url {
                result.url = url;
            }
            if let Some(max_connection_pool_size) = toml.max_connection_pool_size {
                result.max_connection_pool_size = max_connection_pool_size;
            }
        }
        if let Some(args) = app_args {
            if let Some(url) = args.url {
                result.url = url;
            }
            if let Some(max_connection_pool_size) = args.max_connection_pool_size {
                result.max_connection_pool_size = max_connection_pool_size;
            }
        }
        result
    }
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            url: "127.0.0.1".to_owned(),
            max_connection_pool_size: 4,
        }
    }
}
