use axum::Router;
use domain::features::{
    actor::service::MonkService, auth::service::AuthenticationService, game::service::GameService,
    monastery::service::MonasteryService, output::service::ResourceService,
    player::service::PlayerService, process::service::CyclicProcessService,
    skill::service::SkillService, source::service::SourceService,
    surroundings::service::SurroundingsService, user::service::UserService,
};

/// The context of this API.
#[derive(Clone)]
pub struct ApiContext {
    pub authentication_service: AuthenticationService,
}

impl Default for ApiContext {
    /// Creates a new [`ApiContext`].
    fn default() -> Self {
        let player_service = PlayerService::default();
        let monk_service = MonkService::default();
        let skill_service = SkillService::default();
        let source_service = SourceService::default();
        let cyclic_process_service = CyclicProcessService::default();
        let resource_service = ResourceService::default();

        let monastery_service = MonasteryService::new(monk_service, skill_service);
        let surroundings_service =
            SurroundingsService::new(source_service, cyclic_process_service, resource_service);
        let game_service = GameService::new(
            monastery_service,
            player_service.clone(),
            surroundings_service,
        );

        let user_service = UserService::new(game_service);
        let authentication_service = AuthenticationService::new(user_service);

        Self {
            authentication_service,
        }
    }
}

/// Represents a API feature.
pub trait ApiFeature {
    /// Returns the [Routes][`Router<ApiContext>`] of this feature.
    fn routes() -> Router<ApiContext>;
}
