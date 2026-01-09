# Demo Rust API

A demo REST API built with Rust and [Axum](https://github.com/tokio-rs/axum) framework, designed for integration with Backstage.

## Features

- 🦀 Built with Rust and Axum
- 📦 CRUD operations for items
- 🔍 Health check endpoint
- 🐳 Docker ready
- 📚 Backstage catalog integration

## Quick Start

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Docker (optional)

### Running Locally

```bash
# Clone and navigate to the project
cd demo-rust-api

# Run the application
cargo run

# Or run in release mode
cargo run --release
```

The API will be available at `http://localhost:3030`

### Running with Docker

```bash
# Build the image
docker build -t demo-rust-api .

# Run the container
docker run -p 3030:3030 demo-rust-api
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/v1/items` | List all items |
| POST | `/api/v1/items` | Create a new item |
| GET | `/api/v1/items/{id}` | Get item by ID |
| PUT | `/api/v1/items/{id}` | Update an item |
| DELETE | `/api/v1/items/{id}` | Delete an item |

## Usage Examples

### Health Check

```bash
curl http://localhost:3030/health
```

Response:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2026-01-09T12:00:00Z"
}
```

### Create an Item

```bash
curl -X POST http://localhost:3030/api/v1/items \
  -H "Content-Type: application/json" \
  -d '{"name": "My Item", "description": "A test item"}'
```

### List All Items

```bash
curl http://localhost:3030/api/v1/items
```

### Get an Item

```bash
curl http://localhost:3030/api/v1/items/{uuid}
```

### Update an Item

```bash
curl -X PUT http://localhost:3030/api/v1/items/{uuid} \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Name"}'
```

### Delete an Item

```bash
curl -X DELETE http://localhost:3030/api/v1/items/{uuid}
```

## Backstage Integration

This project includes a `catalog-info.yaml` file for Backstage integration. To register this component in Backstage:

1. Navigate to your Backstage instance
2. Go to **Create** → **Register Existing Component**
3. Enter the URL to the `catalog-info.yaml` file
4. Follow the prompts to complete registration

