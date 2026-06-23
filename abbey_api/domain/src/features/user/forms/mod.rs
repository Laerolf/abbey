/// Represents a User blueprint.
#[derive(Clone)]
pub struct UserBlueprint {
    /// The email address of the new User.
    pub email: String,
    /// The password of the new User.
    pub password: String,
}

impl UserBlueprint {
    /// Creates a new [`UserBlueprint`].
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }
}
