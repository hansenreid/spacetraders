use std::time::Duration;

use agent::Agent;
use eyre::Result;
use sqlx::SqlitePool;
use tokio::time;

const DATABASE_URL: &str = "sqlite://spacetraders-db.sqlite?mode=rwc";

pub async fn run() -> Result<()> {
    let pool = SqlitePool::connect(DATABASE_URL).await?;
    let agent = Agent::fetch_or_create(&pool).await?;
    tracing::info!("agent: {:#?}", agent);

    let mut interval = time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        println!("Tick");
    }
}
