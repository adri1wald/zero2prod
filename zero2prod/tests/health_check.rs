mod common;
use common::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let addr = app.address;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{addr}/health_check"))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
