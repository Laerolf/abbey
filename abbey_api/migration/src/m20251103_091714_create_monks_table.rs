use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251102_095220_create_cyclic_processes_table::CyclicProcesses;

#[derive(DeriveIden)]
pub enum Monks {
    Table,
    Id,
    CreatedAt,
    LastUpdatedAt,
    Name,
    AssignedCyclicProcessId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Monks::Table)
                    .if_not_exists()
                    .col(pk_auto(Monks::Id))
                    .col(timestamp_with_time_zone(Monks::CreatedAt))
                    .col(timestamp_with_time_zone_null(Monks::LastUpdatedAt))
                    .col(string(Monks::Name))
                    .col(integer_null(Monks::AssignedCyclicProcessId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monks-cyclic-process")
                            .from(Monks::Table, Monks::AssignedCyclicProcessId)
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
            .drop_table(Table::drop().table(Monks::Table).to_owned())
            .await
    }
}
