#[tokio::test]
async fn integration_test_wrapped_in_a_transaction() {
    // Ideas from: https://stackoverflow.com/questions/65370752/how-do-i-create-an-actix-web-server-that-accepts-both-sqlx-database-pools-and-tr
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not init pg pool");

    sqlx::query("BEGIN")
        .execute(&pool)
        .await
        .expect("BEGIN tx failed");

    let pool_copy_for_rollback = pool.clone();

    let listener =
    std::net::TcpListener::bind("127.0.0.1:7777").expect("Failed to create listener");

    let wrapped_pool_for_extractor = actix_web::web::Data::new(pool);
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .route(
                "/subscribe",
                actix_web::web::post().to(zero2prod::routes::subscribe),
            )
            .app_data(wrapped_pool_for_extractor.clone())
    })
    .listen(listener)
    .expect("Failed to start app")
    .run();

    tokio::spawn(server);

    // START:TEST_CASE
    // Arrange
    let client = reqwest::Client::new();

    // Act
    let body = "name=John%20Doe&email=john.doe%40example.com"; // https://www.urlencoder.org
    let response = client
        .post("http://127.0.0.1:7777/subscribe")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to POST at /subscribe");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&pool_copy_for_rollback)
        .await
        .expect("Could not fetch subscriptions");

    assert_eq!("John Doe", saved.name);
    assert_eq!("john.doe@example.com", saved.email);
    // END:TEST_CASE

    // Cleanup
    sqlx::query("ROLLBACK")
        .execute(&pool_copy_for_rollback)
        .await
        .expect("ROLLBACK tx failed");
}


#[tokio::test]
async fn integration_test_wrapped_in_a_transaction2() {
    // Ideas from: https://stackoverflow.com/questions/65370752/how-do-i-create-an-actix-web-server-that-accepts-both-sqlx-database-pools-and-tr
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not init pg pool");

    sqlx::query("BEGIN")
        .execute(&pool)
        .await
        .expect("BEGIN tx failed");

    let pool_copy_for_rollback = pool.clone();

    let listener =
    std::net::TcpListener::bind("127.0.0.1:7778").expect("Failed to create listener");

    let wrapped_pool_for_extractor = actix_web::web::Data::new(pool);
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .route(
                "/subscribe",
                actix_web::web::post().to(zero2prod::routes::subscribe),
            )
            .app_data(wrapped_pool_for_extractor.clone())
    })
    .listen(listener)
    .expect("Failed to start app")
    .run();

    tokio::spawn(server);

    // START:TEST_CASE
    // Arrange
    let client = reqwest::Client::new();

    // Act
    let body = "name=John%20Doe&email=john.doe%40example.com"; // https://www.urlencoder.org
    let response = client
        .post("http://127.0.0.1:7778/subscribe")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to POST at /subscribe");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&pool_copy_for_rollback)
        .await
        .expect("Could not fetch subscriptions");

    assert_eq!("John Doe", saved.name);
    assert_eq!("john.doe@example.com", saved.email);
    // END:TEST_CASE

    // Cleanup
    sqlx::query("ROLLBACK")
        .execute(&pool_copy_for_rollback)
        .await
        .expect("ROLLBACK tx failed");
}

#[tokio::test]
async fn integration_test_wrapped_in_a_transaction3() {
    // Ideas from: https://stackoverflow.com/questions/65370752/how-do-i-create-an-actix-web-server-that-accepts-both-sqlx-database-pools-and-tr
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not init pg pool");

    sqlx::query("BEGIN")
        .execute(&pool)
        .await
        .expect("BEGIN tx failed");

    let pool_copy_for_rollback = pool.clone();

    let listener =
    std::net::TcpListener::bind("127.0.0.1:7779").expect("Failed to create listener");

    let wrapped_pool_for_extractor = actix_web::web::Data::new(pool);
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .route(
                "/subscribe",
                actix_web::web::post().to(zero2prod::routes::subscribe),
            )
            .app_data(wrapped_pool_for_extractor.clone())
    })
    .listen(listener)
    .expect("Failed to start app")
    .run();

    tokio::spawn(server);

    // START:TEST_CASE
    // Arrange
    let client = reqwest::Client::new();

    // Act
    let body = "name=John%20Doe&email=john.doe%40example.com"; // https://www.urlencoder.org
    let response = client
        .post("http://127.0.0.1:7779/subscribe")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to POST at /subscribe");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&pool_copy_for_rollback)
        .await
        .expect("Could not fetch subscriptions");

    assert_eq!("John Doe", saved.name);
    assert_eq!("john.doe@example.com", saved.email);
    // END:TEST_CASE

    // Cleanup
    sqlx::query("ROLLBACK")
        .execute(&pool_copy_for_rollback)
        .await
        .expect("ROLLBACK tx failed");
}