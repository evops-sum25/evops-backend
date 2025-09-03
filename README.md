# EvOps / Back End

Main repository: https://github.com/evops-sum25/evops

## Features

- API methods to create and view users, events, event images, and event tags
  (groups).
- Combined REST+gRPC web-server so that clients can choose their preferred
  communication way.
- Auto-generated OpenAPI documentation + Swagger webpage.
- Persistent data storage with PostgreSQL.
- Persistent image storage with MinIO.
- Data validation at the type-system level.
- Basic logging of incoming requests and their responses.
- Configurability with environment variables and a [`.env`](/.env.example) file.
- An [API extension](https://github.com/evops-sum25/evops-client-ext) for
  clients to ease the interaction with our back end:
  - Validation functions (such as `validate_event_title` or
    `validate_tag_alias`) that reuse the logic from the back-end core.
  - Fully-typed Markdown parser with our own parse settings.
  - Exporting to WASM with Extism and Protobuf types.
  - Generating type-safe FFI bindings with UniFFI.

## Screenshots

<img src="/assets/swagger-1.jpg" width=480 />
<img src="/assets/swagger-2.jpg" width=480 />
<img src="/assets/grpcui.jpg" width=480 />

## Build Instructions

1. Create a [`.env`](`/.env.example`).

   ```shell
   cp .env.example .env
   ```

2. Run the server.

   - Development mode:

     ```shell
     bacon
     ```

   - Release mode:

     ```shell
     cargo run --release
     ```

3. Both the REST and gRPC servers should be available on http://0.0.0.0:8080 by
   default.
