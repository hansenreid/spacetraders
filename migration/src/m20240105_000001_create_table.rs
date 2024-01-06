use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Waypoint::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Waypoint::Location)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Waypoint::Type).string().not_null())
                    .col(ColumnDef::new(Waypoint::X).integer().not_null())
                    .col(ColumnDef::new(Waypoint::Y).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Waypoint::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Waypoint {
    Table,
    Location,
    Type,
    X,
    Y,
}
