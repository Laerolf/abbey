use time::Duration;

use crate::{
    features::{
        actor::domain::monk::Monk,
        monastery::domain::Monastery,
        output::domain::resource::{Category, Resource},
        source::domain::Source,
        surroundings::domain::Surroundings,
    },
    shared::{error::DomainError, DomainFactory},
};

/// The default amount of Monks in a Monastery.
pub const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

pub struct Game {
    /// The Monastery of the this game.
    pub monastery: Monastery,

    /// The surroundings of the monastery in this game.
    pub surroundings: Surroundings,
}

impl Game {
    pub fn new(monastery: Monastery, surroundings: Surroundings) -> Self {
        Self {
            monastery,
            surroundings,
        }
    }
}

#[derive(Default)]
pub struct GameCreationFactory {}

impl GameCreationFactory {
    /// Creates the Monastery for a new game.
    fn create_monastery(&self) -> Result<Monastery, Box<dyn DomainError>> {
        // TODO: Give a Monk a proper name
        let monks: Vec<Monk> = (0..DEFAULT_AMOUNT_OF_MONKS)
            .map(|_index: i32| Monk::new("Maurits"))
            .collect();

        Ok(Monastery::new(monks))
    }

    /// Creates the surroundings for a new game.
    fn create_surroundings(&self) -> Result<Surroundings, Box<dyn DomainError>> {
        const ONE_MINUTE_CYCLE_DURATION: Duration = Duration::minutes(1);

        let beach_resources = vec![Resource::new("sand", Category::Material)];

        let the_beach: Source = Source::new("Beach", beach_resources, ONE_MINUTE_CYCLE_DURATION)?;

        let sources: Vec<Source> = vec![the_beach];

        Ok(Surroundings::new(sources))
    }
}

impl DomainFactory<Game> for GameCreationFactory {
    /// Creates a new game.
    fn run(&self) -> Result<Game, Box<dyn DomainError>> {
        let monastery = self.create_monastery()?;
        let surroundings = self.create_surroundings()?;

        let new_game = Game::new(monastery, surroundings);

        Ok(new_game)
    }
}
