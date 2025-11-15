use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum CyclicProcesses {
    Table,
    Id,
    Status,
    StartedAt,
    PausedAt,
    CycleInterval,
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
                    .table(CyclicProcesses::Table)
                    .if_not_exists()
                    .col(pk_auto(CyclicProcesses::Id))
                    .col(string(CyclicProcesses::Status))
                    .col(timestamp_with_time_zone_null(CyclicProcesses::StartedAt))
                    .col(timestamp_with_time_zone_null(CyclicProcesses::PausedAt))
                    .col(integer(CyclicProcesses::CycleInterval))
                    .col(integer(CyclicProcesses::Elapsed))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CyclicProcesses::Table).to_owned())
            .await
    }
}
