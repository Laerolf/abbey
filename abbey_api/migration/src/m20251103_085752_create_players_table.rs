use sea_orm_migration::{prelude::*, schema::*};

use crate::m20251103_095220_create_cyclic_processes_table::CyclicProcess;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Player {
    Table,
    Id,
    AssignedProcessId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(pk_auto(Player::Id))
                    .col(integer_null(Player::AssignedProcessId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-player-cyclic-process")
                            .from(Player::Table, Player::AssignedProcessId)
                            .to(CyclicProcess::Table, CyclicProcess::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await
    }
}
