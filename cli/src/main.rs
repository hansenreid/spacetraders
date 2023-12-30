use anyhow::{Ok, Result};
use clap::Parser;

#[derive(Debug, Parser)]
enum Command {
    Register(RegisterArgs),
    Status,
}

#[derive(Debug, Parser)]
struct Config {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Parser)]
struct RegisterArgs {
    #[arg(short, long)]
    symbol: String,

    #[arg(short, long)]
    faction: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = clap::Parser::parse();
    let api_config = api::ApiConfig::new("https://api.spacetraders.io/v2/".into());

    match config.command {
        Some(Command::Status) => {
            let resp = api::status::get(api_config).await?;
            println!("Status: {:?}", resp)
        }

        Some(Command::Register(args)) => {
            println!("Register: {:?}", args)
        }

        None => {
            println!("No command provided")
        }
    }

    Ok(())
}
