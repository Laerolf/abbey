use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251102_095220_create_cyclic_processes_table::CyclicProcesses;

#[derive(DeriveIden)]
pub enum Sources {
    Table,
    Id,
    Name,
    CyclicProcessId,
    LastClaimAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sources::Table)
                    .if_not_exists()
                    .col(pk_auto(Sources::Id))
                    .col(string(Sources::Name))
                    .col(integer(Sources::CyclicProcessId))
                    .col(timestamp_with_time_zone_null(Sources::LastClaimAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sources-cyclic-process")
                            .from(Sources::Table, Sources::CyclicProcessId)
                            .to(CyclicProcesses::Table, CyclicProcesses::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sources::Table).to_owned())
            .await
    }
}
