mod app;
mod components;
#[path = "providers/google.rs"]
mod google;
mod provider;
mod setup;
mod timestamps;

use std::time::Duration;

extern crate google_tasks1 as tasks1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = google::setup().await?;
    setup::run(Duration::from_millis(250), app).await?;

    Ok(())
}
