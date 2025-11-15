use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20251105_071714_create_monks_table::Monks, m20251115_042853_create_skills_table::Skills,
};

#[derive(DeriveIden)]
enum MonkSkills {
    Table,
    Id,
    MonkId,
    SkillId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MonkSkills::Table)
                    .if_not_exists()
                    .col(pk_auto(MonkSkills::Id))
                    .col(integer(MonkSkills::MonkId))
                    .col(integer(MonkSkills::SkillId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monk-skills-monk")
                            .from(MonkSkills::Table, MonkSkills::MonkId)
                            .to(Monks::Table, Monks::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-monk-skills-skill")
                            .from(MonkSkills::Table, MonkSkills::MonkId)
                            .to(Skills::Table, Skills::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MonkSkills::Table).to_owned())
            .await
    }
}
