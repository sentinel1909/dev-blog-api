// src/main.rs

// dependencies
use dev_blog_api_lib::config::ServiceConfig;
use dev_blog_api_lib::service::{DevBlogApiService, ServiceState};
use dev_blog_api_lib::telemetry::{get_subscriber, init_subscriber};
use libsql::Database;
use shuttle_runtime::{Error, SecretStore, Secrets};
use shuttle_turso::Turso;
use std::sync::Arc;

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main(
    #[Turso(addr = "{secrets.TURSO_DB_ADDR}", token = "{secrets.TURSO_DB_TOKEN}")]
    db_client: Database,
    #[Secrets] secrets: SecretStore,
) -> Result<DevBlogApiService, Error> {
    // initialize tracing
    let subscriber = get_subscriber("dev-blog-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // configure the returned Turso database client
    let db_client = Arc::new(db_client);

    // build the service configuration
    let config = ServiceConfig::try_from(secrets)?;

    // build the service state, which includes the configuration and Turso database client
    let service_state = ServiceState {
        service_config: config,
        service_db: db_client,
    };

    // build the router
    let service_router = DevBlogApiService::build_router(service_state);

    // start the service
    Ok(DevBlogApiService { service_router })
}
