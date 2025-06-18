FROM lukemathwalker/cargo-chef:0.1.71-rust-1.87.0-alpine3.22 AS chef
RUN apk update && apk add --no-cache libpq-dev protoc
WORKDIR /app/

FROM chef AS planner
COPY ./ ./
RUN cargo chef prepare

FROM chef AS dev
COPY --from=planner /app/recipe.json ./
RUN cargo chef cook
COPY ./ ./
CMD ["cargo", "run"]
