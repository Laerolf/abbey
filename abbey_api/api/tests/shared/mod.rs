use std::{collections::HashMap, sync::Arc};

use api::{
    error::StartupError,
    features::{self, openapi},
    shared::ApiContext,
};
use axum::{
    Router,
    body::Body,
    http::{Method, Request, header},
    response::Response,
};
use domain::features::auth::domain::AuthenticationTokens;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde_json::Value;
use tower::ServiceExt;
use tower_cookies::CookieManagerLayer;
use tracing::level_filters::LevelFilter;
use utoipa_swagger_ui::SwaggerUi;

pub mod fixtures;
pub mod utils;

#[derive(Default)]
struct TestSetup {
    db_url: String,
}

impl TestSetup {
    fn load_env(mut self) -> Result<Self, StartupError> {
        dotenvy::from_filename(".env.test").ok();

        self.db_url = std::env::var("DATABASE_URL").map_err(|_error| StartupError::MissingDbUrl)?;

        Ok(self)
    }

    async fn refresh_migrations(&self, db: &DatabaseConnection) {
        migration::Migrator::refresh(db)
            .await
            .expect("Failed to refresh database migrations");
    }

    pub async fn build(self) -> Result<TestApp, StartupError> {
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::DEBUG)
            .with_test_writer()
            .try_init()
            .ok();

        let mut opt = ConnectOptions::new(&self.db_url);
        opt.sqlx_logging(false);

        let db_connection = Database::connect(opt)
            .await
            .expect("Failed to create a database connection.");

        self.refresh_migrations(&db_connection).await;

        let context = ApiContext::new(Arc::new(db_connection));

        let router = Router::new()
            .merge(SwaggerUi::new("/openapi").url("/openapi.json", openapi()))
            .nest("/api", features::routes())
            .layer(CookieManagerLayer::new())
            .with_state(context.clone());

        Ok(TestApp { router, context })
    }
}

pub struct TestApp {
    router: Router,
    pub context: ApiContext<DatabaseConnection>,
}

impl TestApp {
    /// Creates a new [`TestApp`].
    pub async fn new() -> TestApp {
        TestApp::create().await
    }

    async fn create() -> Self {
        TestSetup::default()
            .load_env()
            .expect("Failed to create the test app setup.")
            .build()
            .await
            .expect("Failed to setup the test app.")
    }

    pub fn post(&self, uri: &str) -> RequestBuilder {
        RequestBuilder::new(&self.router, Method::POST, uri)
    }

    pub fn get(&self, uri: &str) -> RequestBuilder {
        RequestBuilder::new(&self.router, Method::GET, uri)
    }
}

pub struct RequestBuilder {
    router: Router,
    http_method: Method,
    uri: String,
    body: Body,
    headers: HashMap<String, String>,
    cookies: Vec<String>,
}

impl RequestBuilder {
    fn new(router: &Router, http_method: impl Into<Method>, uri: impl Into<String>) -> Self {
        let mut default_headers = HashMap::new();
        default_headers.insert(
            header::CONTENT_TYPE.to_string(),
            "application/json".to_string(),
        );

        Self {
            router: router.clone(),
            http_method: http_method.into(),
            uri: uri.into(),
            body: Body::empty(),
            headers: default_headers,
            cookies: Vec::new(),
        }
    }

    pub fn body(mut self, body: &Value) -> Self {
        self.body = Body::from(serde_json::to_string(body).unwrap());
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn bearer(self, token: impl Into<String>) -> Self {
        self.header(
            AuthenticationTokens::SessionToken.header_name(),
            format!("Bearer {}", token.into()),
        )
    }

    pub fn cookie(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.cookies
            .push(format!("{}={}", name.into(), value.into()));
        self
    }

    pub async fn send(self) -> Response<Body> {
        let mut request = Request::builder().method(self.http_method).uri(self.uri);

        for (key, value) in self.headers {
            request = request.header(key, value);
        }

        if !self.cookies.is_empty() {
            let cookie_header = self.cookies.join("; ");
            request = request.header("cookie", cookie_header);
        }

        self.router
            .oneshot(request.body(self.body).unwrap())
            .await
            .unwrap()
    }
}
