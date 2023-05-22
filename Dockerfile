FROM rust:1.69-slim-bullseye AS base

WORKDIR /usr/app

RUN cargo init

COPY Cargo.toml /usr/app/Cargo.toml

RUN cargo fetch

COPY ./sql /usr/app/sql
COPY ./src /usr/app/src

# Stage DEV: Start app
FROM base AS development

RUN apt-get update && \
    apt-get install -y ca-certificates pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch && \
    cargo install sqlx-cli

ENV RUST_LOG="city_api=trace"
CMD ["cargo", "watch", "-x", "run"]

# Stage PROD: Start app
FROM base AS builder

RUN cargo build --release --offline

FROM debian:bullseye-slim as production

COPY --from=builder /usr/app/target/release/city-api /app/city-api

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

ENV RUST_LOG="city_api=trace"
CMD ["/app/city-api"]
