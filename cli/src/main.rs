use std::fs;
use std::path::Path;

use clap::Parser;
use eyre::{Ok, Result};
use openapi::{apis, models};
use serde::{Deserialize, Serialize};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AgentConfig {
    token: String,
    agent: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let config: Config = clap::Parser::parse();
    let _agent_config = get_conf();
    let conf = apis::configuration::Configuration::new();

    match config.command {
        Some(Command::Status) => {
            let openapi_response = apis::default_api::get_status(&conf).await?;
            println!("{:#?}", openapi_response);
        }

        Some(Command::Register(args)) => {
            let faction = models::FactionSymbol::Cosmic;
            let symbol = args.symbol.clone();
            let req = models::RegisterRequest::new(faction, symbol);
            let res = apis::default_api::register(&conf, Some(req)).await?;

            write_conf(res)?;

            println!("Registered successfully. Agent file has been updated")
        }

        None => {
            println!("No command provided")
        }
    }

    Ok(())
}

fn get_conf() -> Result<Option<AgentConfig>> {
    let file = Path::new("agent.toml");
    match file.exists() {
        true => {
            let conf = fs::read_to_string(file)?;
            let conf: AgentConfig = toml::from_str(&conf)?;
            Ok(Some(conf))
        }
        false => Ok(None),
    }
}

fn write_conf(res: models::Register201Response) -> Result<()> {
    let file = Path::new("agent.toml");
    let agent_config = AgentConfig {
        token: res.data.token,
        agent: res.data.agent.symbol,
    };

    let str = toml::to_string::<AgentConfig>(&agent_config)?;
    fs::write(file, str)?;
    Ok(())
}
