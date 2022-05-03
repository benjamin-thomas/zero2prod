## Setup

Auto-reload the server

```bash
cargo watch -x check -x run
```

Auto-reload the client

```bash
echo target/debug/zero2prod | entr http localhost:8000/
```

## Observe macro expansion

```bash
cargo install cargo-expand
cargo expand --color=always | less -RS
```