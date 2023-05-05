FROM rust:1.69-bullseye AS base

WORKDIR /usr/app

RUN cargo init
COPY Cargo.toml /usr/app/Cargo.toml

RUN cargo fetch
COPY ./src /usr/app/src

# Stage DEV: Start app
FROM base AS development

RUN apt-get update
RUN apt-get install -y ca-certificates
RUN rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch
RUN cargo install sqlx-cli

RUN cargo build --offline

ENV RUST_LOG="city_api=trace"
CMD ["cargo", "watch", "-x", "run"]

# Stage PROD: Start app
FROM base AS builder

RUN cargo build --release --offline

FROM debian:bullseye-slim

COPY --from=builder /usr/app/target/release/city-api /city-api

ENV RUST_LOG="city_api=trace"
CMD ["/city-api"]
