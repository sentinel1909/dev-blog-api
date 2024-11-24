// src/lib/service.rs

// dependencies
use crate::config::ServiceConfig;
use crate::routes::{
    health_check,
    home::get_home,
    login::get_login_form,
    openapi,
    storage::{storage_check, storage_list},
};
use crate::telemetry::MakeRequestUuid;
use axum::{http::HeaderName, routing::get, Router};
use libsql::Database;
use opendal::Operator;
use shuttle_runtime::{Error, Service};
use std::net::SocketAddr;
use std::sync::Arc;
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
pub struct DevBlogApiService {
    pub service_router: Router,
}

// struct type to represent application state
#[derive(Clone)]
pub struct ServiceState {
    pub service_config: ServiceConfig,
    pub service_db: Arc<Database>,
    pub service_storage: Operator,
}

// methods for the DevBlogApiService  type
impl DevBlogApiService {
    pub fn build_router(state: ServiceState) -> Router {
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
        Router::new()
            .nest_service("/", template_assets_service)
            .nest_service("/assets", assets_service)
            .nest_service("/public", api_router)
    }
}

// implement the Shuttle Service trait on the DevBlogApiService type
#[shuttle_runtime::async_trait]
impl Service for DevBlogApiService {
    async fn bind(self, addr: SocketAddr) -> Result<(), Error> {
        let router = self.service_router;

        axum::serve(tokio::net::TcpListener::bind(addr).await?, router).await?;

        Ok(())
    }
}
