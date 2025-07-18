# `evops-core`

## Overview

The core service layer containing business logic and integrations. Orchestrates database operations (via `evops_db`), file storage (MinIO via `evops_storage`), and ML services (via `evops_ml_client`). Used by `evops-rest` and `evops-grpc`.

### Key Files

- `lib.rs`: 
Initializes application state and dependencies
    - Sets up API routes
    - Key components:
        - `AppState`: Shared application state (thread-safe via `Arc`)
        - `State`: Internal struct holding:
            - Database connection (`evops_db::Database`)
            - File storage client (`evops_storage::Storage`)
            - ML service client (`evops_ml_client::MlClient`)
        - Methods:
            - `new()`: Creates application state with DB/storage/ML connections
            - `arc_clone()`: Explicitly clones shared state
- `services.rs`
Aggregates service modules (event, language, tag, user)

### Service Modules

- `event.rs`
Handles event operations and image management.
Key Methods:
    - `list_tags()` - Paginated tag listing
    - `create_tag()` - Creates new tag
    - `get_tags_by_description()` - Predicts tags using ML service
    - `delete_tag()` - Deletes tag

- `user.rs`
Manages user accounts.
Key Methods:
    - `list_users()` - Gets all users
    - `create_user()`	- Creates new user
    - `find_user()` - Gets user by ID

- `language.rs`
Currently placeholder implementation
Future use: Manage multilingual content

### Critical Workspace Dependencies

1. `evops-db`: Database operations (PostgreSQL)
2. `evops-storage`: File storage (MinIO)
3. `evops-ml-client`: Machine learning service integration
4. `evops-models`: Domain data structures
