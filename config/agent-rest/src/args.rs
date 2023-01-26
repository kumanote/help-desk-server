#[derive(Clone, Debug, PartialEq, Default)]
pub struct AppArgs {
    pub agent_rest: Option<AgentRestArgs>,
    pub database: Option<DatabaseArgs>,
    pub cache: Option<CacheArgs>,
    pub queue: Option<QueueArgs>,
    pub search: Option<SearchArgs>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AgentRestArgs {
    pub secret_key: Option<String>,
    pub bind_address: Option<String>,
    pub allowed_origins: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DatabaseArgs {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CacheArgs {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct QueueArgs {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchArgs {
    pub meilisearch_host: Option<String>,
    pub meilisearch_api_key: Option<String>,
}
