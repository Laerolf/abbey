use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Tasks {
    Table,
    Id,
    Status,
    StartedAt,
    PausedAt,
    Duration,
    Elapsed,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(pk_auto(Tasks::Id))
                    .col(string(Tasks::Status))
                    .col(timestamp_with_time_zone_null(Tasks::StartedAt))
                    .col(timestamp_with_time_zone_null(Tasks::PausedAt))
                    .col(integer(Tasks::Duration))
                    .col(integer(Tasks::Elapsed))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}
