use api::Abbey;

#[tokio::main]
async fn main() {
    if let Err(error) = Abbey::serve().await {
        eprintln!("Startup failed: {}", error);
        std::process::exit(1);
    }
}
