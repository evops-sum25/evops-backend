FROM lukemathwalker/cargo-chef:0.1.71-rust-1.87.0-slim-bookworm AS chef
RUN cargo install bacon@3.15.0
RUN apt-get update && apt-get install -y libpq-dev protobuf-compiler && rm -rf /var/lib/apt/lists/*
WORKDIR /app/

FROM chef AS planner
COPY ./ ./
RUN cargo chef prepare

FROM chef AS dev
COPY --from=planner /app/recipe.json ./
RUN cargo chef cook
COPY ./ ./
CMD ["bacon", "--headless", "run"]