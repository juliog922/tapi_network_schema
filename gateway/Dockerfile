### Multi-stage Dockerfile for API Gateway

# 1. Builder stage
FROM rust:1.86-slim AS builder
WORKDIR /usr/src/gateway

# Install build dependencies
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies by copying manifests first
COPY Cargo.toml Cargo.lock ./
COPY ./config/config.yaml ./config/config.yaml
COPY ./src ./src
COPY ./build.rs ./build.rs
RUN cargo build --release


# 2. Runtime stage
FROM debian:bookworm-slim

# Install only required runtime libraries
RUN apt-get update \
    && apt-get install -y libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /usr/src/gateway/target/release/gateway /usr/local/bin/gateway

# Expose the port and set entrypoint
EXPOSE 8081