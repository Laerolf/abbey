/// Represents a [User][`super::domain::User`] creation form.
#[derive(Clone)]
pub struct UserCreationForm {
    /// The email address of the new [User][`super::domain::User`].
    pub email: String,
}

impl UserCreationForm {
    /// Creates a new [`UserCreationForm`].
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }
}
