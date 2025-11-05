use time::OffsetDateTime;

/// Conversion ratio: 1 real second = 1 game minute = 60 game seconds
/// Each real second represents 1 game minute (60 game seconds)
const GAME_SECONDS_PER_REAL_SECOND: i64 = 60;

/// Represents the clock of the [Game][`crate::features::game::domain::Game`].
pub struct GameClock {}

impl GameClock {
    /// Gets current [Game][`crate::features::game::domain::Game`] time as an OffsetDateTime.
    pub fn now() -> OffsetDateTime {
        let real_seconds_since_epoch = OffsetDateTime::now_utc().unix_timestamp();
        let game_seconds_since_epoch = real_seconds_since_epoch * GAME_SECONDS_PER_REAL_SECOND;

        OffsetDateTime::from_unix_timestamp(game_seconds_since_epoch)
            .expect("Game time calculation resulted in an invalid timestamp.")
    }
}
