use crate::Result;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Deserialize, PartialEq, Clone, Default)]
pub struct AppToml {
    pub database: Option<DatabaseToml>,
    pub cache: Option<CacheToml>,
    pub queue: Option<QueueToml>,
    pub search: Option<SearchToml>,
    pub line: Option<LineToml>,
}

impl AppToml {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        toml::from_str(&contents).map_err(Into::into)
    }
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub struct DatabaseToml {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub struct CacheToml {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub struct QueueToml {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub struct SearchToml {
    pub meilisearch_host: Option<String>,
    pub meilisearch_api_key: Option<String>,
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub struct LineToml {
    pub channel_access_token: Option<String>,
}
