use std::net::TcpListener;

use zero2prod::HEALTH_CHECK;

// tests/health_check.rs
// 'tokio::test' is the equivalent of `tokio::main`.
// It also spares you from having to specify the #[test] attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

// Launch our application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // retrieve port assigned by OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    // return application address to the caller
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let end_point = address + HEALTH_CHECK;
    let response = client
        .get(end_point)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
