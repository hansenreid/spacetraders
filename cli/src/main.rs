use std::fs;
use std::path::Path;

use api::register::RegisterResponse;
use clap::Parser;
use eyre::{Ok, Result};
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
    let api_config = api::ApiConfig::new("https://api.spacetraders.io/v2".into());
    let _agent_config = get_conf();

    match config.command {
        Some(Command::Status) => {
            let resp = api::status::get(api_config).await?;
            println!("Status: {:?}", resp)
        }

        Some(Command::Register(args)) => {
            let req = api::register::RegisterRequest::new(args.faction, args.symbol);
            let res = api::register::register(api_config, req).await?;
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

fn write_conf(res: RegisterResponse) -> Result<()> {
    let file = Path::new("agent.toml");
    let agent_config = AgentConfig {
        token: res.data.token,
        agent: res.data.agent.symbol,
    };

    let str = toml::to_string::<AgentConfig>(&agent_config)?;
    fs::write(file, str)?;
    Ok(())
}
