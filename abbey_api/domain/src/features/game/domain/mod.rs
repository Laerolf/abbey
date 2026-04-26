use crate::features::{
    monastery::domain::Monastery, player::domain::Player, surroundings::domain::Surroundings,
};

/// The default amount of Monks in a Monastery.
pub const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

#[derive(Clone)]
pub struct Game {
    /// The ID of this [`Game`].
    id: Option<i32>,

    /// The [Player][`crate::features::player::domain::Player`] of this [`Game`].
    player: Player,

    /// The [Monastery][`crate::features::monastery::domain::Monastery`] of the this [`Game`].
    monastery: Monastery,

    /// The [Surroundings][`crate::features::surroundings::domain::Surroundings`] of the [Monastery][`crate::features::monastery::domain::Monastery`] in this [`Game`].
    surroundings: Surroundings,
}

impl Game {
    /// Creates a [`Game`].
    pub fn new(player: Player, monastery: Monastery, surroundings: Surroundings) -> Self {
        Self {
            id: None,
            player,
            monastery,
            surroundings,
        }
    }

    /// Restores a [`Game`].
    pub fn restore(
        id: i32,
        player: Player,
        monastery: Monastery,
        surroundings: Surroundings,
    ) -> Self {
        Self {
            id: Some(id),
            player,
            monastery,
            surroundings,
        }
    }

    /// Gets the ID of a [`Game`].
    pub fn id(&self) -> &Option<i32> {
        &self.id
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
