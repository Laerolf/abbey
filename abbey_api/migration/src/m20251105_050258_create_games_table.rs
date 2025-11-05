use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251103_085752_create_players_table::Player,
    m20251105_050818_create_monasteries_table::Monastery,
    m20251105_051143_create_monastery_surroundings_table::Surroundings,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    PlayerId,
    MonasteryId,
    SurroundingsId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(pk_auto(Game::Id))
                    .col(integer(Game::PlayerId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-player")
                            .from(Game::Table, Game::PlayerId)
                            .to(Player::Table, Player::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Game::MonasteryId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-monastery")
                            .from(Game::Table, Game::MonasteryId)
                            .to(Monastery::Table, Monastery::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Game::SurroundingsId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-game-monastery-surroundings")
                            .from(Game::Table, Game::SurroundingsId)
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
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
    }
}
