// src/lib/routes/storage.rs

// dependencies
use crate::error::ApiError;
use crate::service::ServiceState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::Serialize;

// struct type to represent the /storage_check endpoint response
#[derive(Debug, Serialize)]
struct StorageCheckResponse {
    status: String,
}

// struct type to represent the /storage_list endpoint response
#[derive(Debug, Serialize)]
struct StorageListResponse {
    status: String,
    content: Option<Vec<String>>,
}

// storage check handler; if no errors are returned from the check, a 200 OK response with empty body is returned,
// otherwise the error message is returned
#[debug_handler]
#[tracing::instrument(name = "Storage Check", skip(state))]
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

// storage list handler
#[debug_handler]
#[tracing::instrument(name = "Storage List", skip(state))]
pub async fn storage_list(
    State(state): State<ServiceState>,
) -> Result<impl IntoResponse, ApiError> {
    let entries = state
        .service_storage
        .list_with("/")
        .recursive(true)
        .await
        .map_err(|err| {
            tracing::error!("Storage check failed, unable to list items in the bucket: {}", err);
            ApiError::Internal(err.to_string())
        })?;

    let items: Vec<String> = entries
        .into_iter()
        .map(|entry| entry.path().to_string())
        .collect();

    let response = StorageListResponse {
        status: "ok".to_string(),
        content: Some(items),
    };

    Ok((StatusCode::OK, Json(response)))
}
