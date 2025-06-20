# EvOps / Back End

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

## Additional Information

To populate the DB with dummy data, run this command.

```shell
cargo run --package=script-dummy-data
```
