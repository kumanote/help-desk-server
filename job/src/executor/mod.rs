mod search;

pub use self::search::*;

use crate::{config, Result};
use job_config::AppConfig;

pub enum JobExecutor {
    Search(SearchJobExecutor),
}

impl JobExecutor {
    pub fn new_search() -> Self {
        Self::Search(SearchJobExecutor::new())
    }

    pub async fn start(self, app_config: AppConfig) -> Result<()> {
        // set global app config
        config::set_app_config(app_config);
        match self {
            Self::Search(executor) => executor.start().await,
        }
    }
}
