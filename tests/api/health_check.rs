//! tests/api/health_check.rs

use crate::helpers::spawn_app;
use zero2prod::startup::HEALTH_CHECK;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let end_point = &format!("{}{}", &test_app.address, HEALTH_CHECK);
    let response = client
        .get(end_point)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
