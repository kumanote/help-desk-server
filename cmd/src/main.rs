use clap::Parser;
use help_desk_server::Cli;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();
    let exit_code = match cli.run().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        },
    };
    std::process::exit(exit_code);
}
