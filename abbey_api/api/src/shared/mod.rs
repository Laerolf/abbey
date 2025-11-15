use axum::Router;
use domain::features::player::service::PlayerService;

/// The context of this API.
#[derive(Clone)]
pub struct ApiContext {
    pub player_service: PlayerService,
}

impl ApiContext {
    /// Creates a new [`ApiContext`].
    pub fn new() -> Self {
        Self {
            player_service: PlayerService::default(),
        }
    }
}

/// Represents a API feature.
pub trait ApiFeature {
    /// Returns the [Routes][`Router<ApiContext>`] of this feature.
    fn routes() -> Router<ApiContext>;
}
