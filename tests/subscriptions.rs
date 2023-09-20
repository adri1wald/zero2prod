mod common;
use common::spawn_app;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let addr = app.address;
    let db_pool = app.db_pool;
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
        .fetch_one(&db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "adrien@coloop.ai");
    assert_eq!(saved.name, "Adrien Wald");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let addr = app.address;
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

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_empty() {
    // Arrange
    let app = spawn_app().await;
    let clint = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=adrien%40coloop.ai", "empty name"),
        ("name=Adrien%20Wald&email=", "empty email"),
        (
            "name=Adrien%20Wald&email=definitely-not-an-email",
            "invalid email",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = clint
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload had an {}.",
            error_message
        );
    }
}
