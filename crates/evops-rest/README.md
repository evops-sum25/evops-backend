# evops-rest

## Overview

Provides REST API endpoints using `axum` crate. Automatically generates OpenAPI documentation using `aide` crate. Organized into logical service groups (events, tags, users, etc.).

### Key Files

- `lib.rs`: Main entry point
    - Sets up API routes
    - Configures OpenAPI generation

- `routes/`: Endpoint implementations
    - Each file handles related operations
    - Uses file-system routing (nested routes like `/tags/{tag-id}`)

- `docs.rs`: OpenAPI configuration
    - Defines API tags (e.g., `"TagService"`, `"UserService"`)
    - Configures OpenAPI 3.0 schema generation

- `types.rs`: Request/response data structures
    - Request/Response types:
    - Strict validation:
        ```rust
        #[schemars(length(min = 2, max = 50))]
        struct TagName(String);
        ```
    - Pagination support:
        - `last_id`, `limit` `parameters`

- `error.rs`: Error handling
    Standardizes API errors:
    - `400` Bad Request
    - `404` Not Found
    - `409` Conflict
    - `422` Validation Error
    - `500` Server Error

### Adding New Endpoint

1. 
