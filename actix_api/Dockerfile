# Dockerfile for Actix API
FROM rust:latest

# Install system dependencies for OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables for OpenSSL
ENV OPENSSL_DIR=/usr/lib/x86_64-linux-gnu
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig

# Install rustup and configure the environment
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Rust toolchain
RUN rustup update && rustup toolchain install stable && rustup default stable


# Install Rust tools
RUN rustup update
RUN rustup toolchain install stable
RUN rustup default stable

# Create and set working directory
WORKDIR /app

# Copy project files
COPY . .
RUN cargo fetch

EXPOSE 8080