use time::Duration;

use crate::{
    features::{
        self,
        output::domain::resource::{Category, Resource},
        source::domain::Source,
        surroundings::domain::Surroundings,
    },
    shared::{error::DomainError, DomainFactory},
};

pub struct Game {
    /// The surroundings of the abbey in this game.
    pub surroundings: Surroundings,
}

impl Game {
    pub fn new(surroundings: Surroundings) -> Self {
        Self { surroundings }
    }
}

pub struct GameCreationFactory {}

impl GameCreationFactory {
    /// Creates the surroundings for a new game.
    fn create_surroundings() -> Result<Surroundings, Box<dyn DomainError>> {
        const ONE_MINUTE_CYCLE_DURATION: Duration = Duration::minutes(1);

        let beach_resources = vec![Resource::new("sand", Category::Material)];

        let the_beach: Source = Source::new("Beach", beach_resources, ONE_MINUTE_CYCLE_DURATION)?;

        let sources: Vec<Source> = vec![the_beach];

        Ok(Surroundings::new(sources))
    }
}

impl DomainFactory<Game> for GameCreationFactory {
    /// Creates a new game.
    fn run() -> Result<Game, Box<dyn DomainError>> {
        let surroundings = features::game::domain::GameCreationFactory::create_surroundings()?;

        let new_game = Game::new(surroundings);

        Ok(new_game)
    }
}
