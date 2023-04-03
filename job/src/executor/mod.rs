mod inquiry_incoming_event;
mod search;

pub use self::search::*;
pub use inquiry_incoming_event::*;

use crate::{config, Result};
use job_config::AppConfig;

pub enum JobExecutor {
    Search(SearchJobExecutor),
    InquiryIncomingEvent(InquiryIncomingEventJobExecutor),
}

impl JobExecutor {
    pub fn new_search() -> Self {
        Self::Search(SearchJobExecutor::new())
    }

    pub fn new_inquiry_incoming_event() -> Self {
        Self::InquiryIncomingEvent(InquiryIncomingEventJobExecutor::new())
    }

    pub async fn start(self, app_config: AppConfig) -> Result<()> {
        // set global app config
        config::set_app_config(app_config);
        match self {
            Self::Search(executor) => executor.start().await,
            Self::InquiryIncomingEvent(executor) => executor.start().await,
        }
    }
}
