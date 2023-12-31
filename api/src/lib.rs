use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod register;
pub mod status;

pub struct ApiConfig {
    pub(crate) base_url: String,
}

impl ApiConfig {
    pub fn new(url: String) -> Self {
        Self { base_url: url }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    pub starting_faction: String,
    pub ship_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub terms: Terms,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: String,
    pub deadline_to_accept: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms {
    pub deadline: String,
    pub payment: Payment,
    pub deliver: Vec<Deliver>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub on_accepted: i64,
    pub on_fulfilled: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deliver {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i64,
    pub units_fulfilled: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<Trait>,
    pub is_recruiting: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
    pub symbol: String,
    pub nav: Nav,
    pub crew: Crew,
    pub fuel: Fuel,
    pub cooldown: Cooldown,
    pub frame: Frame,
    pub reactor: Reactor,
    pub engine: Engine,
    pub modules: Vec<Module>,
    pub mounts: Vec<Mount>,
    pub registration: Registration,
    pub cargo: Cargo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nav {
    pub system_symbol: String,
    pub waypoint_symbol: String,
    pub route: Route,
    pub status: String,
    pub flight_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub departure: Departure,
    pub origin: Origin,
    pub destination: Destination,
    pub arrival: String,
    pub departure_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub system_symbol: String,
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub system_symbol: String,
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Destination {
    pub symbol: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub system_symbol: String,
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crew {
    pub current: i64,
    pub capacity: i64,
    pub required: i64,
    pub rotation: String,
    pub morale: i64,
    pub wages: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fuel {
    pub current: i64,
    pub capacity: i64,
    pub consumed: Consumed,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumed {
    pub amount: i64,
    pub timestamp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub ship_symbol: String,
    pub total_seconds: i64,
    pub remaining_seconds: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub module_slots: i64,
    pub mounting_points: i64,
    pub fuel_capacity: i64,
    pub condition: i64,
    pub requirements: Requirements,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
    pub power: i64,
    pub crew: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub condition: i64,
    pub power_output: i64,
    pub requirements: Requirements2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements2 {
    pub crew: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub condition: i64,
    pub speed: i64,
    pub requirements: Requirements3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements3 {
    pub power: i64,
    pub crew: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub capacity: Option<i64>,
    pub requirements: Requirements4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements4 {
    pub crew: i64,
    pub power: i64,
    pub slots: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub strength: i64,
    pub requirements: Requirements5,
    #[serde(default)]
    pub deposits: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements5 {
    pub crew: i64,
    pub power: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub name: String,
    pub faction_symbol: String,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    pub capacity: i64,
    pub units: i64,
    pub inventory: Vec<Value>,
}
