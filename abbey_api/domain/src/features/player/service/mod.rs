use crate::{
    features::player::{
        domain::Player, error::PlayerError, forms::PlayerCreationForm, mapper::PlayerMapper,
        repository::PlayerRepository,
    },
    shared::error::DomainError,
};

/// Represents a service handling the [`Player`] topic.
#[derive(Default, Clone)]
pub struct PlayerService {
    repository: PlayerRepository,
}

impl PlayerService {
    /// Creates a new [`Player`].
    pub async fn create_player(&self) -> Result<Player, Box<dyn DomainError>> {
        let creation_form = PlayerCreationForm::new();

        match self
            .repository
            .insert(PlayerMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(new_player) => Ok(PlayerMapper::to_domain_entity(new_player, None)),
            Err(_error) => Err(Box::new(PlayerError::Creation)),
        }
    }
}
