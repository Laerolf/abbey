use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Monk {
    Table,
    Id,
    Name,
    AssignedProcessId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Monk::Table)
                    .if_not_exists()
                    .col(pk_auto(Monk::Id))
                    .col(string(Monk::Name))
                    .col(integer_null(Monk::AssignedProcessId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Monk::Table).to_owned())
            .await
    }
}
