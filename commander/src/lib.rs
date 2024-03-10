use std::{str::FromStr, time::Duration};

use agent::Agent;
use eyre::Result;
use navigation::{location::Location, waypoint::Waypoint, waypoint_type::WaypointType};
use sqlx::SqlitePool;
use tokio::time;

const DATABASE_URL: &str = "sqlite://spacetraders-db.sqlite?mode=rwc";

pub async fn run() -> Result<()> {
    let pool = SqlitePool::connect(DATABASE_URL).await?;
    let agent = Agent::fetch_or_create(&pool).await?;
    tracing::info!("agent: {:#?}", agent);

    let waypoint = Waypoint {
        location: Location::from_str("X1-C46-A1")?,
        waypoint_type: WaypointType::Moon,
        traits: vec![],
        x: 0i32,
        y: 32i32,
    };

    waypoint.save(&pool).await;

    let mut interval = time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        println!("Tick");
    }
}
