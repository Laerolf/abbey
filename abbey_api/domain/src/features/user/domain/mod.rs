use std::{fmt::Display, str::FromStr};

use time::OffsetDateTime;

use crate::{
    features::{game::domain::Game, user::error::UserErrorKind},
    shared::{DomainElement, error::DomainError},
};

/// Represents a user.
#[derive(Clone)]
pub struct User {
    /// The ID of the [`User`].
    id: Option<i32>,

    /// The creation date of this [`User`].
    created_at: Option<OffsetDateTime>,

    /// The date of the last update of this [`User`].
    last_updated_at: Option<OffsetDateTime>,

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
            created_at: None,
            last_updated_at: None,
            email: email.into(),
            password: password.into(),
            status,
            games,
        }
    }

    /// Creates a [`User`].
    pub fn restore(
        id: i32,
        created_at: OffsetDateTime,
        last_updated_at: Option<OffsetDateTime>,
        email: impl Into<String>,
        password: impl Into<String>,
        status: Status,
        games: Vec<Game>,
    ) -> Self {
        Self {
            id: Some(id),
            created_at: Some(created_at),
            last_updated_at,
            email: email.into(),
            password: password.into(),
            status,
            games,
        }
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

impl DomainElement<UserErrorKind> for User {
    /// Gets the ID of this [`User`].
    fn id(&self) -> Result<i32, DomainError<UserErrorKind>> {
        self.id
            .ok_or(DomainError::from(UserErrorKind::NotPersistedYet))
    }

    /// Gets the [creation date][`OffsetDateTime`] of this [`User`].
    fn created_at(&self) -> &Option<OffsetDateTime> {
        &self.created_at
    }

    /// Gets the [latest update date][`OffsetDateTime`] of this [`User`].
    fn last_updated_at(&self) -> &Option<OffsetDateTime> {
        &self.last_updated_at
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
