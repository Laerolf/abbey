use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251105_050258_create_games_table::Games;

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    Status,
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
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))
                    .col(string_uniq(Users::Email))
                    .col(string(Users::Status))
                    .col(integer_null(Users::GameId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-games-user")
                            .from(Users::Table, Users::GameId)
                            .to(Games::Table, Games::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
