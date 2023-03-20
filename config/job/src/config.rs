use crate::{args::*, toml::*, Result};
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub queue: QueueConfig,
    pub search: SearchConfig,
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
        config.database = DatabaseConfig::build(app_toml.database, app_args.database);
        config.cache = CacheConfig::build(app_toml.cache, app_args.cache);
        config.queue = QueueConfig::build(app_toml.queue, app_args.queue);
        config.search = SearchConfig::build(app_toml.search, app_args.search);
        Ok(config)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connection_pool_size: u32,
}

impl DatabaseConfig {
    pub fn build(app_toml: Option<DatabaseToml>, app_args: Option<DatabaseArgs>) -> Self {
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

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "mysql://root:@127.0.0.1:4000/test?charset=utf8mb4".to_owned(),
            max_connection_pool_size: 4,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheConfig {
    pub url: String,
    pub max_connection_pool_size: u32,
}

impl CacheConfig {
    pub fn build(app_toml: Option<CacheToml>, app_args: Option<CacheArgs>) -> Self {
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

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379/0".to_owned(),
            max_connection_pool_size: 4,
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

#[derive(Clone, Debug, PartialEq)]
pub struct SearchConfig {
    pub meilisearch_host: String,
    pub meilisearch_api_key: String,
}

impl SearchConfig {
    pub fn build(app_toml: Option<SearchToml>, app_args: Option<SearchArgs>) -> Self {
        let mut result = Self::default();
        if let Some(toml) = app_toml {
            if let Some(meilisearch_host) = toml.meilisearch_host {
                result.meilisearch_host = meilisearch_host;
            }
            if let Some(meilisearch_api_key) = toml.meilisearch_api_key {
                result.meilisearch_api_key = meilisearch_api_key;
            }
        }
        if let Some(args) = app_args {
            if let Some(meilisearch_host) = args.meilisearch_host {
                result.meilisearch_host = meilisearch_host;
            }
            if let Some(meilisearch_api_key) = args.meilisearch_api_key {
                result.meilisearch_api_key = meilisearch_api_key;
            }
        }
        result
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            meilisearch_host: "127.0.0.1".to_owned(),
            meilisearch_api_key: "MASTER_KEY".to_owned(),
        }
    }
}
