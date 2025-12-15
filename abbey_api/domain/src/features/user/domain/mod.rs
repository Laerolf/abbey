use std::{fmt::Display, str::FromStr};

use crate::features::game::domain::Game;

/// Represents a user.
pub struct User {
    /// The ID of the [`User`].
    pub id: i32,
    /// The email address of the [`User`].
    pub email: String,
    /// The status of the [`User`].
    pub status: Status,
    /// The [Game] of the [`User`].
    pub game: Option<Game>,
}

impl User {
    /// Creates a new [`User`].
    pub fn new(id: i32, email: impl Into<String>, status: Status, game: Option<Game>) -> Self {
        Self {
            id,
            email: email.into(),
            status,
            game,
        }
    }
}

/// Represents a [`User`] status.
#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    /// The [`User`] has been created but not yet verified.
    New,
    /// The [`User`] has been verified.
    Verfied,
}

impl Status {
    /// Returns a string representing the [Status].
    fn as_str(&self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Verfied => "verified",
        }
    }
}

impl FromStr for Status {
    type Err = String;

    /// Returns the [Status] represented by the provided value.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new" => Ok(Self::New),
            "verified" => Ok(Self::Verfied),
            _ => Err(format!("Invalid status: '{}'", s)),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
