use core::panic;
use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tabled::Tabled;
use time::format_description::well_known::Iso8601;
use time::PrimitiveDateTime;

fn display_option<T: Display>(o: &Option<T>) -> String {
    match o {
        Some(t) => format!("{}", t),
        None => format!("{}", "N/A"),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct Agent {
    #[tabled(display_with = "display_option")]
    pub account_id: Option<String>,

    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    pub starting_faction: String,
    pub ship_count: i32,
}

impl From<Box<openapi::models::Agent>> for Agent {
    fn from(value: Box<openapi::models::Agent>) -> Self {
        Self {
            account_id: value.account_id,
            symbol: value.symbol,
            headquarters: value.headquarters,
            credits: value.credits,
            starting_faction: value.starting_faction,
            ship_count: value.ship_count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct Contract {
    pub id: String,
    pub faction_symbol: String,

    #[tabled(skip)]
    pub contract_type: ContractType,

    // pub terms: Box<crate::models::ContractTerms>,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: PrimitiveDateTime,

    #[tabled(display_with = "display_option")]
    pub deadline_to_accept: Option<PrimitiveDateTime>,
}

impl From<Box<openapi::models::Contract>> for Contract {
    fn from(value: Box<openapi::models::Contract>) -> Self {
        let expiration = PrimitiveDateTime::parse(value.expiration.as_str(), &Iso8601::DEFAULT);
        let expiration = match expiration {
            Ok(date) => date,
            Err(e) => {
                println!("Error formatting date: {:?}", e);
                panic!()
            }
        };

        let deadline_to_accept = match value.deadline_to_accept {
            Some(date) => {
                let date = PrimitiveDateTime::parse(date.as_str(), &Iso8601::DEFAULT);
                match date {
                    Ok(date) => Some(date),
                    Err(e) => {
                        println!("Error formatting date: {:?}", e);
                        panic!()
                    }
                }
            }
            None => None,
        };

        Self {
            id: value.id,
            faction_symbol: value.faction_symbol,
            accepted: value.accepted,
            fulfilled: value.fulfilled,
            expiration,
            deadline_to_accept,
            contract_type: value.r#type.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Tabled)]
pub enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

impl From<openapi::models::contract::Type> for ContractType {
    fn from(value: openapi::models::contract::Type) -> Self {
        match value {
            openapi::models::contract::Type::Procurement => ContractType::Procurement,
            openapi::models::contract::Type::Transport => ContractType::Transport,
            openapi::models::contract::Type::Shuttle => ContractType::Shuttle,
        }
    }
}
