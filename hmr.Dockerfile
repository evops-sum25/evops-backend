FROM lukemathwalker/cargo-chef:0.1.71-rust-1.87.0-alpine3.22 AS chef
RUN cargo install bacon@3.15.0
RUN apk update && apk add --no-cache protoc
WORKDIR /app/

FROM chef AS planner
COPY ./ ./
RUN cargo chef prepare

FROM chef AS dev
COPY --from=planner /app/recipe.json ./
RUN cargo chef cook
COPY ./ ./
CMD ["bacon", "--headless", "run"]
