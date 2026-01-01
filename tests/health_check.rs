// tests/health_check.rs
use zero2prod::{run, HEALTH_CHECK};
// 'tokio::test' is the equivalent of `tokio::main`.
// It also spares you from having to specify the #[test] attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

// Launch our application in the background
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let end_point = "http://127.0.0.1:8000".to_owned() + HEALTH_CHECK;
    let response = client
        .get(end_point)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

