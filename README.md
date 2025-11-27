# {{project-name}}

A simple Rust project template to kickstart your development.

## Structure

- `src/main.rs`: The main entry point of the application.
- `src/utils.rs`: Utility functions with unit tests.
- `src/error.rs`: Custom error handling using `derive_more`.
- `examples/`: Directory for example code.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed. If not, download them from [rustup.rs](https://rustup.rs/).

### Running the Application

To run the main application:

```sh
cargo run
```

### Running Examples

To run the provided example:

```sh
cargo run --example c01-simple
```

### Running Tests

To run the unit tests:

```sh
cargo test
```

### Using cargo-generate

Use `cargo-generate` to create a new project based on this template:

```sh
# cargo install cargo-generate

cargo generate --git https://github.com/wzzju/rust-project-template.git --name your-project
```

## Dependencies

- [derive_more](https://crates.io/crates/derive_more): Used for ergonomic error handling.
