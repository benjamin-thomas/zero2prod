use tokio::spawn;
use zero2prod::run;

fn spawn_app() {
    let server = run().expect("Failed to bind address");
    spawn(server);
    return ();
}

#[tokio::test]
async fn health_works() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let url = "http://localhost:8000/health";
    let response = client.get(url).send().await.expect("GET request failed!");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length()); // "UP"
}
