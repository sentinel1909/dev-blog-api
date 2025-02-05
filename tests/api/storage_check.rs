// tests/api/storage_check.rs

// dependencies
use crate::helpers::{cleanup_storage, create_db, create_storage, setup_test_files, spawn_app};
use serde_json::{json, Value};

#[tokio::test]
async fn storage_check_returns_200_ok() {
    // Arrange
    let db = create_db()
        .await
        .expect("Unable to create an in-memory database for testing.");
    let op = create_storage()
        .await
        .expect("Unable to create local storage directory for testing.");
    let app = spawn_app(db, op).await;
    setup_test_files(&app.service_state.service_storage)
        .await
        .expect("Unable to set up test files.");

    // Act
    let response = app
        .api_client
        .get(format!("{}/public/storage_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    let response_body: Value = response
        .json()
        .await
        .expect("Failed to parse JSON from response body.");
    let expected_body = json!({
      "status": "ok",
    });
    assert_eq!(response_body, expected_body);

    // Clean up
    cleanup_storage(&app.service_state.service_storage)
        .await
        .expect("Failed to clean up storage.");
}
