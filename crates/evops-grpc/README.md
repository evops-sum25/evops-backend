# `evops-grpc`

## Overview

Provides gRPC API endpoints using `tonic` framework. Handles protocol buffer serialization, gRPC-web compatibility, and service implementations. Organized into:
- Protocol buffer definitions (`pb`)
- Service implementations (`services`)
- gRPC-specific utilities (`headers`)
### Key Files

- `lib.rs`: Main entry point
    - Creates gRPC router with `axum`
    - Registers all services
    - Sets up gRPC reflection service for debugging
    - Configures CORS with gRPC-specific headers

- `pb.rs`: Protocol Buffer Hub
    - Auto-generated protobuf code
    - Contains all gRPC service and message definitions
    - Includes file descriptor set for reflection
    - Manual validation implementations

 - `pb/impls.rs`: Protobuf ↔ Model Conversions
    - Handles validation and type conversions
    - Key responsibilities
        - String to UUID conversion
        - Field validation
        - Model to gRPC response transformations

- `services/event.rs`: Event Service
    - Implements gRPC methods:
        - CRUD operations
        - Image management (push/find/delete)

- `services/language.rs`: Language Service

- `services/tag.rs`: Tag Service
    - Core methods:
        - `create/list/find/delete`
        - `suggest` - ML-powered tag suggestions`

- `services/user.rs`: User Service
    - Standard CRUD operations
    - UUID handling for all requests

- `headers.rs`: gRPC Header Constants
    - Defines standard gRPC headers
    - Used for CORS configuration and error handling

- `services.rs`: Service Registry
Aggregates all service implementations