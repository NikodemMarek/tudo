extern crate google_tasks1 as tasks1;

use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tasks1::{
    oauth2::{read_application_secret, ApplicationSecret, InstalledFlowReturnMethod},
    TasksHub,
};
use yup_oauth2::{authenticator::Authenticator, InstalledFlowAuthenticator};

use crate::app::App;
use crate::app::Task;
use crate::app::Tasklist;
use crate::timestamps::TimestampType;

static SECRET: &'static str = "client_secret.json";
static TOKEN_CACHE: &'static str = "tokencache.json";

pub async fn setup() -> anyhow::Result<App> {
    let auth_data = login().await?;
    let hub = get_hub(auth_data).await;

    let tasklists = load_tasklists(&hub).await?;
    let app = App::new(&tasklists);

    Ok(app)
}

async fn login() -> anyhow::Result<Authenticator<HttpsConnector<HttpConnector>>> {
    let secret: ApplicationSecret = read_application_secret(SECRET).await?;

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(TOKEN_CACHE)
        .build()
        .await?;

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
    id: &String,
) -> anyhow::Result<Vec<Task>> {
    let raw_tasks = hub.tasks().list(id).doit().await?.1.items;
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
                due,
                ..
            } => Some(Task::new(
                &id.clone(),
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
            )),
            _ => None,
        })
        .flatten()
        .collect();

    Ok(tasks)
}