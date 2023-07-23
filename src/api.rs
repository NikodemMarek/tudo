pub mod tasklists {
    use hyper::client::HttpConnector;
    use hyper_rustls::HttpsConnector;
    use tasks1::TasksHub;

    use crate::app::Tasklist;

    pub async fn load(
        hub: &TasksHub<HttpsConnector<HttpConnector>>,
    ) -> anyhow::Result<Vec<Tasklist>> {
        let raw_tasklists = hub.tasklists().list().doit().await?.1.items;
        let raw_tasklists = match raw_tasklists {
            Some(raw_tasklists) => raw_tasklists,
            _ => Vec::new(),
        };

        let tasklists: Vec<Tasklist> = raw_tasklists
            .iter()
            .map(|x| match x {
                tasks1::api::TaskList {
                    id: Some(id),
                    title: Some(title),
                    ..
                } => Some(Tasklist::new(id.clone(), title.clone())),
                _ => None,
            })
            .flatten()
            .collect();

        Ok(tasklists)
    }
}

pub mod tasks {
    use hyper::client::HttpConnector;
    use hyper_rustls::HttpsConnector;
    use tasks1::TasksHub;

    use crate::app::Task;

    pub async fn load(
        hub: &TasksHub<HttpsConnector<HttpConnector>>,
        id: String,
    ) -> anyhow::Result<Vec<Task>> {
        let raw_tasks = hub.tasks().list(&id).doit().await?.1.items;
        let raw_tasks = match raw_tasks {
            Some(raw_tasks) => raw_tasks,
            _ => Vec::new(),
        };

        let tasks: Vec<Task> = raw_tasks
            .iter()
            .map(|x| match x {
                tasks1::api::Task {
                    id: Some(id),
                    title: Some(title),
                    ..
                } => Some(Task::new(&id.clone(), &title.clone())),
                _ => None,
            })
            .flatten()
            .collect();

        Ok(tasks)
    }
}
