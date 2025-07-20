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

### Dependencies

1. `evops-db`: Database operations (PostgreSQL)
2. `evops-storage`: File storage (MinIO)
3. `evops-ml-client`: Machine learning service integration
4. `evops-models`: Domain data structures