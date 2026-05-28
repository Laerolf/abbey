use entity::monk_skills;
use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    features::{
        actor::{
            domain::{ActorKind, ProcessActorLink, monk::Monk},
            error::ActorErrorKind,
            forms::MonkCreationForm,
            mapper::MonkMapper,
            repository::MonkRepository,
        },
        player::{mapper::PlayerMapper, repository::PlayerRepository},
        process::{
            domain::{ProcessKind, cyclic_process::CyclicProcess},
            service::cyclic_process::CyclicProcessQueryService,
        },
        skill::{domain::Skill, service::SkillQueryService},
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents a service handling the [`Monk`] topic.
#[derive(Clone)]
pub struct MonkService {
    repository: MonkRepository,
}

impl MonkService {
    /// Creates a new [`MonkService`].
    pub fn new(repository: MonkRepository) -> Self {
        Self { repository }
    }

    /// Creates a new [`Monk`].
    pub async fn create_monk(
        &self,
        creation_form: MonkCreationForm,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Monk, DomainError<ActorErrorKind>> {
        self.repository
            .create(creation_form, db_transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))
    }

    /// Creates new [`Monks`][Vec<Monk>].
    pub async fn create_many_monks(
        &self,
        creation_forms: Vec<MonkCreationForm>,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Vec<Monk>, DomainError<ActorErrorKind>> {
        self.repository
            .create_many(creation_forms, db_transaction)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))
    }
}

/// Represents a query service for [`Monks`][Monk].
#[derive(Clone)]
pub struct MonkQueryService {
    repository: MonkRepository,
    skill_query_service: SkillQueryService,
    cyclic_process_query_service: CyclicProcessQueryService,
}

impl MonkQueryService {
    /// Creates a new [`MonkQueryService`].
    pub fn new(
        repository: MonkRepository,
        skill_query_service: SkillQueryService,
        cyclic_process_query_service: CyclicProcessQueryService,
    ) -> Self {
        Self {
            repository,
            skill_query_service,
            cyclic_process_query_service,
        }
    }

    /// Gets the [`Monks`][Vec<Monk>] for the IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<Monk>, DomainError<ActorErrorKind>> {
        let models = self.repository.get_by_ids(ids, db_connection).await?;

        let skill_assignments = self
            .repository
            .get_skill_assignments_by_monk_ids(ids, db_connection)
            .await?;

        let skill_ids: Vec<i32> = skill_assignments
            .iter()
            .map(|assignment| assignment.skill_id)
            .collect();

        let skills = self
            .skill_query_service
            .get_all_by_ids(skill_ids, db_connection)
            .await
            .map_err(|error| DomainError::from(ActorErrorKind::NoSkills).with_cause(error))?;

        let process_ids: Vec<i32> = models
            .iter()
            .filter_map(|monastery_monk_model| monastery_monk_model.assigned_cyclic_process_id)
            .collect();

        let processes: Vec<CyclicProcess> = if process_ids.is_empty() {
            Vec::new()
        } else {
            self.cyclic_process_query_service
                .get_by_ids(&process_ids, db_connection)
                .await
                .map_err(|error| {
                    DomainError::from(ActorErrorKind::GetProcesses).with_cause(error)
                })?
        };

        models
            .into_iter()
            .map(|monk_model| {
                let monk_skill_ids: Vec<i32> = skill_assignments
                    .iter()
                    .filter(|skill_assignment| skill_assignment.monk_id == monk_model.id)
                    .map(|skill_assignment| skill_assignment.skill_id)
                    .collect();

                let monk_skills: Vec<Skill> = skills
                    .iter()
                    .filter(|skill| skill.id().is_ok_and(|id| monk_skill_ids.contains(&id)))
                    .cloned()
                    .collect();

                let monk_process = monk_model
                    .assigned_cyclic_process_id
                    .and_then(|process_id| {
                        processes
                            .iter()
                            .find(|cyclic_process| {
                                cyclic_process.id().is_ok_and(|id| id == process_id)
                            })
                            .cloned()
                    })
                    .map(ProcessKind::CyclicProcess);

                MonkMapper::to_domain_entity(monk_model, monk_skills, monk_process)
                    .map_err(|error| DomainError::from(ActorErrorKind::Creation).with_cause(error))
            })
            .collect::<Result<Vec<Monk>, DomainError<ActorErrorKind>>>()
    }
}

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

    /// Gets the [`Actors`][Vec<ProcessActorLink>] assigned to the Processes with provided IDs.
    pub async fn get_process_actors_by_process_ids<C: ConnectionTrait>(
        &self,
        process_ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<ProcessActorLink>, DomainError<ActorErrorKind>> {
        let players = self
            .player_repository
            .find_by_process_ids(process_ids, db_connection)
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
            .find_by_process_ids(process_ids, db_connection)
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
