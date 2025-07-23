# `evops-core`

## Overview

Acts as the application's source of truth, containing all core business logic and service integrations. This crate is reused by both `evops-rest` and `evops-grpc` to ensure consistent behavior across API boundaries. Orchestrates database operations (via `evops_db`), file storage (MinIO via `evops_storage`), and ML services (via `evops_ml_client`)

### Core Mechanism

1. **State Initialization**
During `AppState` construction the application is wiring together:
    - Database connections
    - Storage clients
    - ML service integrations

2. **API Layer Integration**
    - REST: All endpoints inject AppState via axum's State mechanism
    - gRPC: Service handlers hold shared references to AppState

3. **Business Logic Execution**
Both REST controllers and gRPC services directly call `AppState` methods like `create_event()`

### Key Files

- `lib.rs`: 
Initializes application state and dependencies
    - Key components:
    - Sets up API routes
        - `AppState`: Shared application state (thread-safe via `Arc`)
        - `State`: Internal struct holding:
            - `evops_db::Database`
            - `evops_storage::Storage`
            - `evops_ml_client::MlClient`

- `services.rs`
Aggregates service modules (event, language, tag, user)

### Service Modules

- `event.rs`
Handles event operations and image management

- `user.rs`
Manages user accounts

- `language.rs`
Currently placeholder implementation
Future use: Manage multilingual content
