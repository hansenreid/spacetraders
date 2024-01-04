use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use tabled::Tabled;

use super::{FactionSymbol, Location};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
// TODO: Finish implementation
pub struct Ship {
    pub symbol: String,
    #[tabled(inline)]
    pub registration: ShipRegistration,
    #[tabled(inline)]
    pub nav: ShipNav,
}

impl From<Box<openapi::models::Ship>> for Ship {
    fn from(value: Box<openapi::models::Ship>) -> Self {
        Ship::from(*value)
    }
}

impl From<openapi::models::Ship> for Ship {
    fn from(value: openapi::models::Ship) -> Self {
        Self {
            symbol: value.symbol,
            registration: ShipRegistration::from(value.registration),
            nav: ShipNav::from(value.nav),
        }
    }
}

impl Ship {
    pub fn update_nav(&mut self, nav: ShipNav) {
        self.nav = nav;
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct ShipRegistration {
    pub name: String,
    pub faction_symbol: FactionSymbol,
    pub role: ShipRole,
}

impl From<Box<openapi::models::ShipRegistration>> for ShipRegistration {
    fn from(value: Box<openapi::models::ShipRegistration>) -> Self {
        Self {
            name: value.name,
            faction_symbol: FactionSymbol::parse(value.faction_symbol),
            role: ShipRole::from(value.role),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
// TODO: Finish implementation
pub struct ShipNav {
    pub location: Location,
    pub status: ShipNavStatus,
}

impl From<Box<openapi::models::ShipNav>> for ShipNav {
    fn from(value: Box<openapi::models::ShipNav>) -> Self {
        Self {
            location: Location::parse(value.waypoint_symbol),
            status: value.status.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub enum ShipRole {
    Fabricator,
    Harvester,
    Hauler,
    Interceptor,
    Excavator,
    Transport,
    Repair,
    Surveyor,
    Command,
    Carrier,
    Patrol,
    Satellite,
    Explorer,
    Refinery,
}

impl From<openapi::models::ShipRole> for ShipRole {
    fn from(value: openapi::models::ShipRole) -> Self {
        match value {
            openapi::models::ShipRole::Fabricator => Self::Fabricator,
            openapi::models::ShipRole::Harvester => Self::Harvester,
            openapi::models::ShipRole::Hauler => Self::Hauler,
            openapi::models::ShipRole::Interceptor => Self::Interceptor,
            openapi::models::ShipRole::Excavator => Self::Excavator,
            openapi::models::ShipRole::Transport => Self::Transport,
            openapi::models::ShipRole::Repair => Self::Repair,
            openapi::models::ShipRole::Surveyor => Self::Surveyor,
            openapi::models::ShipRole::Command => Self::Command,
            openapi::models::ShipRole::Carrier => Self::Carrier,
            openapi::models::ShipRole::Patrol => Self::Patrol,
            openapi::models::ShipRole::Satellite => Self::Satellite,
            openapi::models::ShipRole::Explorer => Self::Explorer,
            openapi::models::ShipRole::Refinery => Self::Refinery,
        }
    }
}

impl Display for ShipRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string = match self {
            ShipRole::Fabricator => "FABRICATOR",
            ShipRole::Harvester => "HARVESTER",
            ShipRole::Hauler => "HAULER",
            ShipRole::Interceptor => "INTERCEPTOR",
            ShipRole::Excavator => "EXCAVATOR",
            ShipRole::Transport => "TRANSPORT",
            ShipRole::Repair => "REPAIR",
            ShipRole::Surveyor => "SURVEYOR",
            ShipRole::Command => "COMMAND",
            ShipRole::Carrier => "CARRIER",
            ShipRole::Patrol => "PATROL",
            ShipRole::Satellite => "SATELLITE",
            ShipRole::Explorer => "EXPLORER",
            ShipRole::Refinery => "REFINERY",
        };

        write!(f, "{}", string)
    }
}

impl FromStr for ShipRole {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FABRICATOR" => Ok(Self::Fabricator),
            "HARVESTER" => Ok(Self::Harvester),
            "HAULER" => Ok(Self::Hauler),
            "INTERCEPTOR" => Ok(Self::Interceptor),
            "EXCAVATOR" => Ok(Self::Excavator),
            "TRANSPORT" => Ok(Self::Transport),
            "REPAIR" => Ok(Self::Repair),
            "SURVEYOR" => Ok(Self::Surveyor),
            "COMMAND" => Ok(Self::Command),
            "CARRIER" => Ok(Self::Carrier),
            "PATROL" => Ok(Self::Patrol),
            "SATELLITE" => Ok(Self::Satellite),
            "EXPLORER" => Ok(Self::Explorer),
            "REFINERY" => Ok(Self::Explorer),

            _ => Err(eyre::eyre!("Unkown Ship Role")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub enum ShipNavStatus {
    InTransit,
    InOrbit,
    Docked,
}

impl From<openapi::models::ShipNavStatus> for ShipNavStatus {
    fn from(value: openapi::models::ShipNavStatus) -> Self {
        match value {
            openapi::models::ShipNavStatus::InTransit => Self::InTransit,
            openapi::models::ShipNavStatus::InOrbit => Self::InOrbit,
            openapi::models::ShipNavStatus::Docked => Self::Docked,
        }
    }
}

impl Display for ShipNavStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ShipNavStatus::InTransit => "IN_TRANSIT",
            ShipNavStatus::InOrbit => "IN_ORBIT",
            ShipNavStatus::Docked => "DOCKED",
        };

        write!(f, "{}", string)
    }
}
