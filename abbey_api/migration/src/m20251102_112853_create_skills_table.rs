use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Skills {
    Table,
    Id,
    CreatedAt,
    LastUpdatedAt,
    Name,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Skills::Table)
                    .if_not_exists()
                    .col(pk_auto(Skills::Id))
                    .col(timestamp_with_time_zone(Skills::CreatedAt))
                    .col(timestamp_with_time_zone_null(Skills::LastUpdatedAt))
                    .col(string(Skills::Name))
                    .index(
                        Index::create()
                            .name("ui-skills-name")
                            .table(Skills::Table)
                            .col(Skills::Name)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Skills::Table).to_owned())
            .await
    }
}
