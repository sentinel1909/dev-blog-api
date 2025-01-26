// tests/api/helpers.rs

use dev_blog_api_lib::ServiceState;
// dependencies
use dev_blog_api_lib::{config::ServiceConfig, DevBlogApplication};
use libsql::{Builder, Database, Error};
use opendal::services::Fs;
use opendal::Operator;
use reqwest::Client;
use std::net::TcpListener;
use std::sync::Arc;

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
pub async fn migrate_db(state: ServiceState) -> Result<(), Error> {
    let conn = state.service_db.connect()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles(id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL UNIQUE, date TEXT NOT NULL, slug TEXT NOT NULL, category TEXT NOT NULL, tag TEXT NOT NULL, summary TEXT NOT NULL, content TEXT NOT NULL);",
        (),
    )
    .await?;
    Ok(())
}

pub async fn spawn_app(db: Database) -> TestApp {
    // build the app test configuration
    let config = ServiceConfig {};

    // configure a local directory for testing storage
    let builder = Fs::default().root("dev_blog_testing/content");
    let op = Operator::new(builder).unwrap().finish();
    op.create_dir("storage_check/")
        .await
        .expect("Unable to create temporary directory for testing.");

    // configure the app state for testing
    let state = ServiceState {
        service_config: config,
        service_db: Arc::new(db),
        service_storage: op,
    };

    // build the app for testing
    let application =
        DevBlogApplication::build(state.clone()).expect("Failed to build the application.");
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind port.");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    // run the app
    tokio::spawn(application.run_until_stopped(addr));

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
