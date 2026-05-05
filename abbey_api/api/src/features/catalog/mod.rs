use axum::{Json, Router, extract::State, routing::get};
use domain::features::{output::dto::ResourceDto, skill::dto::SkillDto};
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
        Router::new()
            .route("/skills", get(get_all_skills))
            .route("/resources", get(get_all_resources))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get_all_skills, get_all_resources),
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

/// Gets all existing Resources.
#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/api/catalog/resources",
    responses(
        (status = 200, description = "The resources were found.", body = Vec<ResourceDto>)
    ),
    tag = "Catalog"
)]
async fn get_all_resources(
    State(context): State<ApiContext<DatabaseConnection>>,
) -> Result<Json<Vec<ResourceDto>>, AppError> {
    let all_resources = context
        .catalog_service
        .get_all_resources(context.db_connection())
        .await
        .inspect_err(|error| {
            error!(?error);
        })?;

    Ok(Json(
        all_resources.into_iter().map(ResourceDto::from).collect(),
    ))
}
