use crate::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct AgentRestArgs {
    /// Config file path
    #[arg(short = 'c', long, default_value = "app.toml")]
    config: Option<PathBuf>,

    /// Secret key of agent rest api server
    #[arg(short = 's', long)]
    agent_rest_secret_key: Option<String>,
    /// Bind address of agent rest api server
    #[arg(short = 'b', long)]
    agent_rest_bind_address: Option<String>,
    /// Allowed origins of agent rest api server
    #[arg(short = 'o', long)]
    agent_rest_allowed_origins: Option<Vec<String>>,

    /// Database URL
    #[arg(long)]
    database_url: Option<String>,
    /// Database max connection pool size
    #[arg(long)]
    database_max_connection_pool_size: Option<u32>,

    /// Cache URL
    #[arg(long)]
    cache_url: Option<String>,
    /// Cache max connection pool size
    #[arg(long)]
    cache_max_connection_pool_size: Option<u32>,

    /// Queue URL
    #[arg(long)]
    queue_url: Option<String>,
    /// Queue max connection pool size
    #[arg(long)]
    queue_max_connection_pool_size: Option<u32>,

    /// Hostname of Meilisearch
    #[arg(long)]
    meilisearch_host: Option<String>,
    /// API key of Meilisearch
    #[arg(long)]
    meilisearch_api_key: Option<String>,
}

impl Into<agent_rest_config::AppArgs> for AgentRestArgs {
    fn into(self) -> agent_rest_config::AppArgs {
        agent_rest_config::AppArgs {
            agent_rest: Some(agent_rest_config::AgentRestArgs {
                secret_key: self.agent_rest_secret_key,
                bind_address: self.agent_rest_bind_address,
                allowed_origins: self.agent_rest_allowed_origins,
            }),
            database: Some(agent_rest_config::DatabaseArgs {
                url: self.database_url,
                max_connection_pool_size: self.database_max_connection_pool_size,
            }),
            cache: Some(agent_rest_config::CacheArgs {
                url: self.cache_url,
                max_connection_pool_size: self.cache_max_connection_pool_size,
            }),
            queue: Some(agent_rest_config::QueueArgs {
                url: self.queue_url,
                max_connection_pool_size: self.queue_max_connection_pool_size,
            }),
            search: Some(agent_rest_config::SearchArgs {
                meilisearch_host: self.meilisearch_host,
                meilisearch_api_key: self.meilisearch_api_key,
            }),
        }
    }
}

impl AgentRestArgs {
    pub async fn run(self) -> Result<()> {
        let config_file_path = self.config.clone();
        let config_args: agent_rest_config::AppArgs = self.into();
        let app_config = agent_rest_config::AppConfig::build(config_file_path, config_args)?;
        agent_rest_server::start(app_config)
            .await
            .map_err(Into::into)
    }
}
