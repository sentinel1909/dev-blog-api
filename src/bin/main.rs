// src/main.rs

// dependencies
use shuttle_runtime::Error;
use dev_blog_api_lib::service::DevBlogApiService;
use dev_blog_api_lib::telemetry::{get_subscriber, init_subscriber};

// main function; configures tracing, builds the app router, starts the service
#[shuttle_runtime::main]
async fn main() -> Result<DevBlogApiService, Error> {
    // initialize tracing
    let subscriber = get_subscriber(
        "dev-blog-api".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // build the router
    let app_router = DevBlogApiService::build();

    // start the service
    Ok(DevBlogApiService { app_router })
}
