pub mod agent;
pub use self::agent::Agent;

pub mod contract;
pub use self::contract::{Contract, ContractType};

pub mod faction;
pub use self::faction::FactionSymbol;

pub mod location;
pub use self::location::{Location, Waypoint, WaypointTrait, WaypointTraitSymbol, WaypointType};

pub mod ship;
pub use self::ship::Ship;
pub use self::ship::ShipNavFlightMode;
pub use self::ship::ShipNavStatus;
pub use self::ship::ShipRole;

use std::fmt::Display;

fn display_option<T: Display>(o: &Option<T>) -> String {
    match o {
        Some(t) => format!("{}", t),
        None => format!("{}", "N/A"),
    }
}
