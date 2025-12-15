use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Resources {
    Table,
    Id,
    Name,
    Category,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Resources::Table)
                    .if_not_exists()
                    .col(pk_auto(Resources::Id))
                    .col(string_uniq(Resources::Name))
                    .col(string(Resources::Category))
                    .index(
                        Index::create()
                            .name("ui-resources-name")
                            .table(Resources::Table)
                            .col(Resources::Name)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Resources::Table).to_owned())
            .await
    }
}
