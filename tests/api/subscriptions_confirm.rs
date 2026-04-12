//! tests/api/subscriptions_confirm.rs

use crate::helpers::spawn_app;
use reqwest::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};
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

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=boromir&email=boromir%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .links(s)
            .filter(|l| *l.kind() == linkify::LinkKind::Url)
            .collect();
        assert_eq!(links.len(), 1);
        links[0].as_str().to_owned()
    };

    let raw_confirmation_link = &get_link(body["html"].as_str().unwrap());
    let mut confirmation_link = Url::parse(raw_confirmation_link).unwrap();

    // make sure not to call random apis!
    assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");

    // rewrite the url to include the port
    confirmation_link.set_port(Some(app.port)).unwrap();

    // Act
    let response = reqwest::get(confirmation_link).await.unwrap();

    // Assert
    assert_eq!(response.status().as_u16(), 200);
}
