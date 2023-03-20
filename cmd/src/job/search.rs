use crate::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct SearchJobArgs {
    /// Config file path
    #[arg(short = 'c', long, default_value = "app.toml")]
    config: Option<PathBuf>,

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

impl Into<job_config::AppArgs> for SearchJobArgs {
    fn into(self) -> job_config::AppArgs {
        job_config::AppArgs {
            database: Some(job_config::DatabaseArgs {
                url: self.database_url,
                max_connection_pool_size: self.database_max_connection_pool_size,
            }),
            cache: Some(job_config::CacheArgs {
                url: self.cache_url,
                max_connection_pool_size: self.cache_max_connection_pool_size,
            }),
            queue: Some(job_config::QueueArgs {
                url: self.queue_url,
                max_connection_pool_size: self.queue_max_connection_pool_size,
            }),
            search: Some(job_config::SearchArgs {
                meilisearch_host: self.meilisearch_host,
                meilisearch_api_key: self.meilisearch_api_key,
            }),
        }
    }
}

impl SearchJobArgs {
    pub async fn run(self) -> Result<()> {
        let config_file_path = self.config.clone();
        let config_args: job_config::AppArgs = self.into();
        let app_config = job_config::AppConfig::build(config_file_path, config_args)?;
        // TODO
        println!("{:?}", app_config);
        Ok(())
    }
}
