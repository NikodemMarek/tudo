mod app;
mod components;
#[path = "api/google.rs"]
mod google;
mod setup;
mod timestamps;

use std::time::Duration;

use app::App;

use self::setup::run;

extern crate google_tasks1 as tasks1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = google::setup().await?;
    run(Duration::from_millis(250), app).await?;

    Ok(())
}
