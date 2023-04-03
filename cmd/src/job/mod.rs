mod inquiry_incoming_event;
mod search;

pub use inquiry_incoming_event::*;
pub use search::*;

use crate::Result;
use clap::Subcommand;

/// Subcommand for background task execution.
#[derive(Subcommand)]
pub enum JobSubcommand {
    Search(SearchJobArgs),
    InquiryIncomingEvent(InquiryIncomingEventJobArgs),
}

impl JobSubcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Search(args) => args.run().await,
            Self::InquiryIncomingEvent(args) => args.run().await,
        }
    }
}
