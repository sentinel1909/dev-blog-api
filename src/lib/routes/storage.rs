// src/lib/routes/storage.rs

// dependencies
use crate::error::ApiError;
use crate::service::ServiceState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::Serialize;

// struct type to represent the /health_check endpoint response
#[derive(Serialize)]
struct StorageCheckResponse {
    status: String,
}

// storage check handler; if no errors are returned from the check, a 200 OK response with empty body is returned,
// otherwise the error message is returned
#[debug_handler]
pub async fn storage_check(
    State(state): State<ServiceState>,
) -> Result<impl IntoResponse, ApiError> {
    state.service_storage.check().await.map_err(|err| {
        tracing::error!("Storage check failed: {}", err);
        ApiError::Internal(err.to_string())
    })?;

    let response = StorageCheckResponse {
        status: "ok".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}
