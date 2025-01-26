// tests/api/storage_list.rs

// dependencies
use crate::helpers::{create_db, migrate_db, spawn_app};
use serde_json::{json, Value};

#[tokio::test]
async fn storage_list_returns_200_ok_and_bucket_contents() {
    // Arrange
    let db = create_db()
        .await
        .expect("Unable to create an in-memory database for testing.");
    let app = spawn_app(db).await;
    migrate_db(app.service_state)
        .await
        .expect("Unable to perform migrations on the test database.");

    // Act
    let response = app
        .api_client
        .get(format!("{}/public/storage_list", &app.address))
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
      "content": ["storage_check/test2.md", "storage_check/test1.md", "storage_check/", "/"],
    });
    assert_eq!(response_body, expected_body);
}
