mod player;

use axum::Router;

use crate::{
    features::player::Feature,
    shared::{ApiContext, ApiFeature},
};

pub fn routes() -> Router<ApiContext> {
    Router::new().nest("/players", Feature::routes())
}
