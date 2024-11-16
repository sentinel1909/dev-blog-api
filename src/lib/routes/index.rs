// src/lib/routes/index.rs

// dependencies
use crate::error::ApiError;
use crate::renderer::get_templates;
use axum::response::{Html, IntoResponse};
use tera::Context;

// function to build the index template
fn build_index_template() -> Result<String, ApiError> {
    let context = Context::new();
    get_templates()?
        .render("base.html", &context)
        .map_err(|err| ApiError::Internal(err.to_string()))
}

// index route handler, returns the index template so that it can be rendered by the browser
pub async fn get_index() -> impl IntoResponse {
    match build_index_template() {
        Ok(template) => Html(template),
        Err(e) => Html(e.to_string()),
    }
}
