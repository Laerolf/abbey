pub mod error;
pub mod features;
pub mod shared;

use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
};

use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::{Level, info};
use utoipa_swagger_ui::SwaggerUi;

use crate::{error::StartupError, features::openapi, shared::ApiContext};

#[derive(Default)]
struct AbbeySetup {
    host: Option<String>,
    port: Option<String>,
    db_url: Option<String>,
    log_level: Option<Level>,
}

impl AbbeySetup {
    fn load_env(mut self) -> Result<Self, StartupError> {
        dotenvy::from_filename(".env").ok();

        self.db_url =
            Some(std::env::var("DATABASE_URL").map_err(|_error| StartupError::MissingDbUrl)?);
        self.host = Some(std::env::var("HOST").map_err(|_error| StartupError::MissingHost)?);
        self.port = Some(std::env::var("PORT").map_err(|_error| StartupError::MissingPort)?);

        Ok(self)
    }

    pub fn build(self) -> Result<Abbey, StartupError> {
        Ok(Abbey {
            host: self.host.ok_or(StartupError::MissingHost)?,
            port: self.port.ok_or(StartupError::MissingPort)?,
            db_url: self.db_url.ok_or(StartupError::MissingDbUrl)?,
            log_level: self.log_level.unwrap_or(Level::INFO),
        })
    }
}

pub struct Abbey {
    host: String,
    port: String,
    db_url: String,
    log_level: Level,
}

impl Abbey {
    fn setup() -> AbbeySetup {
        AbbeySetup::default()
    }

    pub async fn serve() -> Result<(), StartupError> {
        let abbey = Self::setup().load_env()?.build()?;

        abbey.serve_with_config().await;

        Ok(())
    }

    async fn serve_with_config(&self) {
        tracing_subscriber::fmt()
            .with_max_level(self.log_level)
            .init();

        let mut opt = ConnectOptions::new(&self.db_url);
        opt.sqlx_logging(false);

        let db_connection = Database::connect(opt)
            .await
            .expect("Failed to create a database connection.");

        // TODO: Adjust accordingly
        let cors = CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_credentials(true)
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

        let router = Router::new()
            .merge(SwaggerUi::new("/openapi").url("/openapi.json", openapi()))
            .nest("/api", features::routes())
            .layer(CookieManagerLayer::new())
            .layer(cors)
            .with_state(ApiContext::new(Arc::new(db_connection)));

        let host_url = format!("{}:{}", self.host, &self.port);

        let listener = TcpListener::bind(host_url)
            .await
            .expect("Failed to create an API listener.");

        if let Ok(address) = listener.local_addr() {
            info!(
                "🌐 The Abbey API is listening on http://{} (OpenAPI: http://{}/openapi)",
                address, address
            );
        }

        axum::serve(listener, router).await.unwrap();
    }
}
