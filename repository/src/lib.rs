use eyre::{Ok, Result};
use sea_orm::*;

pub mod entities;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "sqlite://spacetraders-db.sqlite?mode=rwc";

pub async fn connect() -> Result<DatabaseConnection> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

pub async fn insert_waypoints(
    db: &DatabaseConnection,
    waypoints: Vec<common::models::Waypoint>,
) -> Result<()> {
    let to_insert = waypoints
        .into_iter()
        .map(|w| waypoint::ActiveModel {
            location: ActiveValue::Set(w.location.waypoint_ident()),
            r#type: ActiveValue::Set(w.waypoint_type.to_string()),
            x: ActiveValue::Set(w.x),
            y: ActiveValue::Set(w.y),
        })
        .collect::<Vec<waypoint::ActiveModel>>();

    let res = Waypoint::insert_many(to_insert).exec(db).await?;
    println!("Result: {:?}", res);

    Ok(())
}
