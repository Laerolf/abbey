use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251102_095220_create_tasks_table::Tasks, m20251102_101909_create_resources_table::Resources,
};

#[derive(DeriveIden)]
enum TaskOutputResources {
    Table,
    Id,
    CreatedAt,
    LastUpdatedAt,
    TaskId,
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
                    .table(TaskOutputResources::Table)
                    .if_not_exists()
                    .col(pk_auto(TaskOutputResources::Id))
                    .col(timestamp_with_time_zone(TaskOutputResources::CreatedAt))
                    .col(timestamp_with_time_zone_null(
                        TaskOutputResources::LastUpdatedAt,
                    ))
                    .col(integer(TaskOutputResources::TaskId))
                    .col(integer(TaskOutputResources::ResourceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-output-resources-task")
                            .from(TaskOutputResources::Table, TaskOutputResources::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-output-resources-resource")
                            .from(TaskOutputResources::Table, TaskOutputResources::ResourceId)
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
            .drop_table(Table::drop().table(TaskOutputResources::Table).to_owned())
            .await
    }
}
