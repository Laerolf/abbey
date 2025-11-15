use sea_orm::{Database, DatabaseConnection};
use std::sync::OnceLock;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

/// Represents a [`DatabaseConnection`] pool.
pub struct DatabasePool {}

impl DatabasePool {
    /// Creates a new [`DatabaseConnection`].
    async fn create_db_connection(db_url: String) -> DatabaseConnection {
        Database::connect(db_url)
            .await
            .expect("Could not connect to database.")
    }

    /// Initializes the [`DatabasePool`] by creating a [`DatabaseConnection`].
    pub async fn init(db_url: impl Into<String>) -> Result<(), DatabaseConnection> {
        DB.set(Self::create_db_connection(db_url.into()).await)
    }

    /// Gets a [`DatabaseConnection`].
    pub fn instance() -> &'static DatabaseConnection {
        DB.get().expect("Failed to get a database connection.")
    }
}
