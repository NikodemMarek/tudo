use chrono::NaiveDateTime;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tasks1::TasksHub;
use tui::widgets::{ListState, TableState};

pub struct App {
    hub: TasksHub<HttpsConnector<HttpConnector>>,

    pub should_quit: bool,

    pub tasklists: Vec<Tasklist>,
    pub active_tasklist: usize,
    pub tasks_state: TableState,
}
impl App {
    pub fn new(hub: TasksHub<HttpsConnector<HttpConnector>>) -> Self {
        Self {
            hub,
            should_quit: false,
            tasklists: Vec::new(),
            active_tasklist: 0,
            tasks_state: TableState::default(),
        }
    }

    pub async fn load(&mut self) -> anyhow::Result<()> {
        self.tasklists = api::tasklists::load(&self.hub).await?;
        for tasklist in self.tasklists.iter_mut() {
            tasklist.load(&self.hub).await?;
        }

        Ok(())
    }

    pub fn on_tick(&mut self) {}
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn tasklists_next(&mut self) {
        self.tasks_state = TableState::default();

        self.active_tasklist = (self.active_tasklist + 1) % self.tasklists.len();
    }
    pub fn tasklists_previous(&mut self) {
        self.tasks_state = TableState::default();

        if self.active_tasklist > 0 {
            self.active_tasklist -= 1;
        } else {
            self.active_tasklist = self.tasklists.len() - 1;
        }
    }

    pub fn active_tasklist(&self) -> Option<&Tasklist> {
        self.tasklists.get(self.active_tasklist)
    }

    pub fn tasks_next(&mut self) {
        if let Some(tasklist) = self.active_tasklist() {
            let i = match self.tasks_state.selected() {
                Some(i) => {
                    if i >= tasklist.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };

            self.tasks_state.select(Some(i));
        } else {
            self.tasks_state.select(None);
        }
    }
    pub fn tasks_previous(&mut self) {
        if let Some(tasklist) = self.active_tasklist() {
            let i = match self.tasks_state.selected() {
                Some(i) => {
                    if i == 0 {
                        tasklist.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };

            self.tasks_state.select(Some(i));
        } else {
            self.tasks_state.select(None);
        }
    }
}
use std::fmt;

use crate::api;
impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("should_quit", &self.should_quit)
            .field("tasklists", &self.tasklists)
            .field("active_tasklist", &self.active_tasklist)
            .field("active_task", &self.tasks_state)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct Tasklist {
    pub id: String,
    pub title: String,
    pub tasks: Vec<Task>,
}
impl Tasklist {
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title,
            tasks: Vec::new(),
        }
    }

    pub async fn load(
        &mut self,
        hub: &TasksHub<HttpsConnector<HttpConnector>>,
    ) -> anyhow::Result<()> {
        self.tasks = api::tasks::load(hub, self.id.clone()).await?;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    pub fn get(&self, index: usize) -> Option<&Task> {
        self.tasks.get(index)
    }
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub due: Option<NaiveDateTime>,
}
impl Task {
    pub fn new(id: &str, title: &str, due: Option<NaiveDateTime>) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            due,
        }
    }
}
