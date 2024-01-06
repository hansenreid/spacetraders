pub use sea_orm_migration::prelude::*;

mod m20240105_000001_create_table;

pub struct Migrator;

pub async fn migrate() {
    cli::run_cli(Migrator).await;
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240105_000001_create_table::Migration)]
    }
}
