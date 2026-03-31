//! tests/api/subscriptions_confirm.rs

use crate::helpers::spawn_app;
use zero2prod::startup::SUBSCRIPTIONS_CONFIRMATION;

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_a_400() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = reqwest::get(&format!("{}{}", app.address, SUBSCRIPTIONS_CONFIRMATION))
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 400);
}
