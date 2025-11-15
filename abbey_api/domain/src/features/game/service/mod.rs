use crate::{
    features::{
        game::{
            domain::Game, error::GameError, forms::GameCreationForm, mapper::GameMapper,
            repository::GameRepository,
        },
        monastery::{domain::Monastery, service::MonasteryService},
        player::{domain::Player, service::PlayerService},
        surroundings::{domain::Surroundings, service::SurroundingsService},
    },
    shared::error::DomainError,
};

/// Represents a service handling the [Game] topic.
#[derive(Default)]
pub struct GameService {
    repository: GameRepository,
    monastery_service: MonasteryService,
    player_service: PlayerService,
    surroundings_service: SurroundingsService,
}

impl GameService {
    /// Creates a new [Game].
    pub async fn create_game(&self) -> Result<Game, Box<dyn DomainError>> {
        let monastery: Monastery = self.monastery_service.create_monastery().await?;
        let player: Player = self.player_service.create_player().await?;
        let surroundings: Surroundings = self.surroundings_service.create_surroundings().await?;

        let creation_form = GameCreationForm::new(1, monastery.id, 1);

        match self
            .repository
            .insert(GameMapper::to_new_active_model(creation_form))
            .await
        {
            Ok(game) => Ok(GameMapper::to_domain_entity(
                game,
                player,
                monastery,
                surroundings,
            )),
            Err(_error) => Err(Box::new(GameError::Creation)),
        }
    }
}
