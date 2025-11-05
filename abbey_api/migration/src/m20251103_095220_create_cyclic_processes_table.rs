use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum CyclicProcess {
    Table,
    Id,
    Status,
    StartedAt,
    PausedAt,
    CycleInterval,
    Elapsed,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CyclicProcess::Table)
                    .if_not_exists()
                    .col(pk_auto(CyclicProcess::Id))
                    .col(string(CyclicProcess::Status))
                    .col(timestamp_with_time_zone_null(CyclicProcess::StartedAt))
                    .col(timestamp_with_time_zone_null(CyclicProcess::PausedAt))
                    .col(integer(CyclicProcess::CycleInterval))
                    .col(integer(CyclicProcess::Elapsed))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CyclicProcess::Table).to_owned())
            .await
    }
}
