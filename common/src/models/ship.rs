use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use tabled::Tabled;
use time::format_description::well_known::Iso8601;
use time::{Duration, OffsetDateTime};

use super::{location::WaypointType, FactionSymbol, Location};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
// TODO: Finish implementation
pub struct Ship {
    pub symbol: String,
    #[tabled(inline)]
    pub registration: ShipRegistration,
    #[tabled(inline)]
    pub nav: ShipNav,
    #[tabled(inline)]
    pub cargo: ShipCargo,
    #[tabled(inline)]
    pub fuel: ShipFuel,
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
            cargo: ShipCargo::from(value.cargo),
            fuel: ShipFuel::from(value.fuel),
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
    pub flight_mode: ShipNavFlightMode,
    #[tabled(inline)]
    pub route: ShipNavRoute,
}

impl From<Box<openapi::models::ShipNav>> for ShipNav {
    fn from(value: Box<openapi::models::ShipNav>) -> Self {
        Self {
            location: Location::parse(value.waypoint_symbol),
            status: value.status.into(),
            flight_mode: value.flight_mode.into(),
            route: value.route.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled, JsonSchema)]
pub enum ShipNavFlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}

impl From<openapi::models::ShipNavFlightMode> for ShipNavFlightMode {
    fn from(value: openapi::models::ShipNavFlightMode) -> Self {
        match value {
            openapi::models::ShipNavFlightMode::Drift => Self::Drift,
            openapi::models::ShipNavFlightMode::Stealth => Self::Stealth,
            openapi::models::ShipNavFlightMode::Cruise => Self::Cruise,
            openapi::models::ShipNavFlightMode::Burn => Self::Burn,
        }
    }
}

impl Display for ShipNavFlightMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ShipNavFlightMode::Drift => "DRIFT",
            ShipNavFlightMode::Stealth => "STEALTH",
            ShipNavFlightMode::Cruise => "CRUISE",
            ShipNavFlightMode::Burn => "BURN",
        };

        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct ShipNavRoute {
    #[tabled(display_with = "super::display_option")]
    pub time_to_arrival: Option<Duration>,

    #[tabled(inline)]
    pub destination: ShipNavRouteWayPoint,
    #[tabled(skip)]
    pub origin: ShipNavRouteWayPoint,
    #[tabled(skip)]
    pub arrival: OffsetDateTime,
    #[tabled(skip)]
    pub departure_time: OffsetDateTime,
}

impl From<Box<openapi::models::ShipNavRoute>> for ShipNavRoute {
    fn from(value: Box<openapi::models::ShipNavRoute>) -> Self {
        let arrival = OffsetDateTime::parse(value.arrival.as_str(), &Iso8601::DEFAULT);
        let arrival = match arrival {
            Ok(date) => date,
            Err(e) => {
                println!("Error formatting date: {:?}", e);
                panic!()
            }
        };

        let departure_time =
            OffsetDateTime::parse(value.departure_time.as_str(), &Iso8601::DEFAULT);
        let departure_time = match departure_time {
            Ok(date) => date,
            Err(e) => {
                println!("Error formatting date: {:?}", e);
                panic!()
            }
        };

        let now = OffsetDateTime::now_utc();
        let diff = arrival - now;
        let time_to_arrival = match Duration::is_negative(diff) {
            true => None,
            false => Some(diff),
        };

        Self {
            time_to_arrival,
            arrival,
            departure_time,
            destination: value.destination.into(),
            origin: value.origin.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct ShipNavRouteWayPoint {
    #[tabled(rename = "Destination")]
    pub location: Location,

    #[tabled(rename = "Destination Type")]
    pub waypoint_type: WaypointType,

    #[tabled(skip)]
    pub x: i32,
    #[tabled(skip)]
    pub y: i32,
}

impl From<Box<openapi::models::ShipNavRouteWaypoint>> for ShipNavRouteWayPoint {
    fn from(value: Box<openapi::models::ShipNavRouteWaypoint>) -> Self {
        let location = Location::parse(value.symbol);

        Self {
            location,
            waypoint_type: value.r#type.into(),
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled, JsonSchema)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled, JsonSchema)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct ShipFuel {
    #[tabled(rename = "Fuel Level")]
    pub current: i32,
    #[tabled(rename = "Fuel Capacity")]
    pub capacity: i32,
}

impl From<Box<openapi::models::ShipFuel>> for ShipFuel {
    fn from(value: Box<openapi::models::ShipFuel>) -> Self {
        Self {
            current: value.current,
            capacity: value.capacity,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct ShipCargo {
    #[tabled(rename = "Cargo Level")]
    pub current: i32,
    #[tabled(rename = "Cargo Capacity")]
    pub capacity: i32,
}

impl From<Box<openapi::models::ShipCargo>> for ShipCargo {
    fn from(value: Box<openapi::models::ShipCargo>) -> Self {
        Self {
            current: value.units,
            capacity: value.capacity,
        }
    }
}
