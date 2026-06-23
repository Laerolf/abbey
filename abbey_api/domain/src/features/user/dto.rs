use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{features::user::domain::User, shared::DomainElement};

/// Represents a [User] DTO.
#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq)]
pub struct UserDto {
    /// The ID of the user.
    #[schema(example = 666)]
    pub id: i32,
    /// The email address of the user.
    #[schema(example = "ozzy@in.heaven")]
    pub email: String,
}

impl UserDto {
    /// Creates a new [UserDto].
    pub fn from(user: User) -> Self {
        Self {
            id: user.id().unwrap(),
            email: user.email().to_string(),
        }
    }
}
