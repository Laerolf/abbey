use sea_orm::{Database, DatabaseConnection};
use std::sync::OnceLock;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

/// Represents a [`DatabaseConnection`] pool.
pub struct DatabasePool {}

impl DatabasePool {
    /// Creates a new [`DatabaseConnection`].
    async fn create_db_connection() -> DatabaseConnection {
        let db_url = std::env::var("DATABASE_URL")
            .expect("Failed to find the database url in the environment variables.");

        Database::connect(db_url)
            .await
            .expect("Could not connect to database.")
    }

    /// Initializes the [`DatabasePool`] by creating a [`DatabaseConnection`].
    pub async fn init() -> Result<(), DatabaseConnection> {
        DB.set(Self::create_db_connection().await)
    }

    /// Gets a [`DatabaseConnection`].
    pub fn instance() -> &'static DatabaseConnection {
        DB.get().expect("Failed to get a database connection.")
    }
}
