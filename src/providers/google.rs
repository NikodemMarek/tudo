extern crate google_tasks1 as tasks1;

use envpath::EnvPath;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tasks1::{
    oauth2::{read_application_secret, ApplicationSecret, InstalledFlowReturnMethod},
    TasksHub,
};
use yup_oauth2::{authenticator::Authenticator, InstalledFlowAuthenticator};

use crate::{
    app::{App, Status, Task, Tasklist},
    config::Cfg,
    provider::Provider,
    timestamps::TimestampType,
};

pub struct GoogleTasksProvider {
    hub: TasksHub<HttpsConnector<HttpConnector>>,

    tasklists: Vec<Tasklist>,
}

impl GoogleTasksProvider {
    fn new(hub: TasksHub<HttpsConnector<HttpConnector>>) -> Self {
        Self {
            hub,
            tasklists: Vec::new(),
        }
    }

    async fn load_tasklists(&mut self) -> anyhow::Result<()> {
        self.tasklists = load_tasklists(&self.hub).await?;

        Ok(())
    }

    async fn load_tasklist(&mut self, id: &str) -> anyhow::Result<()> {
        let tasks = load_tasks(&self.hub, id).await?;
        let tasklist = self
            .tasklists
            .iter()
            .find(|t| t.id == id)
            .ok_or(anyhow::anyhow!("tasklist with id {} not found", id))?;
        let tasklist = Tasklist {
            tasks,
            ..tasklist.clone()
        };

        self.tasklists
            .iter_mut()
            .find(|t| t.id == id)
            .map(|t| *t = tasklist);

        Ok(())
    }
}

#[async_trait::async_trait]
impl Provider for GoogleTasksProvider {
    fn get_tasklists(&self) -> &Vec<Tasklist> {
        &self.tasklists
    }

    async fn update_task(&mut self, tasklist_id: &str, task: &Task) -> anyhow::Result<()> {
        self.hub
            .tasks()
            .update(task_to_gtask(&task), tasklist_id, &task.id)
            .doit()
            .await?;

        self.load_tasklist(tasklist_id).await?;

        Ok(())
    }
}

pub async fn setup(cfg: &Cfg) -> anyhow::Result<App> {
    let auth_data = login(&cfg.client_secret).await?;
    let hub = get_hub(auth_data).await;

    let mut provider = GoogleTasksProvider::new(hub);
    provider.load_tasklists().await?;

    let app = App::new(provider);

    Ok(app)
}

async fn login(
    client_secret: &str,
) -> anyhow::Result<Authenticator<HttpsConnector<HttpConnector>>> {
    let secret: ApplicationSecret = read_application_secret(client_secret).await?;

    let token_cache = EnvPath::from(["$dir: cache", "tudo", "config.toml"])
        .de()
        .to_path_buf();

    let token_cache = if token_cache.exists() {
        Some(token_cache)
    } else {
        token_cache
            .parent()
            .map(|prefix| {
                std::fs::create_dir_all(prefix)
                    .ok()
                    .map(|_| token_cache.clone())
            })
            .flatten()
    };

    let auth = match token_cache {
        Some(token_cache) => {
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .persist_tokens_to_disk(token_cache)
                .build()
                .await?
        }
        None => {
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .build()
                .await?
        }
    };

    Ok(auth)
}

async fn get_hub(
    auth_data: Authenticator<HttpsConnector<HttpConnector>>,
) -> TasksHub<HttpsConnector<HttpConnector>> {
    TasksHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth_data,
    )
}

async fn load_tasklists(
    hub: &TasksHub<HttpsConnector<HttpConnector>>,
) -> anyhow::Result<Vec<Tasklist>> {
    let raw_tasklists = hub.tasklists().list().doit().await?.1.items;
    let raw_tasklists = match raw_tasklists {
        Some(raw_tasklists) => raw_tasklists,
        _ => Vec::new(),
    };

    let mut tasklists = Vec::new();
    for tasklist in raw_tasklists.iter() {
        if let tasks1::api::TaskList {
            id: Some(id),
            title: Some(title),
            ..
        } = tasklist
        {
            tasklists.push(Tasklist::new(
                id.to_owned(),
                title.to_owned(),
                &load_tasks(hub, id).await.unwrap_or(Vec::new()),
            ));
        };
    }

    Ok(tasklists)
}

async fn load_tasks(
    hub: &TasksHub<HttpsConnector<HttpConnector>>,
    id: &str,
) -> anyhow::Result<Vec<Task>> {
    let raw_tasks = hub.tasks().list(id).doit().await?.1.items;
    let raw_tasks = match raw_tasks {
        Some(raw_tasks) => raw_tasks,
        _ => Vec::new(),
    };

    let tasks: Vec<Task> = raw_tasks
        .iter()
        .map(|gtask| gtask_to_task(gtask))
        .flatten()
        .collect();

    Ok(tasks)
}

fn gtask_to_task(gtask: &tasks1::api::Task) -> Option<Task> {
    if let tasks1::api::Task {
        id: Some(id),
        status,
        title: Some(title),
        due,
        notes,
        ..
    } = gtask
    {
        Some(Task::new(
            &id.clone(),
            match status.as_deref() {
                Some("needsAction") => Status::Todo,
                Some("completed") => Status::Done,
                _ => Status::Unknown,
            },
            &title.clone(),
            due.clone()
                .map(|x| {
                    chrono::DateTime::parse_from_rfc3339(&x)
                        .ok()
                        .map(|y| match x {
                            i if i.ends_with("T00:00:00.000Z") => {
                                TimestampType::Date(y.naive_local().date())
                            }
                            i if i.starts_with("0000-00-00T") => {
                                TimestampType::Time(y.naive_local().time())
                            }
                            _ => TimestampType::DateTime(y.naive_local()),
                        })
                })
                .flatten(),
            notes.as_deref(),
        ))
    } else {
        None
    }
}

fn task_to_gtask(task: &Task) -> tasks1::api::Task {
    let status = match task.status {
        Status::Todo => Some(String::from("needsAction")),
        Status::Done => Some(String::from("completed")),
        Status::Unknown => None,
    };
    let due = task.due.as_ref().map(|t| match t {
        TimestampType::Date(date) => date.format("%Y-%m-%dT00:00:00.000Z").to_string(),
        TimestampType::Time(time) => time.format("00-00-00T%H:%M:%S.000Z").to_string(),
        TimestampType::DateTime(datetime) => datetime.format("%Y-%m-%dT%H:%M:%S.000Z").to_string(),
    });

    tasks1::api::Task {
        id: Some(task.id.clone()),
        title: Some(task.title.clone()),
        status,
        due,
        notes: task.notes.clone(),
        ..Default::default()
    }
}
