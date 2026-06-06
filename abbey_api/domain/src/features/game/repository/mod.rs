use entity::{
    cyclic_process_resources, cyclic_processes, games, monasteries, monastery_monks, monk_skills,
    monks, players, resources, skills, sources, surroundings, surroundings_sources,
};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::{
    features::{
        actor::{
            domain::{ActorKind, monk::Monk},
            mapper::MonkMapper,
        },
        game::{domain::Game, error::GameErrorKind, mapper::GameMapper},
        monastery::{domain::Monastery, mapper::MonasteryMapper},
        output::{domain::resource::Resource, mapper::ResourceMapper},
        player::{domain::Player, mapper::PlayerMapper},
        process::{
            domain::{ProcessKind, cyclic_process::CyclicProcess},
            mapper::cyclic_process::CyclicProcessMapper,
        },
        skill::{domain::Skill, mapper::SkillMapper},
        source::{domain::Source, error::SourceErrorKind, mapper::SourceMapper},
        surroundings::{domain::Surroundings, mapper::SurroundingsMapper},
    },
    shared::{DomainElement, error::DomainError},
};

/// Represents an element that handles all [Game][`crate::features::game::domain::Game`] database topics.
#[derive(Default, Clone)]
pub struct GameRepository;

impl GameRepository {
    /// Gets the [Player], [Monastery] and [Surroundings] of a [`Game`][games::Model].
    async fn get_relations<C: ConnectionTrait>(
        &self,
        game_model: games::Model,
        db_connection: &C,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        let player = get_player(&game_model, db_connection).await?;
        let monastery = get_monastery(&game_model, db_connection).await?;
        let surroundings = get_surroundings(&game_model, db_connection).await?;

        Ok(GameMapper::to_domain_entity(
            game_model,
            player,
            monastery,
            surroundings,
        ))
    }

    /// Creates a new [Game][`games::Model`] and persists it in the database.
    pub async fn create<C: ConnectionTrait>(
        &self,
        model: games::ActiveModel,
        db_connection: &C,
    ) -> Result<games::Model, DomainError<GameErrorKind>> {
        model
            .insert(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))
    }

    /// Finds a [`Game`][games::Model] by its ID.
    pub async fn find_by_id<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<games::Model>, DomainError<GameErrorKind>> {
        games::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::FindById).with_cause(error))
    }

    /// Gets the [`Games`][Vec<games::Model>] for the provided IDs.
    pub async fn get_by_ids<C: ConnectionTrait>(
        &self,
        ids: &[i32],
        db_connection: &C,
    ) -> Result<Vec<games::Model>, DomainError<GameErrorKind>> {
        games::Entity::find()
            .filter(games::Column::Id.is_in(ids.to_vec()))
            .all(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::GetById).with_cause(error))
    }

    /// Finds a [Game] by its ID and with all its related entities.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Game>, DomainError<GameErrorKind>> {
        let Some(game_model) = games::Entity::find_by_id(*id)
            .one(db_connection)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(self.get_relations(game_model, db_connection).await?))
    }
}

/// Gets the [Player] of the [`Game`].
async fn get_player<C: ConnectionTrait>(
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Player, DomainError<GameErrorKind>> {
    let Some(player) = players::Entity::find()
        .inner_join(games::Entity)
        .filter(games::Column::PlayerId.eq(game_model.player_id))
        .one(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::PlayerNotFound).with_cause(error))?
        .map(|player_model| PlayerMapper::to_domain_entity(player_model, None))
    else {
        return Err(DomainError::from(GameErrorKind::PlayerNotFound));
    };

    Ok(player)
}

/// Gets the [Monks][Vec<Monk>] of the [`Game`]'s [Monastery].
async fn get_monastery_monks<C: ConnectionTrait>(
    monastery_model: &monasteries::Model,
    db_connection: &C,
) -> Result<Vec<Monk>, DomainError<GameErrorKind>> {
    let monastery_monks_models = monks::Entity::find()
        .inner_join(monastery_monks::Entity)
        .filter(monastery_monks::Column::MonasteryId.eq(monastery_model.id))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?;

    let monastery_monk_ids: Vec<i32> = monastery_monks_models
        .iter()
        .map(|monastery_monk_model| monastery_monk_model.id)
        .collect();

    let monastery_monk_skill_assignments = monk_skills::Entity::find()
        .filter(monk_skills::Column::MonkId.is_in(monastery_monk_ids))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?;

    let monastery_monk_skill_ids: Vec<i32> = monastery_monk_skill_assignments
        .iter()
        .map(|assignment| assignment.skill_id)
        .collect();

    let monastery_monk_skills: Vec<Skill> = skills::Entity::find()
        .filter(skills::Column::Id.is_in(monastery_monk_skill_ids))
        .distinct()
        .order_by_asc(skills::Column::Id)
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
        .into_iter()
        .map(SkillMapper::to_domain_entity)
        .collect();

    let monastery_monk_process_ids: Vec<i32> = monastery_monks_models
        .iter()
        .filter_map(|monastery_monk_model| monastery_monk_model.assigned_cyclic_process_id)
        .collect();

    let monastery_monk_process_output_resource_assignments =
        cyclic_process_resources::Entity::find()
            .filter(
                cyclic_process_resources::Column::CyclicProcessId
                    .is_in(monastery_monk_process_ids.clone()),
            )
            .all(db_connection)
            .await
            .map_err(|error| {
                DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error)
            })?;

    let monastery_monk_process_output_resource_ids: Vec<i32> =
        monastery_monk_process_output_resource_assignments
            .iter()
            .map(|assignment| assignment.resource_id)
            .collect();

    let monastery_monk_process_output_resources: Vec<Resource> = resources::Entity::find()
        .filter(resources::Column::Id.is_in(monastery_monk_process_output_resource_ids))
        .distinct()
        .order_by_asc(resources::Column::Id)
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
        .into_iter()
        .map(ResourceMapper::to_domain_entity)
        .collect();

    let monastery_monk_processes: Vec<CyclicProcess> = cyclic_processes::Entity::find()
        .filter(cyclic_processes::Column::Id.is_in(monastery_monk_process_ids))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
        .into_iter()
        .map(|cyclic_process_model| {
            let output_resource_ids: Vec<i32> = monastery_monk_process_output_resource_assignments
                .iter()
                .filter(|assignment| assignment.cyclic_process_id == cyclic_process_model.id)
                .clone()
                .map(|assignment| assignment.resource_id)
                .collect();

            let output_resources: Vec<Resource> = monastery_monk_process_output_resources
                .clone()
                .into_iter()
                .filter(|resource| output_resource_ids.contains(&resource.id().unwrap()))
                .collect();

            CyclicProcessMapper::to_domain_entity(
                cyclic_process_model,
                output_resources,
                Vec::new(),
            )
            .unwrap()
        })
        .collect();

    monastery_monks_models
        .into_iter()
        .map(|monk_model| {
            let monk_skill_ids: Vec<i32> = monastery_monk_skill_assignments
                .iter()
                .filter(|skill_assignment| skill_assignment.monk_id == monk_model.id)
                .map(|skill_assignment| skill_assignment.skill_id)
                .collect();

            let monk_skills: Vec<Skill> = monastery_monk_skills
                .iter()
                .filter(|skill| monk_skill_ids.contains(&skill.id().unwrap()))
                .cloned()
                .collect();

            let monk_process = monk_model
                .assigned_cyclic_process_id
                .and_then(|process_id| {
                    monastery_monk_processes
                        .iter()
                        .find(|cyclic_process| cyclic_process.id().is_ok_and(|id| id == process_id))
                        .map(|cyclic_process| ProcessKind::CyclicProcess(cyclic_process.clone()))
                });

            MonkMapper::to_domain_entity(monk_model, monk_skills, monk_process).map_err(|error| {
                DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error)
            })
        })
        .collect::<Result<Vec<Monk>, DomainError<GameErrorKind>>>()
}

/// Gets the [Monastery] of a [`Game`].
async fn get_monastery<C: ConnectionTrait>(
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Monastery, DomainError<GameErrorKind>> {
    let Some(monastery_model) = monasteries::Entity::find()
        .inner_join(games::Entity)
        .filter(games::Column::MonasteryId.eq(game_model.monastery_id))
        .one(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
    else {
        return Err(DomainError::from(GameErrorKind::MonasteryNotFound));
    };

    let monks = get_monastery_monks(&monastery_model, db_connection).await?;

    MonasteryMapper::to_domain_entity(monastery_model, monks)
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))
}

/// Gets the [Surroundings] of a [Game].
async fn get_surroundings<C: ConnectionTrait>(
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Surroundings, DomainError<GameErrorKind>> {
    let Some(model) = surroundings::Entity::find()
        .inner_join(games::Entity)
        .filter(games::Column::SurroundingsId.eq(game_model.surroundings_id))
        .one(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?
    else {
        return Err(DomainError::from(GameErrorKind::SurroundingsNotFound));
    };

    let sources = get_surroundings_sources(&model, game_model, db_connection).await?;

    SurroundingsMapper::to_domain_entity(model, sources)
        .map_err(|error| DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error))
}

/// Gets the [Sources][Vec<Source>] of a [Game].
async fn get_surroundings_sources<C: ConnectionTrait>(
    surroundings_model: &surroundings::Model,
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Vec<Source>, DomainError<GameErrorKind>> {
    let models = sources::Entity::find()
        .inner_join(surroundings_sources::Entity)
        .filter(surroundings_sources::Column::SurroundingsId.eq(surroundings_model.id))
        .all(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    let processes = get_sources_cyclic_processes(&models, game_model, db_connection).await?;

    models
        .into_iter()
        .map(|source_model| {
            let process = processes
                .iter()
                .find(|process| process.id().unwrap() == source_model.cyclic_process_id)
                .ok_or(DomainError::from(SourceErrorKind::FindProcess))
                .map_err(|error| {
                    DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
                })?;

            Ok(SourceMapper::to_domain_entity(
                source_model,
                process.clone(),
            ))
        })
        .collect::<Result<Vec<Source>, DomainError<GameErrorKind>>>()
}

/// Gets the [Processes][Vec<CyclicProcess>] of the [Sources][Vec<Source>] of a [Game].
async fn get_sources_cyclic_processes<C: ConnectionTrait>(
    source_models: &[sources::Model],
    game_model: &games::Model,
    db_connection: &C,
) -> Result<Vec<CyclicProcess>, DomainError<GameErrorKind>> {
    let source_process_ids: Vec<i32> = source_models
        .iter()
        .map(|model| model.cyclic_process_id)
        .collect();

    let models = cyclic_processes::Entity::find()
        .filter(cyclic_processes::Column::Id.is_in(source_process_ids.clone()))
        .all(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    let output_resource_assignments = cyclic_process_resources::Entity::find()
        .filter(cyclic_process_resources::Column::CyclicProcessId.is_in(source_process_ids.clone()))
        .all(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    let output_resource_ids: Vec<i32> = output_resource_assignments
        .iter()
        .map(|model| model.resource_id)
        .collect();

    let output_resources: Vec<Resource> = resources::Entity::find()
        .filter(resources::Column::Id.is_in(output_resource_ids))
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error))?
        .into_iter()
        .map(ResourceMapper::to_domain_entity)
        .collect();

    let all_assigned_monk_models = monks::Entity::find()
        .filter(monks::Column::AssignedCyclicProcessId.is_in(source_process_ids.clone()))
        .distinct()
        .all(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    let all_assigned_monk_ids: Vec<i32> = all_assigned_monk_models
        .iter()
        .map(|model| model.id)
        .collect();

    let all_assigned_monk_skill_assignments = monk_skills::Entity::find()
        .filter(monk_skills::Column::MonkId.is_in(all_assigned_monk_ids.clone()))
        .distinct()
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?;

    let all_skills: Vec<Skill> = skills::Entity::find()
        .inner_join(monk_skills::Entity)
        .filter(monk_skills::Column::MonkId.is_in(all_assigned_monk_ids))
        .distinct()
        .all(db_connection)
        .await
        .map_err(|error| DomainError::from(GameErrorKind::MonasteryNotFound).with_cause(error))?
        .into_iter()
        .map(SkillMapper::to_domain_entity)
        .collect();

    let optional_player_model = players::Entity::find()
        .filter(players::Column::Id.eq(game_model.player_id))
        .filter(players::Column::AssignedProcessId.is_in(source_process_ids))
        .one(db_connection)
        .await
        .map_err(|error| {
            DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
        })?;

    models
        .into_iter()
        .map(|process_model| {
            let output_resource_ids: Vec<i32> = output_resource_assignments
                .iter()
                .filter(|model| model.cyclic_process_id == process_model.id)
                .map(|model| model.resource_id)
                .collect();

            let output_resources: Vec<Resource> = output_resources
                .clone()
                .into_iter()
                .filter(|resource| output_resource_ids.contains(&resource.id().unwrap()))
                .collect();

            let all_assigned_monks: Vec<Monk> = all_assigned_monk_models
                .clone()
                .into_iter()
                .filter(|model| {
                    model
                        .assigned_cyclic_process_id
                        .is_some_and(|id| process_model.id == id)
                })
                .map(|model| {
                    let skill_ids: Vec<i32> = all_assigned_monk_skill_assignments
                        .iter()
                        .filter(|assignment| assignment.monk_id == model.id)
                        .map(|assignment| assignment.skill_id)
                        .collect();

                    let skills = all_skills
                        .clone()
                        .into_iter()
                        .filter(|skill| skill.id().is_ok_and(|id| skill_ids.contains(&id)))
                        .collect();

                    MonkMapper::to_domain_entity(model, skills, None).map_err(|error| {
                        DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
                    })
                })
                .collect::<Result<Vec<Monk>, DomainError<GameErrorKind>>>()?;

            let mut assigned_actors: Vec<ActorKind> = all_assigned_monks
                .clone()
                .into_iter()
                .map(ActorKind::Monk)
                .collect();

            if let Some(ref player_model) = optional_player_model
                && player_model
                    .assigned_process_id
                    .is_some_and(|id| id == process_model.id)
            {
                assigned_actors.push(ActorKind::Player(PlayerMapper::to_domain_entity(
                    player_model.clone(),
                    None,
                )));
            }

            CyclicProcessMapper::to_domain_entity(process_model, output_resources, assigned_actors)
                .map_err(|error| {
                    DomainError::from(GameErrorKind::SurroundingsNotFound).with_cause(error)
                })
        })
        .collect::<Result<Vec<CyclicProcess>, DomainError<GameErrorKind>>>()
}
