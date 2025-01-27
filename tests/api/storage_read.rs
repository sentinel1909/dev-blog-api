// tests/api/storage_read.rs

// dependencies
use crate::helpers::{create_db, create_storage, migrate_db, spawn_app};
use serde_json::{json, Value};

#[tokio::test]
async fn storage_read_returns_200_ok_and_content_from_each_file() {
    // Arrange
    let db = create_db()
        .await
        .expect("Unable to create an in-memory database for testing.");
    let op = create_storage()
        .await
        .expect("Unable to create local storage directory for testing.");
    let app = spawn_app(db, op).await;
    migrate_db(&app.service_state)
        .await
        .expect("Unable to perform migrations on the test database.");

    // Act
    let response = app
        .api_client
        .get(format!("{}/public/storage_read", &app.address))
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
      "posts": ["---\ntitle: \"Second Test Post\"\ndate: \"2025-01-26\"\nslug: \"second-test-post\"\ncategory: \"test\"\ntag: \"test\"\nsummary: \"This is the summary of the second test post.\"\ndraft: true\nedited: false\n---\n\n# Second Test Post\n\nThis is the second test post.\n", "---\ntitle: \"Test Post\"\ndate: \"2025-01-26\"\nslug: \"test-post\"\ncategory: \"test\"\ntag: \"test\"\nsummary: \"This is the summary of the test post.\"\ndraft: true\nedited: false\n---\n\n# Test Post\n\nThis is a test post.\n", "---\ntitle: \"Third Test Post\"\ndate: \"2025-01-26\"\nslug: \"third-test-post\"\ncategory: \"test\"\ntag: \"test\"\nsummary: \"This is the summary of the third test post.\"\nedited: false\n---\n\n# Third Test Post\n\nThis is the third test post.\n"]
    });
    assert_eq!(response_body, expected_body);
}
