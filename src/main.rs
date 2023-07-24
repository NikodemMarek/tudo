mod api;
mod app;
mod components;
mod setup;
mod timestamps;

use app::App;

extern crate google_tasks1 as tasks1;
use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::enable_raw_mode,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub async fn run(tick_rate: Duration, app: App) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let res = run_app(&mut terminal, app, tick_rate).await;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> anyhow::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| components::ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('l') => app.tasklists_next(),
                    KeyCode::Char('h') => app.tasklists_previous(),
                    KeyCode::Char('j') => app.tasks_next(),
                    KeyCode::Char('k') => app.tasks_previous(),
                    _ => {}
                };
            };
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let hub = setup::setup().await?;
    let mut app = App::new(hub);
    app.load().await?;

    run(Duration::from_millis(250), app).await?;

    Ok(())
}
