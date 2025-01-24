// tests/api/helpers.rs

use dev_blog_api_lib::ServiceState;
// dependencies
use dev_blog_api_lib::{config::ServiceConfig, DevBlogApplication};
use libsql::Builder;
use opendal::services::Fs;
use opendal::Operator;
use reqwest::Client;
use std::net::TcpListener;
use std::sync::Arc;

// struct type which models a test application
pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: Client,
}

pub async fn spawn_app() -> TestApp {
    // build the app test configuration
    let config = ServiceConfig {};

    // build a local database for testing
    let db = Builder::new_local("dev-blog-api-local.db")
        .build()
        .await
        .expect("Unable to build a local database for testing.");

    // configure OpenDAL to use local storage for testing
    let builder = Fs::default()
        .root("dev_blog_testing/content");
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
    let application = DevBlogApplication::build(state).expect("Failed to build the application.");
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind port.");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    // run the app
    tokio::spawn(application.run_until_stopped(addr));

    // configure the base, empty API client for testing
    let client = Client::builder().build().unwrap();

    TestApp {
        address: format!("http:localhost:{port}"),
        port,
        api_client: client,
    }
}
