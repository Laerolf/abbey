use crate::features::{
    monastery::domain::Monastery, player::domain::Player, surroundings::domain::Surroundings,
};

/// The default amount of Monks in a Monastery.
pub const DEFAULT_AMOUNT_OF_MONKS: i32 = 10;

pub struct Game {
    /// The ID of this [`Game`].
    pub id: i32,

    /// The [Player][`crate::features::player::domain::Player`] of this [`Game`].
    pub player: Player,

    /// The [Monastery][`crate::features::monastery::domain::Monastery`] of the this [`Game`].
    pub monastery: Monastery,

    /// The [Surroundings][`crate::features::surroundings::domain::Surroundings`] of the [Monastery][`crate::features::monastery::domain::Monastery`] in this [`Game`].
    pub surroundings: Surroundings,
}

impl Game {
    /// Creates a new [`Game`].
    pub fn new(id: i32, player: Player, monastery: Monastery, surroundings: Surroundings) -> Self {
        Self {
            id,
            player,
            monastery,
            surroundings,
        }
    }
}
