use crate::utils::url_for;

mod init_server;
mod utils;

#[tokio::test]
async fn health_works() {
    init_server::no_tx(|_pool, socket| async move {
        // Arrange
        let client = reqwest::Client::new();

        // Act
        let response = client
            .get(url_for(socket, "/health"))
            .send()
            .await
            .expect("GET request failed!");

        // Assert
        assert!(response.status().is_success());
        assert_eq!(Some(2), response.content_length()); // "UP"
    })
    .await;
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    init_server::with_tx(|pool, socket| async move {
        // Arrange
        let client = reqwest::Client::new();

        // Act
        let body = "name=John%20Doe&email=john.doe%40example.com";
        let response = client
            .post(url_for(socket, "/subscribe"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to POST at /subscribe");

        // Assert
        assert_eq!(200, response.status().as_u16());
        let saved = sqlx::query!("SELECT name, email FROM subscriptions")
            .fetch_one(&pool)
            .await
            .expect("Could not fetch subscriptions");

        assert_eq!("John Do", saved.name);
        assert_eq!("john.doe@example.com", saved.email);
    })
    .await;
}

#[tokio::test]
async fn subscribe_returns_a_400_on_missing_data() {
    init_server::with_tx(|_pool, socket| async move {
        // Arrange
        let client = reqwest::Client::new();

        let cases = vec![
            ("name=John%20Doe", "missing email"),
            ("email=john.doe%40example.com", "missing name"),
            ("", "missing name and email"),
        ];

        for (body, hint) in cases {
            // Act
            let response = client
                .post(url_for(socket, "/subscribe"))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await
                .expect("Failed to POST at /subscribe");

            // Assert
            assert_eq!(
                400,
                response.status().as_u16(),
                "Expected HTTP 400 error (hint: {})",
                hint,
            );
        }
    })
    .await;
}
