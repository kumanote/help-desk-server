use crate::Result;
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct SearchJobArgs {
    /// Config file path
    #[arg(short = 'c', long, default_value = "app.toml")]
    config: Option<PathBuf>,
}

impl SearchJobArgs {
    pub async fn run(self) -> Result<()> {
        let config_file_path = self.config.clone();
        // TODO
        println!("{:?}", config_file_path);
        Ok(())
    }
}
