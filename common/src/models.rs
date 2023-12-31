use core::panic;
use std::fmt::Display;
use std::str::FromStr;

use openapi::models::FactionSymbol as ApiFaction;
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

pub enum FactionSymbol {
    Cosmic,
    Void,
    Galactic,
    Quantum,
    Dominion,
    Astro,
    Corsairs,
    Obsidian,
    Aegis,
    United,
    Solitary,
    Cobalt,
    Omega,
    Echo,
    Lords,
    Cult,
    Ancients,
    Shadow,
    Ethereal,
}

impl FactionSymbol {
    pub fn to_vec() -> Vec<&'static str> {
        vec![
            "COSMIC", "VOID", "GALACTIC", "QUANTUM", "DOMINION", "ASTRO", "CORSAIRS", "OBSIDIAN",
            "AEGIS", "UNITED", "SOLITARY", "COBALT", "OMEGA", "ECHO", "LORDS", "CULT", "ANCIENTS",
            "SHADOW", "ETHEREAL",
        ]
    }
}

impl Into<openapi::models::FactionSymbol> for FactionSymbol {
    fn into(self) -> openapi::models::FactionSymbol {
        match self {
            Self::Cosmic => ApiFaction::Cosmic,
            Self::Void => ApiFaction::Void,
            Self::Galactic => ApiFaction::Galactic,
            Self::Quantum => ApiFaction::Quantum,
            Self::Dominion => ApiFaction::Dominion,
            Self::Astro => ApiFaction::Astro,
            Self::Corsairs => ApiFaction::Corsairs,
            Self::Obsidian => ApiFaction::Obsidian,
            Self::Aegis => ApiFaction::Aegis,
            Self::United => ApiFaction::United,
            Self::Solitary => ApiFaction::Solitary,
            Self::Cobalt => ApiFaction::Cobalt,
            Self::Omega => ApiFaction::Omega,
            Self::Echo => ApiFaction::Echo,
            Self::Lords => ApiFaction::Lords,
            Self::Cult => ApiFaction::Cult,
            Self::Ancients => ApiFaction::Ancients,
            Self::Shadow => ApiFaction::Shadow,
            Self::Ethereal => ApiFaction::Ethereal,
        }
    }
}

impl FromStr for FactionSymbol {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "COSMIC" => Ok(Self::Cosmic),
            "VOID" => Ok(Self::Void),
            "GALACTIC" => Ok(Self::Galactic),
            "QUANTUM" => Ok(Self::Quantum),
            "DOMINION" => Ok(Self::Dominion),
            "ASTRO" => Ok(Self::Astro),
            "CORSAIRS" => Ok(Self::Corsairs),
            "OBSIDIAN" => Ok(Self::Obsidian),
            "AEGIS" => Ok(Self::Aegis),
            "UNITED" => Ok(Self::United),
            "SOLITARY" => Ok(Self::Solitary),
            "COBALT" => Ok(Self::Cobalt),
            "OMEGA" => Ok(Self::Omega),
            "ECHO" => Ok(Self::Echo),
            "LORDS" => Ok(Self::Lords),
            "CULT" => Ok(Self::Cult),
            "ANCIENTS" => Ok(Self::Ancients),
            "SHADOW" => Ok(Self::Shadow),
            "ETHEREAL" => Ok(Self::Ethereal),
            _ => Err(eyre::eyre!("Unkown faction")),
        }
    }
}

impl ToString for FactionSymbol {
    fn to_string(&self) -> String {
        match self {
            Self::Cosmic => String::from("COSMIC"),
            Self::Void => String::from("VOID"),
            Self::Galactic => String::from("GALACTIC"),
            Self::Quantum => String::from("QUANTUM"),
            Self::Dominion => String::from("DOMINION"),
            Self::Astro => String::from("ASTRO"),
            Self::Corsairs => String::from("CORSAIRS"),
            Self::Obsidian => String::from("OBSIDIAN"),
            Self::Aegis => String::from("AEGIS"),
            Self::United => String::from("UNITED"),
            Self::Solitary => String::from("SOLITARY"),
            Self::Cobalt => String::from("COBALT"),
            Self::Omega => String::from("OMEGA"),
            Self::Echo => String::from("ECHO"),
            Self::Lords => String::from("LORDS"),
            Self::Cult => String::from("CULT"),
            Self::Ancients => String::from("ANCIENTS"),
            Self::Shadow => String::from("SHADOW"),
            Self::Ethereal => String::from("ETHEREAL"),
        }
    }
}
