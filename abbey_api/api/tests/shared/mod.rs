use std::sync::OnceLock;

use api::{error::StartupError, features, shared::ApiContext};
use axum::{Router, body::Body, http::Request, response::Response};
use domain::shared::db::DatabaseClient;
use migration::MigratorTrait;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, FromQueryResult, Statement};
use tower::ServiceExt;
use tracing::info;

static TEST_APP: OnceLock<TestApp> = OnceLock::new();

#[derive(FromQueryResult)]
struct TableName {
    tablename: String,
}

#[derive(Default)]
struct TestSetup {
    db_url: Option<String>,
}

impl TestSetup {
    fn load_env(mut self) -> Result<Self, StartupError> {
        dotenv::from_filename(".env.test").ok();

        self.db_url =
            Some(std::env::var("DATABASE_URL").map_err(|_error| StartupError::MissingDbUrl)?);

        Ok(self)
    }

    async fn refresh_migrations(&self, db: &DatabaseConnection) {
        migration::Migrator::refresh(db)
            .await
            .expect("Failed to refresh database migrations");
    }

    pub async fn build(self) -> Result<TestApp, StartupError> {
        tracing_subscriber::fmt().with_test_writer().try_init().ok();

        DatabaseClient::init(
            self.db_url
                .as_ref()
                .expect("A database URL is required.")
                .to_string(),
        )
        .await
        .expect("Failed to create a database connection.");

        self.refresh_migrations(DatabaseClient::get_connection())
            .await;

        let router = Router::new()
            .nest("/api", features::routes())
            .with_state(ApiContext::default());

        Ok(TestApp { router })
    }
}

#[derive(Debug)]
pub struct TestApp {
    router: Router,
}

impl TestApp {
    /// Gets a [`TestApp`].
    pub async fn instance() -> &'static TestApp {
        if TEST_APP.get().is_none() {
            TEST_APP
                .set(TestApp::create().await)
                .expect("Failed to create the test app.");
        }

        TEST_APP.get().unwrap()
    }

    async fn create() -> Self {
        TestSetup::default()
            .load_env()
            .expect("Failed to create the test app setup.")
            .build()
            .await
            .expect("Failed to setup the test app.")
    }

    pub async fn reset_database(&self) {
        let db = DatabaseClient::get_connection();

        info!("RESETING DATABASE [START]");

        let tables: Vec<TableName> = db
            .query_all(Statement::from_string(
                DbBackend::Postgres,
                "SELECT tablename FROM pg_tables WHERE schemaname = 'public'".to_owned(),
            ))
            .await
            .expect("Failed to get table names")
            .iter()
            .filter_map(|row| TableName::from_query_result(row, "").ok())
            .collect();

        for table in tables {
            if table.tablename == "seaql_migrations" {
                continue;
            }

            db.execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    "TRUNCATE TABLE \"{}\" RESTART IDENTITY CASCADE",
                    table.tablename
                ),
            ))
            .await
            .ok();
        }

        info!("RESETING DATABASE [START]");
    }

    pub async fn post(&self, uri: &str, body: serde_json::Value) -> Response<Body> {
        self.router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap()
    }
}
