use crate::Result;
use serde::Deserialize;
use std::{fs::File, io::Read, path::Path};

#[derive(Deserialize, PartialEq, Clone, Default)]
pub struct AppToml {
    pub line_webhook: Option<LineWebhookToml>,
    pub queue: Option<QueueToml>,
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
pub struct LineWebhookToml {
    pub channel_secret: Option<String>,
    pub channel_access_token: Option<String>,
    pub bind_address: Option<String>,
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub struct QueueToml {
    pub url: Option<String>,
    pub max_connection_pool_size: Option<u32>,
}
