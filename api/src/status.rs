use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::ApiConfig;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    status: String,
    version: String,
    reset_date: String,
    stats: Stats,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    agents: i64,
    ships: i64,
    systems: i64,
    waypoints: i64,
}

pub async fn get(config: ApiConfig) -> Result<Status> {
    let status = reqwest::get(config.base_url)
        .await
        .context("Failed to call status endpoint")?
        .error_for_status()
        .context("Status endpoint returned error response")?
        .json::<Status>()
        .await
        .context("Failed to deserialize status response")?;

    Ok(status)
}
