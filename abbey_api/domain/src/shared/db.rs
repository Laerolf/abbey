use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::OnceLock;

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

/// Represents a database client.
pub struct DatabaseClient;

impl DatabaseClient {
    /// Creates a new [`DatabaseConnection`].
    async fn create_db_connection(db_url: String) -> DatabaseConnection {
        let mut opt = ConnectOptions::new(db_url);
        opt.sqlx_logging(false);
        opt.max_connections(10);

        Database::connect(opt)
            .await
            .expect("Could not connect to database.")
    }

    /// Initializes the [`DatabasePool`] by creating a [`DatabaseConnection`].
    pub async fn init(db_url: String) -> Result<(), DatabaseConnection> {
        DB.set(Self::create_db_connection(db_url).await)
    }

    /// Gets a [`DatabaseConnection`].
    pub fn get_connection() -> &'static DatabaseConnection {
        DB.get().expect("Failed to get a database connection.")
    }
}
