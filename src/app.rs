use tui::widgets::TableState;

use crate::{provider::Provider, timestamps::TimestampType};

pub struct App {
    pub should_quit: bool,

    pub provider: Box<dyn Provider>,
    pub active_tasklist: usize,
    pub tasks_state: TableState,
}
impl App {
    pub fn new(provider: impl Provider + 'static) -> Self {
        Self {
            should_quit: false,
            provider: Box::new(provider),
            active_tasklist: 0,
            tasks_state: TableState::default(),
        }
    }

    pub fn on_tick(&mut self) {}
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn tasklists_next(&mut self) {
        self.tasks_state = TableState::default();

        self.active_tasklist = (self.active_tasklist + 1) % self.provider.len();
    }
    pub fn tasklists_previous(&mut self) {
        self.tasks_state = TableState::default();

        if self.active_tasklist > 0 {
            self.active_tasklist -= 1;
        } else {
            self.active_tasklist = self.provider.len() - 1;
        }
    }

    pub fn active_tasklist(&self) -> Option<&Tasklist> {
        self.provider.get_nth_tasklist(self.active_tasklist)
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

    fn active_task(&self) -> Option<&Task> {
        if let Some(tasklist) = self.active_tasklist() {
            if let Some(i) = self.tasks_state.selected() {
                tasklist.get(i)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn toggle_task_state(&mut self) -> anyhow::Result<()> {
        let tasklist = self
            .active_tasklist()
            .ok_or(anyhow::anyhow!("no active tasklist"))?;
        let task = self
            .active_task()
            .ok_or(anyhow::anyhow!("no active task"))?;

        let status = match task.status {
            Status::Todo => Status::Done,
            Status::Done => Status::Todo,
            Status::Unknown => Status::Unknown,
        };

        self.provider
            .update_task(
                &tasklist.id.clone(),
                &Task {
                    status,
                    ..task.clone()
                },
            )
            .await?;

        Ok(())
    }
}

use std::fmt;
impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("should_quit", &self.should_quit)
            .field("tasklists", &self.provider)
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
    pub fn new(id: String, title: String, tasks: &[Task]) -> Self {
        Self {
            id,
            title,
            tasks: tasks.to_vec(),
        }
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
pub enum Status {
    Todo,
    Done,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub status: Status,
    pub title: String,
    pub due: Option<TimestampType>,
    pub notes: Option<String>,
}
impl Task {
    pub fn new(
        id: &str,
        status: Status,
        title: &str,
        due: Option<TimestampType>,
        notes: Option<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            status,
            title: title.to_string(),
            due,
            notes: notes.map(|s| s.to_string()),
        }
    }
}
