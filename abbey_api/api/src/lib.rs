pub mod error;
pub mod features;
pub mod shared;

use axum::Router;
use domain::shared::db::DatabaseClient;
use tokio::net::TcpListener;
use tracing::{Level, info};

use crate::{error::StartupError, shared::ApiContext};

#[derive(Default)]
struct AbbeySetup {
    host: Option<String>,
    port: Option<String>,
    db_url: Option<String>,
    log_level: Option<Level>,
}

impl AbbeySetup {
    fn load_env(mut self) -> Result<Self, StartupError> {
        dotenv::dotenv().ok();

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

    async fn serve_with_config(self) {
        tracing_subscriber::fmt()
            .with_max_level(self.log_level)
            .init();

        DatabaseClient::init(self.db_url)
            .await
            .expect("Failed to create a database connection.");

        let router = Router::new()
            .nest("/api", features::routes())
            .with_state(ApiContext::default());

        let host_url = format!("{}:{}", self.host, self.port);

        let listener = TcpListener::bind(host_url)
            .await
            .expect("Failed to create an API listener.");

        if let Ok(address) = listener.local_addr() {
            info!(
                "{}",
                format!("🌐 The Abbey API is listening on http://{}", address)
            )
        }

        axum::serve(listener, router).await.unwrap();
    }
}
