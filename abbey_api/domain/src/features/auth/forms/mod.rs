/// Represents a account registration form.
#[derive(Clone)]
pub struct RegistrationForm {
    /// The email address of the new account to create.
    pub email: Option<String>,
}

impl RegistrationForm {
    /// Creates a new [`RegistrationCreationForm`].
    pub fn new(email: Option<impl Into<String>>) -> Self {
        Self {
            email: email.map(|email| email.into()),
        }
    }
}
