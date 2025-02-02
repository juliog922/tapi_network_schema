# Dockerfile for Yew frontend
FROM rustlang/rust:nightly-slim

# Create and set working directory
WORKDIR /app

# Configure the environment
COPY Cargo.toml Cargo.lock ./
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo fetch

# Install trunk and dependencies
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y ca-certificates
RUN cargo install --locked trunk

# Copy project files
COPY . .

EXPOSE 8092

