use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251105_050258_create_games_table::Games, m20251115_235801_create_users_table::Users,
};

#[derive(DeriveIden)]
pub enum UserGames {
    Table,
    Id,
    UserId,
    GameId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGames::Table)
                    .if_not_exists()
                    .col(pk_auto(UserGames::Id))
                    .col(integer(UserGames::UserId))
                    .col(integer_uniq(UserGames::GameId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-games-user")
                            .from(UserGames::Table, UserGames::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-games-game")
                            .from(UserGames::Table, UserGames::GameId)
                            .to(Games::Table, Games::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGames::Table).to_owned())
            .await
    }
}
