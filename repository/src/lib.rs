use eyre::{Ok, Result};
use futures::executor::block_on;
use sea_orm::*;

pub mod entities;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "sqlite://spacetraders-db.sqlite?mode=rwc";

pub async fn connect() -> Result<DatabaseConnection> {
    let db = Database::connect(DATABASE_URL).await?;
    Ok(db)
}

pub async fn run() {
    if let Err(err) = block_on(connect()) {
        panic!("{}", err)
    }
}

pub async fn insert_waypoint(waypoint: &common::models::Waypoint) -> Result<()> {
    let entity = waypoint::ActiveModel {
        location: ActiveValue::Set(waypoint.location.waypoint_ident()),
        r#type: ActiveValue::Set(waypoint.waypoint_type.to_string()),
        x: ActiveValue::Set(waypoint.x),
        y: ActiveValue::Set(waypoint.y),
    };

    let db = connect().await?;
    Waypoint::insert(entity).exec(&db).await?;

    Ok(())
}
