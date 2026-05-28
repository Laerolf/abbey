use axum::{Json, Router, extract::State, routing::get};
use domain::features::{auth::domain::session_token::SessionToken, user::dto::UserDto};
use sea_orm::DatabaseConnection;
use tracing::error;
use utoipa::OpenApi;

use crate::{
    error::AppError,
    shared::{ApiContext, ApiFeature},
};

pub struct Feature;

impl ApiFeature for Feature {
    fn routes() -> Router<ApiContext<DatabaseConnection>> {
        Router::new().route("/me", get(get_session_user))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_session_user),
    tags((name = "Users"))
)]
pub struct UsersApiDoc;

/// Gets a User for the provided session.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/users/me",
    responses(
        (status = 200, description = "The user was found.", body = UserDto),
        (
            status = 401,
            description = "Failed to authenticate a user.",
            body = AppError,
            example = json!({
                "message": "Failed to authenticate a user.",
                "code": "error.authentication.authenticate",
            })
        )
    ),
    security(("api_auth" = [])),
    tag = "Users"
)]
async fn get_session_user(
    State(context): State<ApiContext<DatabaseConnection>>,
    session_token: SessionToken,
) -> Result<Json<UserDto>, AppError> {
    let user = context
        .user_query_service
        .get_by_id(session_token.user_id(), context.db_connection())
        .await
        .inspect_err(|error| {
            error!(?error);
        })?;

    Ok(Json(UserDto::from(user)))
}
