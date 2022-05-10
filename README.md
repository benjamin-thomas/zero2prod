# Following along the zero2prod book

## Better IntelliJ/Clio support

Follow this issue: https://github.com/intellij-rust/intellij-rust/issues/6908

For the time being, activate the *experimental* features, as documented.

Then start with the env vars loaded:

```bash
./manage/with_env clion .
```

## Git hook setup

Hook is at: `.git/hooks/pre-push`
```bash
#!/bin/bash

set -e

./manage/run_tests
```

## Catch errors

```bash
cargo check
cargo clippy # catches more than `check`
```

## Auto-reload on code change

Update the server
```bash
# `check` and `clippy` are optional (the latter catches more stuff)
cargo watch --clear -x check -x clippy -x run
LOG=1 ./manage/with_env cargo watch --clear -x run | bunyan

# cargo install bunyan
LOG=1 ./manage/with_env cargo run | bunyan
LOG=1 ./manage/with_env cargo watch --clear -x run | bunyan
```

Re-run the client
```bash
echo target/debug/zero2prod | entr -c http localhost:8000/
while true;do http --form POST localhost:8000/subscribe name=John email=john-$(date +%s)@example.com;sleep 5;done
```

## Testing

```bash
# This incantation will update the current tmux pane green/red
cargo watch -- bash -c './manage/with_env cargo test;./manage/tmux_warn $?'
LOG=1 cargo watch --clear -- bash -c './manage/with_env cargo test;./manage/tmux_warn $?' | bunyan

# Env vars can be loaded from any of those two processes
LOG=1 cargo watch -- ./manage/with_env cargo test
cargo watch -- LOG=1 ./manage/with_env cargo test

# This is the more standard way, no tmux green/red update though
cargo watch -x "check --lib --test health_test"
cargo watch -x check -x clippy -x test
./manage/with_env cargo watch --clear -x test
./manage/with_env cargo watch --clear -x 'test -- --nocapture'

# cargo install bunyan
LOG=1 ./manage/with_env cargo test | bunyan
```

## Observe macro expansion

```bash
cargo install cargo-expand
cargo expand --color=always | less -RS
```

## Add a dependency

```bash
cargo install cargo-edit
cargo add reqwest --dev
```

## Generate and open docs in the browser

```bash
cargo doc --open
```

## Database migrations

```bash
# As documented here: https://crates.io/crates/sqlx-cli
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Create a first **reversible** migration.
# sqlx won't allow mixing reversible and non-reversible migrations in the same project
./manage/sqlx migrate add -r create_subscriptions_table
./manage/sqlx migrate revert
```

## Load environment variables for dev processes

```bash
./manage/with_env psql
./manage/with_env env | grep ^PG

# I loose color capability detection unfortunately
./manage/with_env ls -l --color=always

./manage/with_env cargo test
./manage/with_env cargo watch -x test
```

## Testing

Run a specific test
```bash
./manage/with_env cargo test valid_form_data
./manage/with_env cargo test subscribe_returns_a_200_for_valid_form_data -- --exact

./manage/with_env cargo watch --clear -x 'test -- --nocapture'
```

## Find unused dependencies

```bash
cargo install cargo-udeps
cargo +nightly udeps
```