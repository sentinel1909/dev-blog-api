// tests/api/helpers.rs

use dev_blog_api_lib::ServiceState;
// dependencies
use dev_blog_api_lib::{config::ServiceConfig, DevBlogApplication};
use libsql::{Builder, Database, Error};
use opendal::services::Fs;
use opendal::Operator;
use reqwest::Client;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Notify;

// struct type which models a test application
#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: Client,
    pub service_state: ServiceState,
}

// helper function to create and return a database for testing
pub async fn create_db() -> Result<Database, Error> {
    Builder::new_local(":memory:").build().await
}

// helper function to do the migrations on the testing database
pub async fn migrate_db(state: &ServiceState) -> Result<(), Error> {
    let conn = state.service_db.connect()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles(id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL UNIQUE, date TEXT NOT NULL, slug TEXT NOT NULL, category TEXT NOT NULL, tag TEXT NOT NULL, summary TEXT NOT NULL, content TEXT NOT NULL);",
        (),
    )
    .await?;
    Ok(())
}

// helper function to create local storage for testing
pub async fn create_storage() -> Result<Operator, opendal::Error> {
    let builder = Fs::default().root("dev_blog_testing");
    let op = Operator::new(builder).unwrap().finish();
    op.create_dir("content/").await?;
    Ok(op)
}

pub async fn spawn_app(db: Database, op: Operator) -> TestApp {
    // build the app test configuration
    let config = ServiceConfig {};

    // configure the app state for testing
    let state = ServiceState {
        service_config: config,
        service_db: Arc::new(db),
        service_storage: op,
    };

    // build the app for testing
    let application =
        DevBlogApplication::build(state.clone()).expect("Failed to build the application.");
    let listener = TcpListener::bind("localhost:0")
        .await
        .expect("Failed to bind port.");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    {
        // run the app
        let server_started = Arc::new(Notify::new());
        tokio::spawn({
            let server_started = Arc::clone(&server_started);
            async move {
                server_started.notify_one();
                application.run_until_stopped(listener).await;
            }
        });
        server_started.notified().await;
    }

    // configure the base, empty API client for testing
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: format!("http:localhost:{port}"),
        port,
        api_client: client,
        service_state: state,
    }
}
