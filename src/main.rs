mod app;
mod components;
mod config;
#[path = "providers/google.rs"]
mod google;
mod provider;
mod setup;
mod timestamps;

use std::time::Duration;

extern crate google_tasks1 as tasks1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::get_config()?;

    let app = google::setup(&cfg).await?;
    setup::run(Duration::from_millis(250), app).await?;

    Ok(())
}
