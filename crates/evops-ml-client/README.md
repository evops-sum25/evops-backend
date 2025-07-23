# `evops-ml-client`

## Overview

Provides gRPC client implementation for communicating with Machine Learning services. Handles connection management, request serialization, and response parsing.

### Key Files

- `lib.rs`: Core Client Setup
    Main entry point that establishes gRPC connection to ML server.
    Key Responsibilities:
    - Creates `MlClient` struct wrapping gRPC channel
    - Implements connection retry logic with 5-second intervals
    - Initializes gRPC client stub

- `logic.rs`: Business Logic
    Contains domain-specific operations and data transformations.
    1. Converts `EventDescription` to protobuf format
    2. Executes gRPC call to ML service

- `pb.rs`: Protobuf Bindings
Auto-generated file (via `tonic-build`) containing:
    - gRPC client stubs (`MlServiceClient`)
    - Protobuf message definitions