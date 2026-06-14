use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    features::game::error::{GameEngineErrorKind, GameErrorKind},
    shared::error::DomainError,
};

/// Represents a generator for Game seeds.
pub struct GameSeedGenerator;

impl GameSeedGenerator {
    /// Gets the next random Game seed.
    pub fn next() -> u64 {
        rand::random::<u64>()
    }
}

#[cfg(test)]
pub mod game_seed_generator_tests {

    use crate::features::game::domain::game_engine::GameSeedGenerator;

    #[test]
    pub fn test_get_next_random_game_seed() {
        // When
        let game_seed = GameSeedGenerator::next();

        // Then
        assert_ne!(0, game_seed);
    }
}

/// Represents a Game engine.
#[derive(Clone, serde::Deserialize)]
pub struct GameEngine {
    generator: ChaCha8Rng,
}

impl GameEngine {
    /// Creates a new [`GameEngine`].
    pub fn new(seed: u64) -> Self {
        Self {
            generator: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    /// Restores a [`GameEngine`].
    pub fn restore(generator: ChaCha8Rng) -> Self {
        Self { generator }
    }

    /// Restores a [`GameEngine`].
    pub fn from(state: &str) -> Result<Self, DomainError<GameErrorKind>> {
        let generator: ChaCha8Rng = serde_json::from_str(state).map_err(|error| {
            DomainError::from(GameErrorKind::GameEngine(GameEngineErrorKind::Deserialize))
                .with_cause(error)
        })?;

        Ok(Self::restore(generator))
    }

    pub fn generator(&self) -> &ChaCha8Rng {
        &self.generator
    }

    pub fn as_string(&self) -> Result<String, DomainError<GameErrorKind>> {
        serde_json::to_string(self.generator()).map_err(|error| {
            DomainError::from(GameErrorKind::GameEngine(GameEngineErrorKind::Serialize))
                .with_cause(error)
        })
    }

    /// Picks a random element from the provided collections of elements.
    pub fn pick_random_element<T: Clone>(&mut self, elements: &[T]) -> Option<T> {
        let index = self.generator.random_range(0..elements.len());
        Some(elements[index].clone())
    }
}

#[cfg(test)]
pub mod game_engine_tests {
    use crate::features::game::domain::game_engine::GameEngine;

    #[test]
    pub fn test_pick_random_element_should_return_an_element() {
        // Given
        let names = vec!["Emi", "Ozzy", "Jill"];
        let seed = 666666;

        let mut game_engine = GameEngine::new(seed);

        // When
        let optional_name = game_engine.pick_random_element(&names);

        // Then
        assert_eq!("Jill", optional_name.unwrap())
    }

    #[test]
    pub fn test_game_engine_should_be_serializable() {
        // Given
        let names = vec!["Emi", "Ozzy", "Jill"];
        let seed = 666666;

        let mut given_game_engine = GameEngine::new(seed);
        given_game_engine.pick_random_element(&names);

        let given_game_engine_state = given_game_engine
            .as_string()
            .expect("The given GameEngine to be serializable.");

        // When
        let mut game_engine = GameEngine::from(&given_game_engine_state)
            .expect("The generator to be deserializable.");

        let optional_name = game_engine.pick_random_element(&names);

        // Then
        assert_eq!("Ozzy", optional_name.unwrap())
    }
}
