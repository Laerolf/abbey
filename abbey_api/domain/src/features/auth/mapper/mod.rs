use crate::features::{
    auth::{
        dto::{RegisterResponse, RegisterUserRequest},
        forms::RegistrationForm,
    },
    user::domain::User,
};

/// Represents an element that maps authentication elements.
pub struct AuthenticationDtoMapper;

impl AuthenticationDtoMapper {
    /// Maps a [RegisterUserRequest] to a [`RegistrationForm`].
    pub fn to_registration_form(request: RegisterUserRequest) -> RegistrationForm {
        RegistrationForm::new(request.email)
    }

    /// Maps a [`User`] to a [RegisterResponse].
    pub fn to_response(entity: User) -> RegisterResponse {
        RegisterResponse {
            user_id: entity.id,
            email: entity.email,
        }
    }
}
