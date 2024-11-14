// src/main.rs

// dependencies
use dev_blog_api_lib::service::DevBlogApiService;
use dev_blog_api_lib::telemetry::{get_subscriber, init_subscriber};
use libsql::Database;
use shuttle_runtime::{CustomError, Error, SecretStore, Secrets};
use shuttle_turso::Turso;
use std::sync::Arc;

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main(
    #[Turso(addr = "{secrets.TURSO_DB_ADDR}", token = "{secrets.TURSO_DB_TOKEN}")] client: Database,
    #[Secrets] _secrets: SecretStore,
) -> Result<DevBlogApiService, Error> {
    // initialize tracing
    let subscriber = get_subscriber("dev-blog-api".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // wrap the database client in an Arc and create a connection
    let client = Arc::new(client);
    let conn = client.connect().map_err(|err| {
        let error_msg = format!("Unable to obtain a database connection: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    // create the users database
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user ( id INTEGER NOT NULL PRIMARY KEY );",
        (),
    )
    .await
    .map_err(|err| {
        let error_msg = format!("Unable to create the user table: {}", err);
        CustomError::new(err).context(error_msg)
    })?;

    // create the session database
    conn.execute("CREATE TABLE IF NOT EXISTS session ( id TEXT NOT NULL PRIMARY KEY, user_id INTEGER NOT NULL REFERENCES user(id), expires_at INTEGER NOT NULL );", ())
        .await
        .map_err(|err| {
            let error_msg = format!("Unable to create the session table: {}", err);
            CustomError::new(err).context(error_msg)
        })?;

    // build the router
    let app_router = DevBlogApiService::build();

    // start the service
    Ok(DevBlogApiService { app_router })
}
