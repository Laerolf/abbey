mod auth;
mod catalog;
mod cyclic_processes;
mod games;
mod users;

use axum::Router;
use sea_orm::DatabaseConnection;
use utoipa::{
    OpenApi,
    openapi::{
        ComponentsBuilder, InfoBuilder, OpenApi as OpenApiDoc, OpenApiBuilder,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    },
};

use crate::{
    features::{
        auth::{AuthApiDoc, Feature as AuthFeature},
        catalog::{CatalogApiDoc, Feature as CatalogFeature},
        cyclic_processes::{CyclicProcessesApiDoc, Feature as CyclicProcessFeature},
        games::{Feature as GameFeature, GamesApiDoc},
        users::{Feature as UserFeature, UsersApiDoc},
    },
    shared::{ApiContext, ApiFeature},
};

pub fn openapi() -> OpenApiDoc {
    OpenApiBuilder::new()
        .info(InfoBuilder::new().title("Abbey API").build())
        .components(Some(
            ComponentsBuilder::new()
                .security_scheme(
                    "api_auth",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .build(),
        ))
        .build()
        .merge_from(AuthApiDoc::openapi())
        .merge_from(UsersApiDoc::openapi())
        .merge_from(GamesApiDoc::openapi())
        .merge_from(CyclicProcessesApiDoc::openapi())
        .merge_from(CatalogApiDoc::openapi())
}

pub fn routes() -> Router<ApiContext<DatabaseConnection>> {
    Router::new()
        .nest("/auth", AuthFeature::routes())
        .nest("/users", UserFeature::routes())
        .nest("/games", GameFeature::routes())
        .nest("/cyclic-processes", CyclicProcessFeature::routes())
        .nest("/catalog", CatalogFeature::routes())
}
