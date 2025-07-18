# `evops`

## Overview

The main executable crate serving as the backend entry point. Handles:
- Configuration loading
- Logging initialization
- Server startup (REST + gRPC)
- Graceful shutdown
- Dependency integration

### Key Files

- `config.rs`
Loads configuration from environment variables:
    - Validates URL formats
    - Uses `eyre` for error handling

- `main.rs`
Entry point with async runtime setup
Key components:
    - `init_logging()`: Configures `tracing` with env-based filters
    - `tcp_listener()`: Binds to all interfaces on specified port
    - `rest_grpc_app()`: Combines REST and gRPC routers

### Key Dependencies

1. `evops-core`
Core business logic crate. Contains shared application state, database/storage/ML integrations, and domain logic reused across REST/gRPC.

2. `evops-rest`
Dedicated REST API implementation. Defines HTTP routes, request/validation handlers, and OpenAPI 3 docs.

3. `evops-grpc`
gRPC service definitions and implementations. Generates protocol buffers code and handles RPC calls.

4. `axum`
Modern web framework for building REST APIs. Handles HTTP routing, middleware, and request/response processing.

5. `axum_tonic`
Critical integration crate that enables simultaneous REST + gRPC services on the same port. Coordinates traffic between the two protocols.

6. `tokio`
Async runtime. Enables concurrent operations.