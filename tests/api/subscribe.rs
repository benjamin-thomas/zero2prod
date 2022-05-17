#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    crate::with_tx(|pool, socket| async move {
        // Arrange
        let client = reqwest::Client::new();

        // Act
        let body = "name=John%20Doe&email=john.doe%40example.com";
        let response = client
            .post(crate::url_for(socket, "/subscribe"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to POST at /subscribe");

        // Assert
        // User is saved to the database
        assert_eq!(200, response.status().as_u16());
        let saved = sqlx::query!("SELECT name, email FROM subscriptions")
            .fetch_one(&pool)
            .await
            .expect("Could not fetch subscriptions");

        assert_eq!("John Doe", saved.name);
        assert_eq!("john.doe@example.com", saved.email);

        // Will send an email later via a background job
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*)
            FROM queue WHERE status = 0
            AND message->'SendConfirmEmail'->>'email' = 'john.doe@example.com';
            "#,
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch queue count");
        assert_eq!(1, row.count.unwrap());
    })
    .await;
}

#[tokio::test]
async fn subscribe_returns_a_400_on_bad_data_data() {
    crate::with_tx(|_pool, socket| async move {
        // Arrange
        let client = reqwest::Client::new();

        let cases = vec![
            ("name=John%20Doe", "missing email"),
            ("email=john.doe%40example.com", "missing name"),
            ("", "missing name and email"),
            ("name=&email=john.doe%40example.com", "empty name"),
            ("name=John&email=", "empty email"),
            ("name=John&email=bogus-email", "invalid email"),
        ];

        for (body, hint) in cases {
            // Act
            let response = client
                .post(crate::url_for(socket, "/subscribe"))
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
