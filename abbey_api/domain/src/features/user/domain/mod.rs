use std::{fmt::Display, str::FromStr};

use crate::features::game::domain::Game;

/// Represents a user.
#[derive(Clone)]
pub struct User {
    /// The ID of the [`User`].
    id: Option<i32>,
    /// The email address of the [`User`].
    email: String,
    /// The password of the [`User`].
    password: String,
    /// The status of the [`User`].
    status: Status,
    /// The [Game]s of the [`User`].
    games: Vec<Game>,
}

impl User {
    /// Creates a new [`User`].
    pub fn new(
        email: impl Into<String>,
        password: impl Into<String>,
        status: Status,
        games: Vec<Game>,
    ) -> Self {
        Self {
            id: None,
            email: email.into(),
            password: password.into(),
            status,
            games,
        }
    }

    /// Creates a [`User`].
    pub fn restore(
        id: i32,
        email: impl Into<String>,
        password: impl Into<String>,
        status: Status,
        games: Vec<Game>,
    ) -> Self {
        Self {
            id: Some(id),
            email: email.into(),
            password: password.into(),
            status,
            games,
        }
    }

    /// Gets the ID of the [`User`].
    pub fn id(&self) -> &Option<i32> {
        &self.id
    }

    /// Gets the email address of the [`User`].
    pub fn email(&self) -> &String {
        &self.email
    }

    /// Gets the password of the [`User`].
    pub fn password(&self) -> &String {
        &self.password
    }

    /// Gets the status of the [`User`].
    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Gets the [Games][`Vec<Game>`] of the [`User`].
    pub fn games(&self) -> &Vec<Game> {
        &self.games
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
