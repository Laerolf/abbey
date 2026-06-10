use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Represents a Game engine.
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

    /// Creates a [`GameEngine`].
    pub fn from(generator: ChaCha8Rng) -> Self {
        Self { generator }
    }

    pub fn generator(&self) -> &ChaCha8Rng {
        &self.generator
    }

    /// Picks a random element from the provided collections of elements.
    pub fn pick_random_element<T: Clone>(&mut self, elements: &[T]) -> Option<T> {
        let index = self.generator.random_range(0..elements.len());
        Some(elements[index].clone())
    }
}

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

        let given_game_engine_state = serde_json::to_string(given_game_engine.generator())
            .expect("The given GameEngine to be serializable.");

        // When
        let generator = serde_json::from_str(&given_game_engine_state)
            .expect("The generator to be deserializable.");

        let mut game_engine = GameEngine::from(generator);
        let optional_name = game_engine.pick_random_element(&names);

        // Then
        assert_eq!("Ozzy", optional_name.unwrap())
    }
}
