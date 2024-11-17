# Flastex Core

**Flastex Core** is the heart of the Flastex BPM engine. This crate handles the core functionality of BPMN process modeling, task execution, and workflow orchestration. It is lightweight, flexible, and designed to work with or without additional services such as API layers or databases.

## Features
- BPMN 2.0 parsing
- Workflow execution engine
- Task and event handling
- Pluggable architecture for adding additional services (DB, API, etc.)

## Getting Started
To use **Flastex Core** in your project, add the following to your `Cargo.toml`:
```toml
[dependencies]
flastex-core = "0.1"
```

## Usage
```rust
use flastex_core::BpmnEngine;

let engine = BpmnEngine::new();
engine.load_process("path_to_bpmn.xml");
engine.execute();
```

## Roadmap
- BPMN advanced features support
- Asynchronous task execution
- Error handling and recovery

## Contributing
We appreciate your interest in contributing to Flastex! Please follow the guidelines outlined in our [CONTRIBUTING.md](../CONTRIBUTING.md) document for instructions on how to contribute, submit bug reports, or suggest enhancements.