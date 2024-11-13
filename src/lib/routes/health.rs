// src/lib/handlers/health.rs

// dependencies
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

// struct type to represent the /health_check endpoint response
#[derive(Serialize)]
struct HealthCheckResponse {
    status: &'static str,
}

// health check handler; returns 200 OK and an empty body
pub async fn health_check() -> impl IntoResponse {
    let response = HealthCheckResponse { status: "ok" };

    (StatusCode::OK, Json(response))
}
