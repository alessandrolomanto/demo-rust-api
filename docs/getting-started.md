# Getting Started

This guide will help you get the Demo Rust API up and running on your local machine.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 1.75+** — Install via [rustup](https://rustup.rs/)
- **Docker** (optional) — For containerized deployment

### Verify Rust Installation

```bash
rustc --version
cargo --version
```

## Running Locally

### Option 1: Using Cargo

```bash
# Navigate to the project directory
cd demo-rust-api

# Run in development mode
cargo run

# Or run in release mode (optimized)
cargo run --release
```

The API will start on `http://localhost:3030`.

### Option 2: Using Docker

```bash
# Build the Docker image
docker build -t demo-rust-api .

# Run the container
docker run -p 3030:3030 demo-rust-api
```

## Verify It's Working

Once the server is running, test the health endpoint:

```bash
curl http://localhost:3030/health
```

Expected response:

```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2026-01-09T12:00:00Z"
}
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3030` | The port the server listens on |
| `RUST_LOG` | `demo_rust_api=debug,tower_http=debug` | Log level configuration |

### Setting Environment Variables

```bash
# Linux/macOS
export PORT=8080
export RUST_LOG=info
cargo run

# Or inline
PORT=8080 RUST_LOG=info cargo run
```

## What's Next?

- Check out the [API Reference](api-reference.md) for all available endpoints
- Learn about the [Architecture](architecture.md)

