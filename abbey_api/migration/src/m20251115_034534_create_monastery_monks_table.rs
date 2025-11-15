use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251105_050818_create_monasteries_table::Monasteries,
    m20251105_071714_create_monks_table::Monks,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum MonasteryMonks {
    Table,
    Id,
    MonasteryId,
    MonkId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MonasteryMonks::Table)
                    .if_not_exists()
                    .col(pk_auto(MonasteryMonks::Id))
                    .col(integer(MonasteryMonks::MonasteryId))
                    .col(integer(MonasteryMonks::MonkId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monastery-monks-monastery")
                            .from(MonasteryMonks::Table, MonasteryMonks::MonasteryId)
                            .to(Monasteries::Table, Monasteries::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monastery-monks-monk")
                            .from(MonasteryMonks::Table, MonasteryMonks::MonkId)
                            .to(Monks::Table, Monks::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MonasteryMonks::Table).to_owned())
            .await
    }
}
