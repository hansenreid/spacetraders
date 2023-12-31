use std::path::Path;
use std::str::FromStr;
use std::{fs, vec};

use clap::Parser;
use eyre::{Ok, Result};
use inquire::{Select, Text};
use openapi::{apis, models};
use serde::{Deserialize, Serialize};
use tabled::Table;

#[derive(Debug, Parser)]
enum Command {
    Register,
    GetAgent,
    Status,
}

#[derive(Debug, Parser)]
struct Config {
    #[clap(subcommand)]
    command: Option<Command>,
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
    let agent_config = get_conf()?;
    let conf = apis::configuration::Configuration::new();

    match config.command {
        Some(Command::Status) => {
            let openapi_response = apis::default_api::get_status(&conf).await?;
            println!("{:#?}", openapi_response);
        }

        Some(Command::Register) => {
            let symbol = Text::new("What is your agent symbol?").prompt()?;

            let faction_answer =
                Select::new("Select a faction:", common::models::FactionSymbol::to_vec())
                    .prompt()?;
            let faction = common::models::FactionSymbol::from_str(faction_answer)?.into();

            let req = models::RegisterRequest::new(faction, symbol);
            let res = apis::default_api::register(&conf, Some(req)).await?;
            write_conf(&res)?;

            let agent = common::models::Agent::from(res.data.agent);
            let agent_table = vec![agent];
            let agent_table = Table::new(agent_table).to_string();
            println!("\nAgent\n{}\n", agent_table);

            let contract = common::models::Contract::from(res.data.contract);
            let contract_table = vec![contract];
            let contract_table = Table::new(contract_table).to_string();
            println!("\nContract\n{}\n", contract_table);

            println!("Registered successfully. Agent file has been updated")
        }

        Some(Command::GetAgent) => match agent_config {
            Some(agent_config) => {
                let api_config = get_authenticated_config(agent_config);
                let res = apis::agents_api::get_my_agent(&api_config).await?;

                let agent = common::models::Agent::from(res.data);
                let agent_table = vec![agent];
                let agent_table = Table::new(agent_table).to_string();
                println!("\n{}\n", agent_table);
            }

            None => println!("No agent found. Please register first"),
        },

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

fn get_authenticated_config(agent_config: AgentConfig) -> apis::configuration::Configuration {
    apis::configuration::Configuration {
        bearer_access_token: Some(agent_config.token),
        ..Default::default()
    }
}

fn write_conf(res: &models::Register201Response) -> Result<()> {
    let file = Path::new("agent.toml");
    let agent_config = AgentConfig {
        token: res.data.token.clone(),
        agent: res.data.agent.symbol.clone(),
    };

    let str = toml::to_string::<AgentConfig>(&agent_config)?;
    fs::write(file, str)?;
    Ok(())
}
