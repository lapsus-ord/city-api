FROM rust:1.69-slim-bullseye AS builder

WORKDIR /usr/app

ENV SQLX_OFFLINE=true

RUN apt-get update && \
    apt-get install -y ca-certificates pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*
RUN cargo init

COPY Cargo.toml /usr/app/Cargo.toml
RUN cargo fetch

COPY ./sql /usr/app/sql
COPY ./src /usr/app/src
COPY ./sqlx-data.json /usr/app/sqlx-data.json

RUN cargo build --release --offline

FROM debian:bullseye-slim as production
LABEL org.opencontainers.image.source="https://github.com/lapsus-ord/city-api"

COPY --from=builder /usr/app/target/release/city-api /app/city-api

RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

ENV RUST_LOG="city_api=trace"
CMD ["/app/city-api"]
