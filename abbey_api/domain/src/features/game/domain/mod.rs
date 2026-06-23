use time::OffsetDateTime;

use crate::{
    features::{
        game::{domain::game_engine::GameEngine, error::GameErrorKind},
        monastery::domain::Monastery,
        player::domain::Player,
        surroundings::domain::Surroundings,
    },
    shared::{DomainElement, error::DomainError},
};

pub mod game_engine;

/// The default amount of Monks in a Monastery.
pub const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

#[derive(Clone)]
pub struct Game {
    /// The ID of this [`Game`].
    id: Option<i32>,

    /// The creation date of this [`Game`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`Game`].
    last_updated_at: Option<OffsetDateTime>,

    /// The GameEngine of this [`Game`].
    game_engine: GameEngine,

    /// The Player of this [`Game`].
    player: Player,

    /// The Monastery of the this [`Game`].
    monastery: Monastery,

    /// The Surroundings of the Monastery in this [`Game`].
    surroundings: Surroundings,
}

impl Game {
    /// Creates a [`Game`].
    pub fn new(
        game_engine: GameEngine,
        player: Player,
        monastery: Monastery,
        surroundings: Surroundings,
    ) -> Self {
        Self {
            id: None,
            created_at: None,
            last_updated_at: None,
            game_engine,
            player,
            monastery,
            surroundings,
        }
    }

    /// Restores a [`Game`].
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        game_engine: GameEngine,
        player: Player,
        monastery: Monastery,
        surroundings: Surroundings,
    ) -> Self {
        Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            game_engine,
            player,
            monastery,
            surroundings,
        }
    }

    /// Gets the [GameEngine] of a [`Game`].
    pub fn game_engine(&self) -> &GameEngine {
        &self.game_engine
    }

    /// Gets the [Player] of a [`Game`].
    pub fn player(&self) -> &Player {
        &self.player
    }

    /// Gets the [Monastery] of a [`Game`].
    pub fn monastery(&self) -> &Monastery {
        &self.monastery
    }

    /// Gets the [Surroundings] of a [`Game`].
    pub fn surroundings(&self) -> &Surroundings {
        &self.surroundings
    }
}

impl DomainElement<GameErrorKind> for Game {
    /// Gets the ID of this [`Game`].
    fn id(&self) -> Result<i32, DomainError<GameErrorKind>> {
        self.id
            .ok_or(DomainError::from(GameErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`Game`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`Game`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
    }
}
