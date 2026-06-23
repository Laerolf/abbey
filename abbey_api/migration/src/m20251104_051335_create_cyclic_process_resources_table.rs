use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251102_095220_create_cyclic_processes_table::CyclicProcesses,
    m20251102_101909_create_resources_table::Resources,
};

#[derive(DeriveIden)]
enum CyclicProcessResources {
    Table,
    Id,
    CreatedAt,
    LastUpdatedAt,
    CyclicProcessId,
    ResourceId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CyclicProcessResources::Table)
                    .if_not_exists()
                    .col(pk_auto(CyclicProcessResources::Id))
                    .col(timestamp_with_time_zone(CyclicProcessResources::CreatedAt))
                    .col(timestamp_with_time_zone_null(
                        CyclicProcessResources::LastUpdatedAt,
                    ))
                    .col(integer(CyclicProcessResources::CyclicProcessId))
                    .col(integer(CyclicProcessResources::ResourceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cyclic-process-resources-cyclic-process")
                            .from(
                                CyclicProcessResources::Table,
                                CyclicProcessResources::CyclicProcessId,
                            )
                            .to(CyclicProcesses::Table, CyclicProcesses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-cyclic-process-resources-resource")
                            .from(
                                CyclicProcessResources::Table,
                                CyclicProcessResources::ResourceId,
                            )
                            .to(Resources::Table, Resources::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CyclicProcessResources::Table)
                    .to_owned(),
            )
            .await
    }
}
