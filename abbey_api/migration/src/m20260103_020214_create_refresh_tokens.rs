use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251115_235801_create_users_table::Users;

#[derive(DeriveIden)]
enum RefreshTokens {
    Table,
    Id,
    CreatedAt,
    LastUpdatedAt,
    UserId,
    Value,
    ExpiresAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RefreshTokens::Table)
                    .if_not_exists()
                    .col(pk_auto(RefreshTokens::Id))
                    .col(timestamp_with_time_zone(RefreshTokens::CreatedAt))
                    .col(timestamp_with_time_zone_null(RefreshTokens::LastUpdatedAt))
                    .col(integer_uniq(RefreshTokens::UserId))
                    .col(string(RefreshTokens::Value))
                    .col(timestamp_with_time_zone(RefreshTokens::ExpiresAt))
                    .index(
                        Index::create()
                            .name("ui-refresh-tokens-user-id")
                            .table(RefreshTokens::Table)
                            .col(RefreshTokens::UserId)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("ui-refresh-tokens-value")
                            .table(RefreshTokens::Table)
                            .col(RefreshTokens::Value)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-refresh-tokens-user")
                            .from(RefreshTokens::Table, RefreshTokens::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await
    }
}
