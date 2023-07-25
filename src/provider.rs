use crate::app::{Task, Tasklist};

pub trait Provider {
    fn get_tasklists(&self) -> &Vec<Tasklist>;

    fn get_tasklist(&self, tasklist_id: &str) -> Option<&Tasklist>;
    fn get_nth_tasklist(&self, n: usize) -> Option<&Tasklist> {
        self.get_tasklists().get(n)
    }

    fn get_task(&self, tasklist_id: &str, task_id: &str) -> Option<&Task>;

    fn tasklists_len(&self) -> usize {
        self.get_tasklists().len()
    }
}

impl std::fmt::Debug for dyn Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Provider")
            .field("tasklists", &self.get_tasklists())
            .finish()
    }
}
