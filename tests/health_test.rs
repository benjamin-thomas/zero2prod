fn spawn_app() -> String {
    let listener =
        std::net::TcpListener::bind("localhost:0").expect("Failed to start listener (random port)");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start server");

    tokio::spawn(server);
    return format!("http://localhost:{}", port);
}

#[tokio::test]
async fn health_works() {
    // Arrange
    let base_url = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(base_url + "/health")
        .send()
        .await
        .expect("GET request failed!");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(2), response.content_length()); // "UP"
}
