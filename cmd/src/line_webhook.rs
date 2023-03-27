use crate::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct LineWebhookArgs {
    /// Config file path
    #[arg(short = 'c', long, default_value = "app.toml")]
    config: Option<PathBuf>,

    /// Line channel secret
    #[arg(short = 's', long)]
    line_channel_secret: Option<String>,
    /// Line channel access token
    #[arg(short = 'a', long)]
    line_channel_access_token: Option<String>,
    /// Bind address of line webhook server
    #[arg(short = 'b', long)]
    line_webhook_bind_address: Option<String>,

    /// Queue URL
    #[arg(long)]
    queue_url: Option<String>,
    /// Queue max connection pool size
    #[arg(long)]
    queue_max_connection_pool_size: Option<u32>,
}

impl Into<line_webhook_config::AppArgs> for LineWebhookArgs {
    fn into(self) -> line_webhook_config::AppArgs {
        line_webhook_config::AppArgs {
            line_webhook: Some(line_webhook_config::LineWebhookArgs {
                channel_secret: self.line_channel_secret,
                channel_access_token: self.line_channel_access_token,
                bind_address: self.line_webhook_bind_address,
            }),
            queue: Some(line_webhook_config::QueueArgs {
                url: self.queue_url,
                max_connection_pool_size: self.queue_max_connection_pool_size,
            }),
        }
    }
}

impl LineWebhookArgs {
    pub async fn run(self) -> Result<()> {
        let config_file_path = self.config.clone();
        let config_args: line_webhook_config::AppArgs = self.into();
        let app_config = line_webhook_config::AppConfig::build(config_file_path, config_args)?;
        line_webhook_server::start(app_config)
            .await
            .map_err(Into::into)
    }
}
