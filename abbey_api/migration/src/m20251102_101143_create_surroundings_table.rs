use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Surroundings {
    Table,
    Id,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Surroundings::Table)
                    .if_not_exists()
                    .col(pk_auto(Surroundings::Id))
                    .col(timestamp_with_time_zone(Surroundings::CreatedAt))
                    .col(timestamp_with_time_zone_null(Surroundings::LastUpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Surroundings::Table).to_owned())
            .await
    }
}
