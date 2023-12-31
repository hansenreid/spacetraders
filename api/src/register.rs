use crate::{Agent, ApiConfig, Contract, Faction, Ship};

use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    faction: String,
    symbol: String,
}

impl RegisterRequest {
    pub fn new(faction: String, symbol: String) -> Self {
        Self { faction, symbol }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    pub data: Data,
}

pub async fn register(config: ApiConfig, req: RegisterRequest) -> Result<RegisterResponse> {
    let client = reqwest::Client::builder().build()?;
    let url = format!("{}/register", config.base_url);

    let res = client
        .post(url)
        .json(&req)
        .send()
        .await
        .wrap_err("Failed to call register endpoint")?
        .error_for_status()
        .wrap_err("Register endpoint returned error response")?
        .json::<RegisterResponse>()
        .await
        .wrap_err("Failed to deserialize register response")?;

    Ok(res)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub token: String,
    pub agent: Agent,
    pub contract: Contract,
    pub faction: Faction,
    pub ship: Ship,
}
