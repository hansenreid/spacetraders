use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::location::Location;
use super::FactionSymbol;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct Agent {
    #[tabled(display_with = "super::display_option")]
    pub account_id: Option<String>,

    pub symbol: String,
    pub headquarters: Location,
    pub credits: i64,
    pub starting_faction: FactionSymbol,
    pub ship_count: i32,
}

impl From<Box<openapi::models::Agent>> for Agent {
    fn from(value: Box<openapi::models::Agent>) -> Self {
        Self {
            account_id: value.account_id,
            symbol: value.symbol,
            headquarters: Location::parse(value.headquarters),
            credits: value.credits,
            starting_faction: FactionSymbol::parse(value.starting_faction),
            ship_count: value.ship_count,
        }
    }
}
