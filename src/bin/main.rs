// src/main.rs

// dependencies
use dev_blog_api_lib::service::{AppState, DevBlogApiService};
use dev_blog_api_lib::telemetry::{get_subscriber, init_subscriber};
use libsql::Database;
use shuttle_runtime::{Error, SecretStore, Secrets};
use shuttle_turso::Turso;
use std::sync::Arc;

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main(
    #[Turso(addr = "{secrets.TURSO_DB_ADDR}", token = "{secrets.TURSO_DB_TOKEN}")] db_client: Database,
    #[Secrets] _secrets: SecretStore,
) -> Result<DevBlogApiService, Error> {
    // initialize tracing
    let subscriber = get_subscriber("dev-blog-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // wrap the db_client in an Arc and create the http_client, add each to the service state 
    let db_client = Arc::new(db_client);
    let http_client = DevBlogApiService::build_http_client();
    let service_state = AppState { service_http_client: http_client, service_db: db_client.clone() }; 

    // build the router
    let service_router = DevBlogApiService::build_router(db_client);

    // start the service
    Ok(DevBlogApiService { service_router, service_state})
}
