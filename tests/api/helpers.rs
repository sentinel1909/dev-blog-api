// tests/api/helpers.rs

// dependencies
use dev_blog_api_lib::{config::ServiceConfig, DevBlogApplication};
use reqwest::Client;
use std::net::TcpListener;

// struct type which models a test application
pub struct TestApp {
  pub address: String,
  pub port: u16,
  pub api_client: Client,
}

// function which configures the TestApp state
fn configure_state() {
  todo!()
}

pub async fn spawn_app() -> TestApp {

  let config = ServiceConfig{};

  let application = DevBlogApplication::build(state).expect("Failed to build the application.");

  let listener = TcpListener::bind("localhost:0").expect("Failed to bind port.");
  let addr = listener.local_addr().unwrap();
  let port = addr.port();

  tokio::spawn(application.run_until_stopped(addr));

  let client = Client::builder()
    .build()
    .unwrap();

  TestApp {
    address: format!("http:localhost:{port}"),
    port,
    api_client: client,
  }

}

