extern crate google_tasks1 as tasks1;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use tasks1::oauth2::{read_application_secret, ApplicationSecret, InstalledFlowReturnMethod};
use tasks1::TasksHub;
use yup_oauth2::authenticator::Authenticator;
use yup_oauth2::InstalledFlowAuthenticator;

static SECRET: &'static str = "client_secret.json";
static TOKEN_CACHE: &'static str = "tokencache.json";

pub async fn login() -> anyhow::Result<Authenticator<HttpsConnector<HttpConnector>>> {
    let secret: ApplicationSecret = read_application_secret(SECRET).await?;

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(TOKEN_CACHE)
        .build()
        .await?;

    Ok(auth)
}

pub async fn get_hub(
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

pub async fn setup() -> anyhow::Result<TasksHub<HttpsConnector<HttpConnector>>> {
    let auth_data = login().await?;
    let hub = get_hub(auth_data).await;
    Ok(hub)
}
