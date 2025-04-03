# Indexer Core Service

## Overview

`indexer-core-service` is a Rust-based microservice built using the [Actix-Web](https://actix.rs/) framework. It serves as the core indexing service in the microservices architecture.

## Features

- High-performance API using Actix-Web.
- Modular design for scalability and maintainability.
- Health check endpoint at `/api/health`.

## How to Run

1. **Install Rust**:
   Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/).

2. **Run the Service**:
   ```bash
   cd indexer-core-service
   cargo run