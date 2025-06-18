FROM rust:1.87.0-bookworm AS builder
RUN apt-get update && apt-get install --assume-yes protobuf-compiler
COPY ./ ./
RUN cargo install --path=crates/evops/

FROM debian:bookworm-slim
RUN apt-get update && apt-get install --assume-yes libpq-dev
COPY --from=builder /usr/local/cargo/bin/evops /usr/local/bin/
ENTRYPOINT ["evops"]
