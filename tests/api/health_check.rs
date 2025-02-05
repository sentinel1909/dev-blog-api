// tests/api/health_check.rs

// dependencies
use crate::helpers::{cleanup_storage, create_db, create_storage, spawn_app};
use serde_json::{json, Value};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let db = create_db()
        .await
        .expect("Unable to create an in-memory database for testing.");
    let op = create_storage()
        .await
        .expect("Unable to create local storage directory for testing.");
    let app = spawn_app(db, op).await;


    // Act
    let response = app
        .api_client
        .get(format!("{}/public/health_check", &app.address))
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
      "status": "ok"
    });
    assert_eq!(response_body, expected_body);

    // Clean up
    cleanup_storage(&app.service_state.service_storage)
        .await
        .expect("Failed to clean up storage.");
}
