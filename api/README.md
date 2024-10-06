
# Flastex API

**Flastex API** provides an HTTP REST interface for interacting with the Flastex BPM engine. Built on top of **Axum**, it enables users to manage processes, query the status of workflows, and interact with the engine over HTTP.

## Features
- REST API for BPM process management
- Start, stop, and query processes via HTTP
- Built with **Axum** for performance and scalability

## Getting Started
To integrate the **Flastex API**, add the following to your `Cargo.toml`:
```toml
[dependencies]
flastex-api = "0.1"
flastex-core = { path = "../flastex-core" }
```

## Usage
```rust
use flastex_api::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
```

## Roadmap
- API authentication and authorization
- WebSocket support for real-time process updates

## Contributing
Feel free to fork the repository and submit pull requests! Check the [contributing guidelines](../CONTRIBUTING.md) for more details.