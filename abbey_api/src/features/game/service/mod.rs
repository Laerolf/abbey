use crate::{
    features::game::domain::{Game, GameCreationFactory},
    shared::{error::DomainError, DomainFactory},
};

#[derive(Default)]
pub struct GameService {}

impl GameService {
    /// Creates a new [Game][`crate::features::game::domain::Game`].
    pub fn create_game(&self) -> Result<Game, Box<dyn DomainError>> {
        GameCreationFactory::default().run()
    }
}

#[cfg(test)]
mod game_service_tests {

    mod game_creation {
        use crate::features::{
            actor::domain::{actor_status::ActorStatus, Actor},
            game::{domain::DEFAULT_AMOUNT_OF_MONKS, service::GameService},
        };

        #[test]
        fn a_new_game_has_surroundings_with_sources() {
            // Given
            let service: GameService = GameService::default();

            // When
            let game = service
                .create_game()
                .expect("It should be possible to create a game.");

            // Then
            assert!(!game.surroundings.sources.is_empty());
        }

        #[test]
        fn a_new_game_has_a_monastery_with_monks() {
            // Given
            let service: GameService = GameService::default();

            // When
            let game = service
                .create_game()
                .expect("It should be possible to create a game.");

            // Then
            assert_eq!(DEFAULT_AMOUNT_OF_MONKS as usize, game.monastery.monks.len());
        }

        #[test]
        fn a_new_game_has_an_available_player() {
            // Given
            let service: GameService = GameService::default();

            // When
            let game = service
                .create_game()
                .expect("It should be possible to create a game.");

            // Then
            assert_eq!(ActorStatus::Available, game.player.status());
        }
    }
}
