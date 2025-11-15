use domain::shared::db::DatabasePool;

pub async fn main() {
    dotenv::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .expect("Failed to find the database url in the environment variables.");

    DatabasePool::init(db_url)
        .await
        .expect("Failed to create a database connection.");

    println!("Hello, world!");
}
