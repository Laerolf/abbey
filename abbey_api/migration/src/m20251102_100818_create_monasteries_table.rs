use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum Monasteries {
    Table,
    Id,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Monasteries::Table)
                    .if_not_exists()
                    .col(pk_auto(Monasteries::Id))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Monasteries::Table).to_owned())
            .await
    }
}
