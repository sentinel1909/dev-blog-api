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
    let config = ServiceConfig {};

    let db = Builder::new_local("dev-blog-api-local.db")
        .build()
        .await
        .expect("Unable to build a local database for testing.");

    let mut builder = Fs::default();
    builder.root("/tmp");
    let op = Operator::new(builder).unwrap().finish();

    let state = ServiceState {
        service_config: config,
        service_db: Arc::new(db),
        service_storage: op,
    };

    let application = DevBlogApplication::build(state).expect("Failed to build the application.");

    let listener = TcpListener::bind("localhost:0").expect("Failed to bind port.");
    let addr = listener.local_addr().unwrap();
    let port = addr.port();

    tokio::spawn(application.run_until_stopped(addr));

    let client = Client::builder().build().unwrap();

    TestApp {
        address: format!("http:localhost:{port}"),
        port,
        api_client: client,
    }
}
