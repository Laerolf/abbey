use entity::{monk_skills, monks, skills};
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Statement,
};

use crate::{
    features::{
        actor::{domain::ActorKind, error::ActorErrorKind},
        monk::{domain::Monk, mapper::MonkMapper},
        skill::{domain::Skill, mapper::SkillMapper},
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that handles all [`Actor`][ActorKind] database topics.
#[derive(Default, Clone)]
pub struct ActorRepository;

impl ActorRepository {
    /// Finds an [`Actor`][ActorKind] for the provided ID and Game ID.
    pub async fn find_by_id_for_game<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Option<ActorKind>, DomainError<ActorErrorKind>> {
        let Some(monk_model) = monks::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db_connection.get_database_backend(),
                r#"
                        SELECT mo.*
                        FROM monks mo
                        INNER JOIN monastery_monks mm ON mm.monk_id = mo.id
                        INNER JOIN games g ON g.monastery_id = mm.monastery_id
                        WHERE g.id = $1
                        AND mo.id = $2
                    "#,
                [(*game_id).into(), (*id).into()],
            ))
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        let monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.eq(monk_model.id))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let monk = ActorKind::Monk(
            MonkMapper::to_domain_entity(monk_model, monk_skills, None)
                .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?,
        );

        Ok(Some(monk))
    }

    /// Gets an [`Actor`][ActorKind] for the provided ID and Game ID.
    pub async fn get_by_id_for_game<C: ConnectionTrait>(
        &self,
        id: &i32,
        game_id: &i32,
        db_connection: &C,
    ) -> Result<ActorKind, DomainError<ActorErrorKind>> {
        self.find_by_id_for_game(id, game_id, db_connection)
            .await?
            .ok_or_else(|| DomainError::from(ActorErrorKind::GetById))
    }

    /// Finds [`Actors`][Vec<ActorKind>] for the provided IDs and Game ID.
    pub async fn find_many_by_ids_for_game<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        game_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<ActorKind>, DomainError<ActorErrorKind>> {
        let placeholders: Vec<String> = (2..=ids.len() + 1).map(|i| format!("${}", i)).collect();
        let in_clause = placeholders.join(", ");

        let sql = format!(
            r#"
                SELECT mo.*
                FROM monks mo
                INNER JOIN monastery_monks mm ON mm.monk_id = mo.id
                INNER JOIN monasteries mon ON mon.id = mm.monastery_id
                INNER JOIN games g ON g.monastery_id = mon.id
                WHERE g.id = $1
                AND mo.id IN ({})
            "#,
            in_clause
        );

        let mut values: Vec<sea_orm::Value> = vec![(*game_id).into()];
        values.extend(ids.iter().map(|id| (*id).into()));

        let all_monk_models = monks::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db_connection.get_database_backend(),
                sql,
                values,
            ))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?;

        let all_monk_ids: Vec<i32> = all_monk_models
            .iter()
            .map(|monk_model| monk_model.id)
            .collect();

        let all_monk_skill_assignments = monk_skills::Entity::find()
            .filter(monk_skills::Column::MonkId.is_in(all_monk_ids.clone()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?;

        let all_monk_skills: Vec<Skill> = skills::Entity::find()
            .inner_join(monk_skills::Entity)
            .filter(monk_skills::Column::MonkId.is_in(all_monk_ids))
            .distinct()
            .order_by_asc(skills::Column::Id)
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))?
            .into_iter()
            .map(SkillMapper::to_domain_entity)
            .collect();

        let all_monks: Vec<Monk> = all_monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_ids: Vec<i32> = all_monk_skill_assignments
                    .iter()
                    .filter(|skill_assignment_model| {
                        skill_assignment_model.monk_id == monk_model.id
                    })
                    .map(|skill_assignment_model| skill_assignment_model.skill_id)
                    .collect();

                let monk_skills = all_monk_skills
                    .iter()
                    .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                    .cloned()
                    .collect();

                MonkMapper::to_domain_entity(monk_model, monk_skills, None)
                    .map_err(|error| DomainError::from(ActorErrorKind::FindById).with_cause(error))
            })
            .collect::<Result<Vec<Monk>, DomainError<ActorErrorKind>>>()?;

        let all_actors: Vec<ActorKind> = all_monks.into_iter().map(ActorKind::Monk).collect();

        Ok(all_actors)
    }
}
