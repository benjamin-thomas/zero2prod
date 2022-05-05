use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::startup;

#[tokio::test]
async fn health_works() {
    // Arrange
    let pg_pool = get_pg_pool().await;
    let base_url = spawn_app(pg_pool);
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

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let pg_connection = get_pg_pool().await;

    let base_url = spawn_app(pg_connection);
    let client = reqwest::Client::new();
    let mut connection = PgConnection::connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not connect to the database!");

    // Act
    let body = "name=John%20Doe&email=john.doe%40example.com"; // https://www.urlencoder.org
    let response = client
        .post(base_url + "/subscribe")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to POST at /subscribe");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Could not fetch subscriptions");

    assert_eq!("John", saved.name);
    assert_eq!("john.doe@example.com", saved.email);
}

#[tokio::test]
async fn subscribe_returns_a_400_on_missing_data() {
    // Arrange
    let pg_connection = get_pg_pool().await;
    let base_url = spawn_app(pg_connection);
    let endpoint = base_url + "/subscribe";
    let client = reqwest::Client::new();

    let cases = vec![
        ("name=John%20Doe", "missing email"),
        ("email=john.doe%40example.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (body, hint) in cases {
        // Act
        let response = client
            .post(&endpoint)
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
}

async fn get_pg_pool() -> PgPool {
    PgPool::connect(&zero2prod::config::get_conn_string())
        .await
        .expect("Could not connect to the database!")
}

fn spawn_app(pg_pool: PgPool) -> String {
    let listener =
        std::net::TcpListener::bind("localhost:0").expect("Failed to start listener (random port)");

    let port = listener.local_addr().unwrap().port();
    let server = startup::run(listener, pg_pool).expect("Failed to start server");

    tokio::spawn(server);
    return format!("http://localhost:{}", port);
}
