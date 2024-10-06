# Flastex DB

**Flastex DB** provides a persistence layer for the Flastex BPM engine using **Sea-ORM**. This crate allows users to store and query BPM process states, history, and task results in a relational database.

## Features
- Persistent storage of BPM process states
- Integration with **Sea-ORM** for database operations
- Database migrations

## Getting Started
To include **Flastex DB**, add the following to your `Cargo.toml`:
```toml
[dependencies]
flastex-db = "0.1"
flastex-core = { path = "../flastex-core" }
sea-orm = "0.4"
```

## Usage
```rust
use flastex_db::DbConnection;

let db = DbConnection::new("sqlite::memory:").await?;
db.migrate().await?;
```

## Roadmap
- Support for NoSQL databases
- Custom database backends

## Contributing
We encourage contributions! Please review our [contributing guidelines](../CONTRIBUTING.md) before submitting a pull request.