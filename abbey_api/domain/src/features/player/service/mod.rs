use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    features::player::{
        domain::Player, error::PlayerErrorKind, forms::PlayerCreationForm,
        repository::PlayerRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Player`] topic.
#[derive(Clone)]
pub struct PlayerService {
    repository: PlayerRepository,
}

impl PlayerService {
    /// Creates a new [`PlayerService`].
    pub fn new(repository: PlayerRepository) -> Self {
        Self { repository }
    }

    /// Finds a [`Player`] by its ID and all its related entities.
    pub async fn find_by_id_with_relations<C: ConnectionTrait>(
        &self,
        id: &i32,
        db_connection: &C,
    ) -> Result<Option<Player>, DomainError<PlayerErrorKind>> {
        self.repository
            .find_by_id_with_relations(id, db_connection)
            .await
    }

    /// Creates a new [`Player`].
    pub async fn create_player(
        &self,
        db_transaction: &DatabaseTransaction,
    ) -> Result<Player, DomainError<PlayerErrorKind>> {
        let creation_form = PlayerCreationForm::new();

        self.repository.create(creation_form, db_transaction).await
    }
}
