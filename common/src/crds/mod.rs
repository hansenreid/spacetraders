use k8s_openapi::chrono::{DateTime, Utc};
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::{self, faction::FactionSymbol};

#[derive(Deserialize, CustomResource, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    kind = "Manager",
    group = "spacetraders.io",
    version = "v1",
    namespaced
)]
#[kube(status = "ManagerStatus")]
pub struct ManagerSpec {
    pub symbol: String,
    pub faction: FactionSymbol,
    pub namespace: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ManagerStatus {
    pub checksum: String,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Deserialize, CustomResource, Serialize, Clone, Debug, JsonSchema)]
#[kube(kind = "Agent", group = "spacetraders.io", version = "v1", namespaced)]
#[kube(status = "AgentStatus")]
pub struct AgentSpec {
    pub symbol: String,
    pub faction: FactionSymbol,
    pub token: Option<String>,
    pub reset_date: Option<DateTime<Utc>>,
}
#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct AgentStatus {
    pub checksum: String,
    pub ships_initialized: bool,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Deserialize, CustomResource, Serialize, Clone, Debug, JsonSchema)]
#[kube(kind = "Ship", group = "spacetraders.io", version = "v1", namespaced)]
#[kube(status = "ShipStatus")]
pub struct ShipSpec {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<models::ShipRole>,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct ShipStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<models::Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ShipNavStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_mode: Option<models::ShipNavFlightMode>,
}
