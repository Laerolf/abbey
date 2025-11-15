mod error;
mod features;
mod shared;

use std::env::VarError;

use axum::Router;
use domain::shared::db::DatabasePool;
use tokio::net::TcpListener;
use tracing::{Level, event};

use crate::{error::ApiError, shared::ApiContext};

#[derive(Default)]
pub struct AbbeySetup {
    host: Option<String>,
    port: Option<String>,
    db_url: Option<String>,
    log_level: Option<Level>,
}

impl AbbeySetup {
    fn load_env(mut self) -> Result<Self, VarError> {
        dotenv::dotenv().ok();

        self.db_url = Some(std::env::var("DATABASE_URL")?);
        self.host = Some(std::env::var("HOST")?);
        self.port = Some(std::env::var("PORT")?);

        Ok(self)
    }

    pub fn build(self) -> Result<Abbey, ApiError> {
        Ok(Abbey {
            host: self.host.ok_or(ApiError::MissingHost)?,
            port: self.port.ok_or(ApiError::MissingPort)?,
            db_url: self.db_url.ok_or(ApiError::MissingDbUrl)?,
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
    pub fn setup() -> AbbeySetup {
        AbbeySetup::default()
    }

    pub async fn serve() {
        let abbey = Self::setup()
            .load_env()
            .expect("Failed to load configuration from environment")
            .build()
            .expect("Failed to build Abbey server");

        abbey.serve_with_config().await;
    }

    pub async fn serve_with_config(self) {
        tracing_subscriber::fmt()
            .with_max_level(self.log_level)
            .init();

        // TODO: Move this to the api subproject and share with the service constructors in the ApiState
        DatabasePool::init(&self.db_url)
            .await
            .expect("Failed to create a database connection.");

        let router = Router::new()
            .nest("/api", features::routes())
            .with_state(ApiContext::new());

        let host_url = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(host_url)
            .await
            .expect("Failed to create an API listener.");

        if let Ok(address) = listener.local_addr() {
            event!(
                Level::INFO,
                "{}",
                &format!("🌐 The Abbey API is listening on http://{}", address)
            );
        }

        axum::serve(listener, router).await.unwrap();
    }
}
