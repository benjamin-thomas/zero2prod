#[tokio::test]
async fn health_works() {
    crate::no_tx(|_pool, socket| async move {
        // Arrange
        let client = reqwest::Client::new();

        // Act
        let response = client
            .get(crate::url_for(socket, "/health"))
            .send()
            .await
            .expect("GET request failed!");

        // Assert
        assert!(response.status().is_success());
        assert_eq!(Some(2), response.content_length()); // "UP"
    })
    .await;
}
