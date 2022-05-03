# Following along the zero2prod book

## Catch errors

```bash
cargo check
cargo clippy # catches more than `check` 
```

## Auto-reload on code change

Update the server
```bash
# `check` and `clippy` are optional (the latter catches more stuff)
cargo watch -x check -x clippy -x run
```

Re-run the client
```bash
echo target/debug/zero2prod | entr http localhost:8000/
```

## Testing

```bash
cargo watch -x "check --lib --test health_test"
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