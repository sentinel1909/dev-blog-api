// src/main.rs

// dependencies
use dev_blog_api_lib::config::ServiceConfig;
use dev_blog_api_lib::service::{DevBlogApplication, ServiceState};
use dev_blog_api_lib::telemetry::{get_subscriber, init_subscriber};
use libsql::Database;
use opendal::Operator;
use shuttle_opendal::Opendal;
use shuttle_runtime::{CustomError, Error, SecretStore, Secrets};
use shuttle_turso::Turso;
use std::sync::Arc;

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main(
    #[Turso(addr = "{secrets.TURSO_DB_ADDR}", token = "{secrets.TURSO_DB_TOKEN}")] client: Database,
    #[Secrets] secrets: SecretStore,
    #[Opendal(scheme = "s3")] storage: Operator,
) -> Result<DevBlogApplication, Error> {
    // initialize tracing
    let subscriber = get_subscriber("dev-blog-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // configure the returned Turso database client and run initial migrations to create the posts table
    let client = Arc::new(client);
    let conn = client.connect().map_err(|err| {
        let error_msg = format!("Unable to connect to the database: {}", err);
        CustomError::new(err).context(error_msg)
    })?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles(id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL UNIQUE, date TEXT NOT NULL, slug TEXT NOT NULL, category TEXT NOT NULL, tag TEXT NOT NULL, summary TEXT NOT NULL, content TEXT NOT NULL);",
        (),
    )
    .await
    .map_err(|err| {
        let error_msg = format!("Unable to create the articles table: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    // build the service configuration
    let config = ServiceConfig::try_from(secrets)?;

    // build the service state, which includes the configuration and Turso database client
    let service_state = ServiceState {
        service_config: config,
        service_db: client,
        service_storage: storage,
    };

    // build the router
    let DevBlogApplication(router) = DevBlogApplication::build(service_state)?;

    // start the service
    Ok(DevBlogApplication(router))
}
