use eyre::{Context, Ok, Result};
use sea_orm::*;

pub mod entities;
use entities::{prelude::*, *};

use crate::models::WaypointTraitSymbol::Marketplace;
use crate::models::WaypointType;

const DATABASE_URL: &str = "sqlite://spacetraders-db.sqlite?mode=rwc";

pub async fn connect() -> Result<DatabaseConnection> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

pub async fn insert_waypoints(
    db: &DatabaseConnection,
    waypoints: Vec<super::models::Waypoint>,
) -> Result<()> {
    let to_insert = waypoints
        .into_iter()
        .map(|w| {
            let traits = w
                .traits
                .into_iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(",");

            waypoint::ActiveModel {
                traits: ActiveValue::Set(traits),
                location: ActiveValue::Set(w.location.waypoint_ident()),
                r#type: ActiveValue::Set(w.waypoint_type.to_string()),
                x: ActiveValue::Set(w.x),
                y: ActiveValue::Set(w.y),
            }
        })
        .collect::<Vec<waypoint::ActiveModel>>();

    let res = Waypoint::insert_many(to_insert).exec(db).await?;
    println!("Result: {:?}", res);

    Ok(())
}

pub async fn get_marketplace_waypoints(
    db: &DatabaseConnection,
) -> Result<Vec<crate::models::Waypoint>> {
    Ok(Waypoint::find()
        .filter(waypoint::Column::Traits.contains(Marketplace.to_string()))
        .all(db)
        .await
        .wrap_err("Failed to query waypoints")?
        .into_iter()
        .map(|w| crate::models::Waypoint {
            location: crate::models::Location::parse(w.location),
            waypoint_type: WaypointType::parse(w.r#type),
            traits: crate::models::WaypointTrait::parse_vec(w.traits),
            x: w.x,
            y: w.y,
        })
        .collect::<Vec<crate::models::Waypoint>>())
}
