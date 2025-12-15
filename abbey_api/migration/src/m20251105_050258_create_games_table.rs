use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251102_100818_create_monasteries_table::Monasteries,
    m20251102_101143_create_surroundings_table::Surroundings,
    m20251103_085752_create_players_table::Players,
};

#[derive(DeriveIden)]
pub enum Games {
    Table,
    Id,
    PlayerId,
    MonasteryId,
    SurroundingsId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Games::Table)
                    .if_not_exists()
                    .col(pk_auto(Games::Id))
                    .col(integer(Games::PlayerId))
                    .col(integer(Games::MonasteryId))
                    .col(integer(Games::SurroundingsId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-player")
                            .from(Games::Table, Games::PlayerId)
                            .to(Players::Table, Players::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-monastery")
                            .from(Games::Table, Games::MonasteryId)
                            .to(Monasteries::Table, Monasteries::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-monastery-surroundings")
                            .from(Games::Table, Games::SurroundingsId)
                            .to(Surroundings::Table, Surroundings::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Games::Table).to_owned())
            .await
    }
}
