mod agent_rest;
mod job;
mod line_webhook;

mod error;
use error::Error;
type Result<T> = core::result::Result<T, Error>;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub enum Cli {
    /// Start agent rest api server
    #[command(arg_required_else_help = false)]
    AgentRest(agent_rest::AgentRestArgs),
    /// Start agent rest api server
    #[command(arg_required_else_help = false)]
    LineWebhook(line_webhook::LineWebhookArgs),
    /// Start background task worker
    #[command(subcommand)]
    Job(job::JobSubcommand),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::AgentRest(args) => args.run().await,
            Self::LineWebhook(args) => args.run().await,
            Self::Job(subcommand) => subcommand.run().await,
        }
    }
}
