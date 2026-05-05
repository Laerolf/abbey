use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251102_095220_create_tasks_table::Tasks, m20251102_101909_create_resources_table::Resources,
};

#[derive(DeriveIden)]
enum TaskInputResources {
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
                    .table(TaskInputResources::Table)
                    .if_not_exists()
                    .col(pk_auto(TaskInputResources::Id))
                    .col(timestamp_with_time_zone(TaskInputResources::CreatedAt))
                    .col(timestamp_with_time_zone_null(
                        TaskInputResources::LastUpdatedAt,
                    ))
                    .col(integer(TaskInputResources::TaskId))
                    .col(integer(TaskInputResources::ResourceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-input-resources-task")
                            .from(TaskInputResources::Table, TaskInputResources::TaskId)
                            .to(Tasks::Table, Tasks::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-input-resources-resource")
                            .from(TaskInputResources::Table, TaskInputResources::ResourceId)
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
            .drop_table(Table::drop().table(TaskInputResources::Table).to_owned())
            .await
    }
}
