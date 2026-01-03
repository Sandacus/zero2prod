use std::net::TcpListener;

use zero2prod::startup::{run, HEALTH_CHECK, SUBSCRIBE};
use zero2prod::configuration::get_configuration;
use sqlx::{PgConnection, Connection};

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
    let server = run(listener).expect("Failed to bind address");
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let config = get_configuration().expect("Failed to read configuration");
    let connection_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres!");
    let client = reqwest::Client::new();

    // Act
    let endpoint = app_address + SUBSCRIBE;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute POST request");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin")
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let endpoint = format!("{}{}", &app_address, SUBSCRIBE);
        let response = client
            .post(endpoint)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute POST request");
    
        // Assert
        assert_eq!(
            400, 
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.", 
            error_message
        );
    }

}