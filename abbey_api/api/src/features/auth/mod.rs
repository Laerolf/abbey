use axum::{Json, Router, extract::State, routing::post};
use domain::features::{
    auth::{dto::RegisterUserRequest, mapper::AuthenticationDtoMapper},
    user::{dto::UserDto, repository::UserWithRelations},
};
use tracing::error;

use crate::{error::AppError, shared::ApiFeature};

pub struct Feature;

impl ApiFeature for Feature {
    fn routes() -> axum::Router<crate::shared::ApiContext> {
        Router::new().route("/register", post(register))
    }
}

#[axum::debug_handler]
async fn register(
    State(context): State<crate::shared::ApiContext>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<UserDto>, AppError> {
    let registration_form = AuthenticationDtoMapper::to_registration_form(payload);

    let UserWithRelations { user, .. } = context
        .authentication_service
        .register(registration_form)
        .await
        .inspect_err(|error| {
            error!(?error);
        })?;

    Ok(Json(UserDto {
        user_id: user.id,
        email: user.email,
    }))
}
