use sea_orm::DatabaseTransaction;

use crate::{
    features::{
        game::{
            domain::Game, error::GameErrorKind, forms::GameCreationForm, mapper::GameMapper,
            repository::GameRepository,
        },
        monastery::service::MonasteryService,
        player::service::PlayerService,
        surroundings::service::SurroundingsService,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [Game] topic.
#[derive(Clone)]
pub struct GameService {
    repository: GameRepository,
    monastery_service: MonasteryService,
    player_service: PlayerService,
    surroundings_service: SurroundingsService,
}

impl GameService {
    /// Creates a new [`GameService`].
    pub fn new(
        monastery_service: MonasteryService,
        player_service: PlayerService,
        surroundings_service: SurroundingsService,
    ) -> Self {
        Self {
            repository: GameRepository,
            monastery_service,
            player_service,
            surroundings_service,
        }
    }

    /// Finds a [`Game`] by its ID.
    pub async fn find_by_id(&self, _id: i32) -> Result<Option<Game>, GameErrorKind> {
        // TODO: Find everything by its ID and construct the Game as in create_game
        Ok(None)
    }

    /// Creates a new [Game].
    pub async fn create_game_in_transaction(
        &self,
        transaction: &DatabaseTransaction,
    ) -> Result<Game, DomainError<GameErrorKind>> {
        let related_monastery = self
            .monastery_service
            .create_monastery_in_transaction(transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let related_player = self
            .player_service
            .create_player_in_transaction(transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let related_surroundings = self
            .surroundings_service
            .create_surroundings_in_transaction(transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        let creation_form = GameCreationForm::new(
            related_player.player.id,
            related_monastery.monastery.id,
            related_surroundings.surroundings.id,
        );

        let related_game = self
            .repository
            .create_with_relations_in_transaction(creation_form, transaction)
            .await
            .map_err(|error| DomainError::from(GameErrorKind::Creation).with_cause(error))?;

        Ok(GameMapper::to_domain_entity_with_relations(
            related_game,
            related_player,
            related_monastery,
            related_surroundings,
        ))
    }
}
