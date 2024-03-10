use chrono::NaiveDateTime;
use eyre::Result;
use openapi::{
    apis::{
        self,
        configuration::Configuration,
        default_api::{self, RegisterError},
    },
    models::{register_201_response_data::Register201ResponseData, FactionSymbol, RegisterRequest},
};
use serde::{Deserialize, Serialize};
use sqlx::{Error::RowNotFound, SqlitePool};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("database error")]
    DBError(#[from] sqlx::Error),

    #[error("error registering new agent")]
    RegistrationError(#[from] apis::Error<RegisterError>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    pub account_id: Option<String>,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    pub starting_faction: String,
    pub ship_count: i64,
    pub token: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Agent {
    pub async fn fetch_or_create(pool: &SqlitePool) -> Result<Agent, AgentError> {
        let agent = fetch_agent(pool).await;

        match agent {
            Ok(agent) => Ok(agent),
            Err(RowNotFound) => create_agent(&pool).await,
            Err(err) => Err(AgentError::DBError(err)),
        }
    }
}

async fn fetch_agent(pool: &SqlitePool) -> Result<Agent, sqlx::Error> {
    sqlx::query_file_as!(Agent, "src/fetch_one.sql")
        .fetch_one(pool)
        .await
}

async fn create_agent(pool: &SqlitePool) -> Result<Agent, AgentError> {
    let res = register_agent("Natingar3".into(), FactionSymbol::Cosmic)
        .await
        .map_err(|err| AgentError::RegistrationError(err))?;
    let agent = res.agent;

    sqlx::query_file!(
        "src/insert_one.sql",
        agent.account_id,
        agent.symbol,
        agent.headquarters,
        agent.credits,
        agent.starting_faction,
        agent.ship_count,
        res.token,
    )
    .execute(pool)
    .await?;

    fetch_agent(pool)
        .await
        .map_err(|err| AgentError::DBError(err))
}

async fn register_agent(
    symbol: String,
    faction: FactionSymbol,
) -> Result<Box<Register201ResponseData>, apis::Error<RegisterError>> {
    let conf = Configuration::new();
    let req = RegisterRequest::new(faction.into(), symbol);
    let res = default_api::register(&conf, Some(req)).await?;

    Ok(res.data)
}

impl From<Box<Register201ResponseData>> for Agent {
    fn from(value: Box<Register201ResponseData>) -> Self {
        let agent = value.agent;
        Self {
            account_id: agent.account_id,
            symbol: agent.symbol,
            headquarters: agent.headquarters,
            credits: agent.credits,
            starting_faction: agent.starting_faction,
            ship_count: agent.ship_count as i64,
            token: value.token,
            created_at: None,
            updated_at: None,
        }
    }
}
