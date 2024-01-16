use std::path::Path;
use std::str::FromStr;
use std::{fs, vec};

use clap::Parser;
use eyre::{Ok, Result};
use inquire::{Select, Text};
use openapi::apis;
use serde::{Deserialize, Serialize};
use tabled::Table;

#[derive(Debug, Parser)]
enum Command {
    GetAgent,
    Step,
    GetShips,
    InitManager,
    Status,
    RefreshWaypoints,
    CrdGen,
    Run,
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
    tracing_subscriber::fmt::init();

    let config: Config = clap::Parser::parse();
    let agent_config = get_conf()?;
    let conf = apis::configuration::Configuration::new();

    match config.command {
        Some(Command::Run) => operator::run().await?,

        Some(Command::CrdGen) => operator::crdgen()?,

        Some(Command::Status) => {
            let openapi_response = apis::default_api::get_status(&conf).await?;
            println!("{:#?}", openapi_response);
        }

        Some(Command::InitManager) => {
            let symbol = Text::new("What is your agent symbol?").prompt()?;

            let faction_answer =
                Select::new("Select a faction:", common::models::FactionSymbol::to_vec())
                    .prompt()?;
            let faction = common::models::FactionSymbol::from_str(faction_answer)?.into();
            operator::init_manager(symbol, faction).await?
        }

        Some(Command::GetAgent) => match agent_config {
            Some(agent_config) => {
                let api_config = get_authenticated_config(agent_config.token);
                let res = apis::agents_api::get_my_agent(&api_config).await?;

                let agent = common::models::Agent::from(res.data);
                let agent_table = vec![agent];
                let agent_table = Table::new(agent_table).to_string();
                println!("\n{}\n", agent_table);
            }

            None => println!("No agent found. Please register first"),
        },

        Some(Command::GetShips) => match agent_config {
            Some(agent_config) => {
                let api_config = get_authenticated_config(agent_config.token);
                let page = 1;
                let limit = 10;
                let res =
                    apis::fleet_api::get_my_ships(&api_config, Some(page), Some(limit)).await?;

                let mut ships = res
                    .data
                    .into_iter()
                    .map(|ship| common::models::Ship::from(ship))
                    .collect::<Vec<common::models::Ship>>();

                let total = res.meta.total;
                let num_pages = (total as f32 / limit as f32).ceil() as i32;

                for n in (page + 1)..=num_pages {
                    let res =
                        apis::fleet_api::get_my_ships(&api_config, Some(n), Some(limit)).await?;
                    for s in res.data {
                        let ship = common::models::Ship::from(s);

                        ships.push(ship)
                    }
                }

                let ship_table = Table::new(ships).to_string();
                println!("\n{}\n", ship_table)
            }

            None => println!("No agent found. Please register first"),
        },

        Some(Command::Step) => match agent_config {
            Some(agent_config) => {
                let api_config = get_authenticated_config(agent_config.token);
                let db = common::repository::connect().await?;
                let dest = common::models::Location::from_str("X1-GQ23-H45")?;
                let mut machine = common::machines::TravelMachineWrapper::new(
                    api_config,
                    &db,
                    dest,
                    "NATINGAR2-3",
                )
                .await?;

                loop {
                    match machine {
                        common::machines::TravelMachineWrapper::TravelComplete => {
                            println!("Travel has been completed!");
                            break;
                        }
                        _ => {
                            machine = machine.step().await?;
                        }
                    }
                }
            }
            None => println!("No agent found. Please register first"),
        },

        Some(Command::RefreshWaypoints) => match agent_config {
            Some(agent_config) => {
                let api_config = get_authenticated_config(agent_config.token);
                let loc = common::models::Location::from_str("X1-GQ23-H45")?;
                let page = 1;
                let limit = 20;

                let res = apis::systems_api::get_system_waypoints(
                    &api_config,
                    loc.system_ident().as_str(),
                    Some(page),
                    Some(limit),
                    None,
                    None,
                )
                .await?;

                let mut waypoints = res
                    .data
                    .into_iter()
                    .map(|waypoint| common::models::Waypoint::from(waypoint))
                    .collect::<Vec<common::models::Waypoint>>();

                let total = res.meta.total;
                let num_pages = (total as f32 / limit as f32).ceil() as i32;

                for n in (page + 1)..=num_pages {
                    let res = apis::systems_api::get_system_waypoints(
                        &api_config,
                        loc.system_ident().as_str(),
                        Some(n),
                        Some(limit),
                        None,
                        None,
                    )
                    .await?;

                    for w in res.data {
                        let waypoint = common::models::Waypoint::from(w);

                        waypoints.push(waypoint)
                    }
                }

                println!("Total waypoints: {}", res.meta.total);

                let db = common::repository::connect().await?;
                common::repository::insert_waypoints(&db, waypoints).await?;
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

fn get_authenticated_config(bearer_token: String) -> apis::configuration::Configuration {
    apis::configuration::Configuration {
        bearer_access_token: Some(bearer_token),
        ..Default::default()
    }
}
