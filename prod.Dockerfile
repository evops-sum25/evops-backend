FROM rust:1.87.0-alpine3.22 AS builder
RUN apk update && apk add --no-cache musl-dev protoc
COPY ./ ./
RUN cargo install --path=crates/evops/

FROM alpine:3.22
COPY .env ./
COPY --from=builder /usr/local/cargo/bin/evops /usr/local/bin/
ENTRYPOINT ["evops"]
