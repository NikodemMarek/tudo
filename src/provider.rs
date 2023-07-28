use crate::app::{Task, Tasklist};

#[async_trait::async_trait]
pub trait Provider {
    fn get_tasklists(&self) -> &Vec<Tasklist>;
    fn len(&self) -> usize {
        self.get_tasklists().len()
    }

    fn get_tasklist(&self, tasklist_id: &str) -> Option<&Tasklist> {
        self.get_tasklists().iter().find(|t| t.id == tasklist_id)
    }
    fn get_nth_tasklist(&self, n: usize) -> Option<&Tasklist> {
        self.get_tasklists().get(n)
    }

    fn get_task(&self, tasklist_id: &str, task_id: &str) -> Option<&Task> {
        self.get_tasklist(tasklist_id)
            .and_then(|t| t.tasks.iter().find(|t| t.id == task_id))
    }
    async fn update_task(&mut self, tasklist_id: &str, task: &Task) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl std::fmt::Debug for dyn Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Provider")
            .field("tasklists", &self.get_tasklists())
            .finish()
    }
}
