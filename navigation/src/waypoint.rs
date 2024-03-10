use openapi::apis::{self, systems_api::GetSystemWaypointsError};
use sqlx::SqlitePool;
use thiserror::Error;

use crate::{location::Location, waypoint_trait::WaypointTrait, waypoint_type::WaypointType};

#[derive(Error, Debug)]
pub enum WaypointError {
    #[error("database error")]
    DBError(#[from] sqlx::Error),

    #[error("error retrieving waypoints")]
    ApiError(#[from] apis::Error<GetSystemWaypointsError>),
}

#[derive(Debug)]
pub struct Waypoint {
    pub location: Location,
    pub waypoint_type: WaypointType,
    pub traits: Vec<WaypointTrait>,
    pub x: i32,
    pub y: i32,
}

impl Waypoint {
    pub async fn save(self, pool: &SqlitePool) {
        let location = self.location.to_string();
        let r#type = self.waypoint_type.to_string();
        let traits = self
            .traits
            .iter()
            .map(WaypointTrait::to_string)
            .collect::<Vec<String>>()
            .join(",");

        let _ = sqlx::query_file!(
            "src/insert_waypoint.sql",
            location,
            r#type,
            traits,
            self.x,
            self.y
        )
        .execute(pool)
        .await;
    }
}

impl From<openapi::models::Waypoint> for Waypoint {
    fn from(value: openapi::models::Waypoint) -> Self {
        let traits = value
            .traits
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<WaypointTrait>>();
        Self {
            location: Location::parse(value.symbol),
            waypoint_type: value.r#type.into(),
            traits,
            x: value.x,
            y: value.y,
        }
    }
}

pub async fn initialize_system_waypoints(system: String) {}
