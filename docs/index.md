# Demo Rust API

Welcome to the **Demo Rust API** documentation. This service provides a RESTful API for managing items, built with Rust and the Axum framework.

## Overview

The Demo Rust API is a lightweight, high-performance microservice designed for:

- 🦀 **Speed** — Built with Rust for maximum performance
- 📦 **Simplicity** — Clean CRUD operations for item management
- 🐳 **Containerized** — Docker-ready for easy deployment
- 🔒 **Safe** — Rust's memory safety guarantees

## Quick Links

| Resource | Description |
|----------|-------------|
| [Getting Started](getting-started.md) | How to run the API locally |
| [API Reference](api-reference.md) | Complete endpoint documentation |
| [Architecture](architecture.md) | System design and tech stack |

## Architecture at a Glance

```
┌─────────────────────────────────────────────────────────┐
│                    Demo Rust API                        │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐  │
│  │   Axum      │───▶│   Handlers  │───▶│  In-Memory  │  │
│  │   Router    │    │   (CRUD)    │    │    Store    │  │
│  └─────────────┘    └─────────────┘    └─────────────┘  │
│         │                                               │
│         ▼                                               │
│  ┌─────────────────────────────────────────────────┐    │
│  │              Middleware Layer                   │    │
│  │  • CORS • Tracing • Request Logging             │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
```

## Tech Stack

| Technology | Purpose |
|------------|---------|
| **Rust** | Programming language |
| **Axum** | Web framework |
| **Tokio** | Async runtime |
| **Serde** | Serialization/deserialization |
| **Tower-HTTP** | Middleware (CORS, tracing) |

## Service Information

- **Default Port**: `3030`
- **Health Endpoint**: `/health`
- **API Version**: `v1`
- **Base Path**: `/api/v1`

## Status

This service is currently in **experimental** lifecycle stage and is actively being developed.

