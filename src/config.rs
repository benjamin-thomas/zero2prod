use std::env;

pub fn must_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("Env var '{}' not found!", name))
}

pub fn get_conn_string() -> String {
    format!(
        "postgres://{}:{}@{}:5432/{}",
        must_env("PGUSER"),
        must_env("PGPASSWORD"),
        must_env("PGHOST"),
        must_env("PGDATABASE"),
    )
}
