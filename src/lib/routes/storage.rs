// src/lib/routes/storage.rs

// dependencies
use crate::error::ApiError;
use crate::service::ServiceState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use opendal::{Entry, Error};
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

// struct type to represent the /storage_read endpoint response
#[derive(Debug, Serialize)]
struct StorageReadResponse {
    status: String,
    posts: Vec<String>,
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

// utility function which gets all the entries in the bucket
async fn list(state: &ServiceState) -> Result<Vec<Entry>, Error> {
    state.service_storage.list_with("/").recursive(true).await
}

// storage list handler
#[debug_handler]
#[tracing::instrument(name = "Storage List", skip(state))]
pub async fn storage_list(
    State(state): State<ServiceState>,
) -> Result<impl IntoResponse, ApiError> {
    let entries = list(&state).await.map_err(|err| {
        tracing::error!(
            "Storage list failed, unable to list items in the bucket: {}",
            err
        );
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

// storage read handler
#[debug_handler]
#[tracing::instrument(name = "Storage Read", skip(state))]
pub async fn storage_read(
    State(state): State<ServiceState>,
) -> Result<impl IntoResponse, ApiError> {
    let entries = list(&state).await.map_err(|err| {
        tracing::error!(
            "Storage read failed, unable to list items in the bucket: {}",
            err
        );
        ApiError::Internal(err.to_string())
    })?;

    let items: Vec<String> = entries
        .into_iter()
        .filter(|entry| entry.metadata().is_file())
        .map(|entry| entry.path().to_string())
        .collect();

    let mut posts: Vec<String> = Vec::new();
    for item in items.into_iter() {
        let bytes = state
            .service_storage
            .read_with(&item)
            .await
            .map_err(|err| {
                tracing::error!(
                "Storage read failed, unable to read the content of the items in the bucket: {}",
                err
            );
                ApiError::Internal(err.to_string())
            })?;
        let content = bytes.to_vec();
        let post = String::from_utf8(content).map_err(|err| {
            tracing::error!(
                "Storage read failed, unable to convert file contents to a string: {}", err
            );
            ApiError::Internal(err.to_string())
        })?;
        posts.push(post);
    }

    let response = StorageReadResponse {
        status: "ok".to_string(),
        posts,
    };

    Ok((StatusCode::OK, Json(response)))
}
