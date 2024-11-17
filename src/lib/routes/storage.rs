// src/lib/routes/storage.rs

// dependencies
use crate::service::ServiceState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

// struct type to represent the /health_check endpoint response
#[derive(Serialize)]
struct StorageCheckResponse {
    status: String,
}

// storage check handler; if no errors are returned from the check, a 200 OK response with empty body is returned,
// otherwise the error message is returned 
pub async fn storage_check(State(state): State<ServiceState>) -> impl IntoResponse {
    let op = state.service_storage.check().await;
    let response = match op {
        Ok(_) => StorageCheckResponse { status: "ok".to_string() },
        Err(e) => StorageCheckResponse { status: e.to_string() },
    };

    (StatusCode::OK, Json(response))
}
