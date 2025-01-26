// src/lib/service.rs

// dependencies
use crate::config::ServiceConfig;
use crate::routes::{
    health_check,
    home::get_home,
    login::get_login_form,
    openapi,
    storage::{storage_check, storage_list, storage_read},
};
use crate::telemetry::MakeRequestUuid;
use anyhow::Result;
use axum::{http::HeaderName, routing::get, Router};
use libsql::Database;
use opendal::Operator;
use shuttle_runtime::{Error, Service};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::layer::Layer;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    normalize_path::NormalizePathLayer,
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    services::ServeDir,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// struct type to represent the server service
pub struct DevBlogApplication(pub Router);

// struct type to represent application state
#[derive(Clone)]
pub struct ServiceState {
    pub service_config: ServiceConfig,
    pub service_db: Arc<Database>,
    pub service_storage: Operator,
}

// methods for the DevBlogApiService  type
impl DevBlogApplication {
    pub fn build(state: ServiceState) -> Result<DevBlogApplication> {
        // define the tracing layer
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .include_headers(true)
                    .level(Level::INFO),
            )
            .on_response(DefaultOnResponse::new().include_headers(true));

        // define a layer to handle CORS (Cross Origin Resource Sharing)
        let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

        // create template assets, wrap them in a trace layer
        let template_assets_service = ServiceBuilder::new()
            .layer(&trace_layer)
            .service(ServeDir::new("templates"));

        // create assets, wrap them in a trace layer
        let assets_service = ServiceBuilder::new()
            .layer(&trace_layer)
            .service(ServeDir::new("assets"));

        // build the router and wrap it with CORS and the telemetry layers
        let x_request_id = HeaderName::from_static("x-request-id");
        let public_routes = Router::new()
            .route("/home", get(get_home))
            .route("/login", get(get_login_form))
            .route("/health_check", get(health_check))
            .route("/storage_check", get(storage_check))
            .route("/storage_list", get(storage_list))
            .route("/storage_read", get(storage_read))
            .route("/docs/openapi.json", get(openapi))
            .with_state(state)
            .layer(cors)
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        x_request_id.clone(),
                        MakeRequestUuid,
                    ))
                    .layer(trace_layer)
                    .layer(PropagateRequestIdLayer::new(x_request_id)),
            );

        // wrap the API routes with Normalize Path Layer
        let api_router = NormalizePathLayer::trim_trailing_slash().layer(public_routes);

        // combine the api and asset routes to make the complete router
        let app = Router::new()
            .nest_service("/assets", assets_service)
            .nest_service("/public", api_router)
            .fallback_service(template_assets_service);

        Ok(Self(app))
    }

    // function to run the application in tests
    pub async fn run_until_stopped(self, addr: SocketAddr) {
        let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.0).await.unwrap();
    }
}

// implement the Shuttle Service trait on the DevBlogApiService type
#[shuttle_runtime::async_trait]
impl Service for DevBlogApplication {
    async fn bind(self, addr: SocketAddr) -> Result<(), Error> {
        let router = self.0;

        axum::serve(tokio::net::TcpListener::bind(addr).await?, router).await?;

        Ok(())
    }
}
