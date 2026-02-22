mod app;
mod attend;
mod browser;
mod config;
mod scheduler;
mod screenshot;

#[tokio::main]
async fn main() {
    if let Err(e) = app::run().await {
        eprintln!("Application error: {}", e);
    }
}
