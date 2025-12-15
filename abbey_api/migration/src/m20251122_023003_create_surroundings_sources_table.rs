use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251102_101143_create_surroundings_table::Surroundings,
    m20251103_231216_create_sources_table::Sources,
};

#[derive(DeriveIden)]
enum SurroundingsSources {
    Table,
    Id,
    SurroundingsId,
    SourceId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SurroundingsSources::Table)
                    .if_not_exists()
                    .col(pk_auto(SurroundingsSources::Id))
                    .col(integer(SurroundingsSources::SurroundingsId))
                    .col(integer(SurroundingsSources::SourceId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-surroundings-sources-surroundings")
                            .from(
                                SurroundingsSources::Table,
                                SurroundingsSources::SurroundingsId,
                            )
                            .to(Surroundings::Table, Surroundings::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-surroundings-sources-source")
                            .from(
                                SurroundingsSources::Table,
                                SurroundingsSources::SurroundingsId,
                            )
                            .to(Sources::Table, Sources::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SurroundingsSources::Table).to_owned())
            .await
    }
}
