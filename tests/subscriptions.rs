mod common;
use common::spawn_app;
use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let addr = spawn_app();
    let config = get_configuration().expect("Failed to read configuration");
    let connection_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    let name = "Adrien%20Wald";
    let email = "adrien%40coloop.ai";
    let body = format!("name={name}&email={email}");

    // Act
    let response = client
        .post(format!("{addr}/subscriptions"))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "adrien@coloop.ai");
    assert_eq!(saved.name, "Adrien Wald");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let name = "Adrien%20Wald";
    let email = "adrien%40coloop.ai";
    let test_cases = vec![
        (format!("name={name}"), "missing the email"),
        (format!("email={email}"), "missing the name"),
        ("".to_owned(), "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{addr}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
