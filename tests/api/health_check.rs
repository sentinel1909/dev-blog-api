// tests/api/health_check.rs

// dependencies
use crate::helpers::spawn_app;
use serde_json::{json, Value};

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

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
}
