use entity::monk_skills;
use sea_orm::ConnectionTrait;

use crate::{
    features::{
        actor::{
            domain::{ActorKind, ProcessActorLink},
            error::ActorErrorKind,
        },
        monk::{mapper::MonkMapper, repository::MonkRepository},
        player::{mapper::PlayerMapper, repository::PlayerRepository},
        skill::service::SkillQueryService,
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a query service for [`Actors`][ActorKind].
#[derive(Clone)]
pub struct ActorQueryService {
    player_repository: PlayerRepository,
    monk_repository: MonkRepository,
    skill_query_service: SkillQueryService,
}

impl ActorQueryService {
    /// Creates a new [`ActorQueryService`].
    pub fn new(
        player_repository: PlayerRepository,
        monk_repository: MonkRepository,
        skill_query_service: SkillQueryService,
    ) -> Self {
        Self {
            player_repository,
            monk_repository,
            skill_query_service,
        }
    }

    /// Gets the [`Actors`][Vec<ActorKind>] with the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<ActorKind>, DomainError<ActorErrorKind>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let players = self
            .player_repository
            .get_by_ids(ids, db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::GetByIds).with_cause(error))?
            .into_iter()
            .map(|model| {
                Ok(ActorKind::Player(PlayerMapper::to_domain_entity(
                    model, None,
                )))
            })
            .collect::<Result<Vec<ActorKind>, DomainError<ActorErrorKind>>>()?;

        let monk_models = self.monk_repository.get_by_ids(ids, db_connection).await?;

        let monk_ids: Vec<i32> = monk_models.iter().map(|model| model.id).collect();

        let all_monk_skill_assignment_models = self
            .monk_repository
            .get_skill_assignments_by_monk_ids(&monk_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })?;

        let all_monk_skill_ids: Vec<i32> = all_monk_skill_assignment_models
            .iter()
            .map(|model| model.skill_id)
            .collect();

        let monk_skills = self
            .skill_query_service
            .get_all_by_ids(all_monk_skill_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })?;

        let monks = monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_assignment_models: Vec<&monk_skills::Model> =
                    all_monk_skill_assignment_models
                        .iter()
                        .filter(|skill_assignment| skill_assignment.monk_id == monk_model.id)
                        .collect();
                let monk_skill_ids: Vec<i32> = monk_skill_assignment_models
                    .iter()
                    .map(|skill_assignment| skill_assignment.skill_id)
                    .collect();

                let monk_skills = monk_skills
                    .iter()
                    .filter(|skill| {
                        skill
                            .id()
                            .is_ok_and(|skill_id| monk_skill_ids.contains(&skill_id))
                    })
                    .cloned()
                    .collect();

                Ok(ActorKind::Monk(
                    MonkMapper::to_domain_entity(monk_model, monk_skills, None).map_err(
                        |error| DomainError::from(ActorErrorKind::Restore).with_cause(error),
                    )?,
                ))
            })
            .collect::<Result<Vec<ActorKind>, DomainError<ActorErrorKind>>>()?;

        Ok(monks.into_iter().chain(players).collect())
    }

    /// Gets the [`Actors`][Vec<ProcessActorLink>] assigned to the Process with provided ID.
    pub async fn get_process_actors_by_process_id<C: ConnectionTrait>(
        &self,
        process_id: &i32,
        db_connection: &C,
    ) -> Result<Vec<ProcessActorLink>, DomainError<ActorErrorKind>> {
        let players = self
            .player_repository
            .get_by_process_id(process_id, db_connection)
            .await?
            .into_iter()
            .map(|model| {
                let process_id = model
                    .assigned_process_id
                    .ok_or_else(|| DomainError::from(ActorErrorKind::GetProcesses))?;

                Ok(ProcessActorLink::from(
                    process_id,
                    ActorKind::Player(PlayerMapper::to_domain_entity(model, None)),
                ))
            })
            .collect::<Result<Vec<ProcessActorLink>, DomainError<ActorErrorKind>>>()?;

        let monk_models = self
            .monk_repository
            .get_by_process_id(process_id, db_connection)
            .await?;

        let monk_ids: Vec<i32> = monk_models.iter().map(|model| model.id).collect();

        let all_monk_skill_assignment_models = self
            .monk_repository
            .get_skill_assignments_by_monk_ids(&monk_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })?;

        let all_monk_skill_ids: Vec<i32> = all_monk_skill_assignment_models
            .iter()
            .map(|model| model.skill_id)
            .collect();

        let monk_skills = self
            .skill_query_service
            .get_all_by_ids(all_monk_skill_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })?;

        let monks = monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_assignment_models: Vec<&monk_skills::Model> =
                    all_monk_skill_assignment_models
                        .iter()
                        .filter(|skill_assignment| skill_assignment.monk_id == monk_model.id)
                        .collect();

                let monk_skill_ids: Vec<i32> = monk_skill_assignment_models
                    .iter()
                    .map(|skill_assignment| skill_assignment.skill_id)
                    .collect();

                let monk_skills = monk_skills
                    .iter()
                    .filter(|skill| {
                        skill
                            .id()
                            .is_ok_and(|skill_id| monk_skill_ids.contains(&skill_id))
                    })
                    .cloned()
                    .collect();

                let process_id = monk_model
                    .assigned_cyclic_process_id
                    .ok_or_else(|| DomainError::from(ActorErrorKind::GetProcesses))?;

                MonkMapper::to_domain_entity(monk_model, monk_skills, None)
                    .map(|monk| ProcessActorLink::from(process_id, ActorKind::Monk(monk)))
                    .map_err(|error| DomainError::from(ActorErrorKind::Restore).with_cause(error))
            })
            .collect::<Result<Vec<ProcessActorLink>, DomainError<ActorErrorKind>>>()?;

        Ok(monks.into_iter().chain(players).collect())
    }

    /// Gets the [`Actors`][Vec<ProcessActorLink>] assigned to the Processes with provided IDs.
    pub async fn get_process_actors_by_process_ids<C: ConnectionTrait>(
        &self,
        process_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<ProcessActorLink>, DomainError<ActorErrorKind>> {
        let players = self
            .player_repository
            .get_by_process_ids(process_ids, db_connection)
            .await?
            .into_iter()
            .map(|model| {
                let process_id = model
                    .assigned_process_id
                    .ok_or_else(|| DomainError::from(ActorErrorKind::GetProcesses))?;

                Ok(ProcessActorLink::from(
                    process_id,
                    ActorKind::Player(PlayerMapper::to_domain_entity(model, None)),
                ))
            })
            .collect::<Result<Vec<ProcessActorLink>, DomainError<ActorErrorKind>>>()?;

        let monk_models = self
            .monk_repository
            .get_by_process_ids(process_ids, db_connection)
            .await?;

        let monk_ids: Vec<i32> = monk_models.iter().map(|model| model.id).collect();

        let all_monk_skill_assignment_models = self
            .monk_repository
            .get_skill_assignments_by_monk_ids(&monk_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })?;

        let all_monk_skill_ids: Vec<i32> = all_monk_skill_assignment_models
            .iter()
            .map(|model| model.skill_id)
            .collect();

        let monk_skills = self
            .skill_query_service
            .get_all_by_ids(all_monk_skill_ids, db_connection)
            .await
            .map_err(|error| {
                DomainError::from(ActorErrorKind::GetSkillAssignmentsByIds).with_cause(error)
            })?;

        let monks = monk_models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_assignment_models: Vec<&monk_skills::Model> =
                    all_monk_skill_assignment_models
                        .iter()
                        .filter(|skill_assignment| skill_assignment.monk_id == monk_model.id)
                        .collect();
                let monk_skill_ids: Vec<i32> = monk_skill_assignment_models
                    .iter()
                    .map(|skill_assignment| skill_assignment.skill_id)
                    .collect();

                let monk_skills = monk_skills
                    .iter()
                    .filter(|skill| {
                        skill
                            .id()
                            .is_ok_and(|skill_id| monk_skill_ids.contains(&skill_id))
                    })
                    .cloned()
                    .collect();

                let process_id = monk_model
                    .assigned_cyclic_process_id
                    .ok_or_else(|| DomainError::from(ActorErrorKind::GetProcesses))?;

                MonkMapper::to_domain_entity(monk_model, monk_skills, None)
                    .map(|monk| ProcessActorLink::from(process_id, ActorKind::Monk(monk)))
                    .map_err(|error| DomainError::from(ActorErrorKind::Restore).with_cause(error))
            })
            .collect::<Result<Vec<ProcessActorLink>, DomainError<ActorErrorKind>>>()?;

        Ok(monks.into_iter().chain(players).collect())
    }
}
