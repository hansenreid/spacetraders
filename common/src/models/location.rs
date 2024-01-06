use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use tabled::Tabled;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct Location {
    pub sector: String,
    pub system: String,
    pub waypoint: String,
}

impl Location {
    pub fn parse(input: String) -> Self {
        match Self::from_str(input.as_str()) {
            Ok(location) => location,
            Err(e) => {
                println!("Failed to parse location {}: {:?}", input, e);
                panic!()
            }
        }
    }

    pub fn system_ident(&self) -> String {
        format!("{}-{}", self.sector, self.system)
    }

    pub fn waypoint_ident(&self) -> String {
        self.to_string()
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.sector, self.system, self.waypoint)
    }
}

impl FromStr for Location {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("-").collect::<Vec<&str>>().as_slice() {
            [sector, system, waypoint] => Ok(Self {
                sector: sector.to_string(),
                system: system.to_string(),
                waypoint: waypoint.to_string(),
            }),

            _ => Err(eyre::eyre!("Failed to parse location")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub struct Waypoint {
    pub location: Location,
    pub waypoint_type: WaypointType,
    pub x: i32,
    pub y: i32,
}

impl From<openapi::models::Waypoint> for Waypoint {
    fn from(value: openapi::models::Waypoint) -> Self {
        Self {
            location: Location::parse(value.symbol),
            waypoint_type: value.r#type.into(),
            x: value.x,
            y: value.y,
        }
    }
}

// impl From<entities::waypoint::Model> for Waypoint {
//     fn from(value: entities::waypoint::Model) -> Self {
//         Self {
//             location: Location::parse(value.location.clone()),
//             waypoint_type: WaypointType::parse(value.r#type.clone()),
//             x: value.x,
//             y: value.y,
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Tabled)]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Asteroid,
    EngineeredAsteroid,
    AsteroidBase,
    Nebula,
    DebrisField,
    GravityWell,
    ArtificialGravityWell,
    FuelStation,
}

impl WaypointType {
    pub fn parse(input: String) -> Self {
        match Self::from_str(input.as_str()) {
            Ok(waypoint_type) => waypoint_type,
            Err(e) => {
                println!("Failed to parse waypoint-type {}: {:?}", input, e);
                panic!()
            }
        }
    }
}

impl FromStr for WaypointType {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PLANET" => Ok(Self::Planet),
            "GAS_GIANT" => Ok(Self::GasGiant),
            "MOON" => Ok(Self::Moon),
            "ORBITAL_STATION" => Ok(Self::OrbitalStation),
            "JUMP_GATE" => Ok(Self::JumpGate),
            "ASTEROID_FIELD" => Ok(Self::AsteroidField),
            "ASTEROID" => Ok(Self::Asteroid),
            "ENGINEERED_ASTEROID" => Ok(Self::EngineeredAsteroid),
            "ASTEROID_BASE" => Ok(Self::AsteroidBase),
            "NEBULA" => Ok(Self::Nebula),
            "DEBRIS_FIELD" => Ok(Self::DebrisField),
            "GRAVITY_WELL" => Ok(Self::GravityWell),
            "ARTIFICIAL_GRAVIY_WELL" => Ok(Self::ArtificialGravityWell),
            "FUEL_STATION" => Ok(Self::FuelStation),
            _ => Err(eyre::eyre!("Failed to parse waypoint-type")),
        }
    }
}

impl From<openapi::models::WaypointType> for WaypointType {
    fn from(value: openapi::models::WaypointType) -> Self {
        match value {
            openapi::models::WaypointType::Planet => Self::Planet,
            openapi::models::WaypointType::GasGiant => Self::GasGiant,
            openapi::models::WaypointType::Moon => Self::Moon,
            openapi::models::WaypointType::OrbitalStation => Self::OrbitalStation,
            openapi::models::WaypointType::JumpGate => Self::JumpGate,
            openapi::models::WaypointType::AsteroidField => Self::AsteroidField,
            openapi::models::WaypointType::Asteroid => Self::Asteroid,
            openapi::models::WaypointType::EngineeredAsteroid => Self::EngineeredAsteroid,
            openapi::models::WaypointType::AsteroidBase => Self::AsteroidBase,
            openapi::models::WaypointType::Nebula => Self::Nebula,
            openapi::models::WaypointType::DebrisField => Self::DebrisField,
            openapi::models::WaypointType::GravityWell => Self::GravityWell,
            openapi::models::WaypointType::ArtificialGravityWell => Self::ArtificialGravityWell,
            openapi::models::WaypointType::FuelStation => Self::FuelStation,
        }
    }
}

impl Display for WaypointType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            WaypointType::Planet => "PLANET",
            WaypointType::GasGiant => "GAS_GIANT",
            WaypointType::Moon => "MOON",
            WaypointType::OrbitalStation => "ORBITAL_STATION",
            WaypointType::JumpGate => "JUMP_GATE",
            WaypointType::AsteroidField => "ASTEROID_FIELD",
            WaypointType::Asteroid => "ASTEROID",
            WaypointType::EngineeredAsteroid => "ENGINEERED_ASTEROID",
            WaypointType::AsteroidBase => "ASTEROID_BASE",
            WaypointType::Nebula => "NEBULA",
            WaypointType::DebrisField => "DEBRIS_FIELD",
            WaypointType::GravityWell => "GRAVITY_WELL",
            WaypointType::ArtificialGravityWell => "ARTIFICIAL_GRAVIY_WELL",
            WaypointType::FuelStation => "FUEL_STATION",
        };

        write!(f, "{}", string)
    }
}
