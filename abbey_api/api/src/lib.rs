use domain::shared::db::DatabasePool;

pub async fn main() {
    dotenv::dotenv().ok();

    DatabasePool::init()
        .await
        .expect("Failed to create a database connection.");

    println!("Hello, world!");
}
