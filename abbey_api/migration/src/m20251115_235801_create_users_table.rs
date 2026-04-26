use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Email,
    Password,
    Status,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string_uniq(Users::Email))
                    .col(string(Users::Password))
                    .col(string(Users::Status))
                    .index(
                        Index::create()
                            .name("ui-users-email")
                            .table(Users::Table)
                            .col(Users::Email)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}
