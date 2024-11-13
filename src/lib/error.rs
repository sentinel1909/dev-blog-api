// src/lib/error.rs

// dependencies
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

// struct type to represent an ApiError
#[derive(Debug, Clone, Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

// implement the IntoResponse trait for the ApiError type
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::BadRequest(err) => (StatusCode::BAD_REQUEST, format!("Bad Request: {}", err)),
            ApiError::Internal(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", err),
            ),
            ApiError::NotFound(err) => (StatusCode::NOT_FOUND, format!("Not Found: {}", err)),
        };
        tracing::error!("Error occurred: {:?}", self);
        (status, msg).into_response()
    }
}
