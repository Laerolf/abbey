use sea_orm::DatabaseTransaction;

use crate::{
    features::player::{
        domain::Player,
        error::PlayerErrorKind,
        forms::PlayerCreationForm,
        mapper::PlayerMapper,
        repository::{PlayerRepository, PlayerWithRelations},
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Player`] topic.
#[derive(Clone, Default)]
pub struct PlayerService {
    repository: PlayerRepository,
}

impl PlayerService {
    /// Finds a [`Player`] by its ID and all its related entities.
    pub async fn find_by_id_with_relations(
        &self,
        id: i32,
    ) -> Result<Option<Player>, DomainError<PlayerErrorKind>> {
        let Some(model) = self
            .repository
            .find_by_id_with_relations(id)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::FindById).with_cause(error))?
        else {
            return Ok(None);
        };

        Ok(Some(PlayerMapper::to_domain_entity_with_relations(model)))
    }

    /// Creates a new [`Player`][`PlayerWithRelations`].
    pub async fn create_player_in_transaction(
        &self,
        transaction: &DatabaseTransaction,
    ) -> Result<PlayerWithRelations, DomainError<PlayerErrorKind>> {
        let creation_form = PlayerCreationForm::new();

        self.repository
            .create_with_relations_in_transaction(creation_form, transaction)
            .await
            .map_err(|error| DomainError::from(PlayerErrorKind::Creation).with_cause(error))
    }
}
