use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251102_095220_create_cyclic_processes_table::CyclicProcesses;

#[derive(DeriveIden)]
pub enum Players {
    Table,
    Id,
    CreatedAt,
    LastUpdatedAt,
    AssignedProcessId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Players::Table)
                    .if_not_exists()
                    .col(pk_auto(Players::Id))
                    .col(timestamp_with_time_zone(Players::CreatedAt))
                    .col(timestamp_with_time_zone_null(Players::LastUpdatedAt))
                    .col(integer_null(Players::AssignedProcessId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-players-cyclic-process")
                            .from(Players::Table, Players::AssignedProcessId)
                            .to(CyclicProcesses::Table, CyclicProcesses::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Players::Table).to_owned())
            .await
    }
}
