// src/lib/routes/openapi.rs

// dependencies
use axum::Json;
use utoipa::OpenApi;

// struct type to represent OpenAPI docs
#[derive(OpenApi)]
#[openapi(paths(openapi))]
struct ApiDoc;

// return JSON version of an OpenAPI schema
#[utoipa::path(
  get,
  path = "/api/docs/openapi.json",
  responses(
    (status = 200, description = "JSON file", body = ())
  )
)]
pub async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
