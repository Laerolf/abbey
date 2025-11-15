use axum::{Router, routing::get};

use crate::shared::ApiFeature;

pub struct Feature;

impl ApiFeature for Feature {
    fn routes() -> axum::Router<crate::shared::ApiContext> {
        Router::new().route("/hello-world", get(|| async { "Hello, World!" }))
    }
}
