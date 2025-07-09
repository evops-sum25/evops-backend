FROM lukemathwalker/cargo-chef:0.1.71-rust-1.87.0-slim-bookworm AS chef
RUN apt-get update && apt-get install -y wget unzip && \
    wget https://github.com/protocolbuffers/protobuf/releases/download/v26.1/protoc-26.1-linux-x86_64.zip && \
    unzip protoc-26.1-linux-x86_64.zip -d /usr/local && \
    rm protoc-26.1-linux-x86_64.zip
RUN apt-get install -y libssl-dev pkg-config libpq-dev
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
WORKDIR /app/server-ext
RUN cargo chef prepare --recipe-path recipe.json
WORKDIR /app/server-ext/client-ext
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS dev
COPY --from=planner /app/recipe.json /app/
COPY --from=planner /app/server-ext/recipe.json /app/server-ext/
COPY --from=planner /app/server-ext/client-ext/recipe.json /app/server-ext/client-ext/
WORKDIR /app/server-ext/client-ext
RUN cargo chef cook --recipe-path recipe.json
WORKDIR /app/server-ext
RUN cargo chef cook --recipe-path recipe.json
WORKDIR /app
RUN cargo chef cook --recipe-path recipe.json
COPY . /app
WORKDIR /app/
CMD ["cargo", "run"]