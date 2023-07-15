FROM rust:1.70-slim-bullseye AS builder

WORKDIR /usr/app
ENV SQLX_OFFLINE=true

# Update compiler dependencies
RUN apt update && \
    apt install -y pkg-config musl-tools musl-dev && \
    rustup target add x86_64-unknown-linux-musl && \
    rm -rf /var/lib/apt/lists/*

RUN cargo init

# Fetch dependencies
COPY Cargo.toml /usr/app/Cargo.toml
RUN cargo fetch

# Copy src files
COPY ./sql /usr/app/sql
COPY ./src /usr/app/src
COPY ./sqlx-data.json /usr/app/sqlx-data.json

# Build binary with musl
RUN cargo build --target x86_64-unknown-linux-musl --release --offline

FROM scratch as production

COPY --from=builder /usr/app/target/x86_64-unknown-linux-musl/release/city-api /app/city-api

CMD ["/app/city-api"]
