#[tokio::main]
async fn main() {
    api::Abbey::serve().await
}
