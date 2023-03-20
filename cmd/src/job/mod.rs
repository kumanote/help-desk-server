mod search;
pub use search::*;

use crate::Result;
use clap::Subcommand;

/// Subcommand for background task execution.
#[derive(Subcommand)]
pub enum JobSubcommand {
    Search(SearchJobArgs),
}

impl JobSubcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::Search(args) => args.run().await,
        }
    }
}
