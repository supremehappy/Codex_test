# Codex_test

This repository contains a minimal Axum based REST API example.

## Running the server

```
cargo run -p rest_api
```

The server exposes two endpoints:

- `GET /sample/getHello` returns a simple greeting message.
- `POST /sample/postContent` accepts JSON `{ "content": "..." }` and echoes it back.

## Running tests

```
cargo test -p rest_api
```
