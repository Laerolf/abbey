mod auth;
mod games;

use axum::Router;

use crate::{
    features::auth::Feature,
    shared::{ApiContext, ApiFeature},
};

pub fn routes() -> Router<ApiContext> {
    Router::new().nest("/auth", Feature::routes())
}
