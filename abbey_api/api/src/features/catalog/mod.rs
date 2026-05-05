use axum::{Json, Router, extract::State, routing::get};
use domain::features::{game::dto::GameDto, skill::dto::SkillDto};
use sea_orm::DatabaseConnection;
use tracing::error;
use utoipa::OpenApi;

use crate::{
    error::AppError,
    shared::{ApiContext, ApiFeature},
};

pub struct Feature;

impl ApiFeature for Feature {
    fn routes() -> axum::Router<ApiContext<DatabaseConnection>> {
        Router::new().route("/skills", get(get_all_skills))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_all_skills),
    tags((name = "Catalog"))
)]
pub struct CatalogApiDoc;

/// Gets all existing Skills.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/catalog/skills",
    responses(
        (status = 200, description = "The skills were found.", body = Vec<SkillDto>)
    ),
    tag = "Catalog"
)]
async fn get_all_skills(
    State(context): State<ApiContext<DatabaseConnection>>,
) -> Result<Json<Vec<SkillDto>>, AppError> {
    let all_skills = context
        .catalog_service
        .get_all_skills(context.db_connection())
        .await
        .inspect_err(|error| {
            error!(?error);
        })?;

    Ok(Json(all_skills.into_iter().map(SkillDto::from).collect()))
}
