# Dockerfile for Yew frontend
FROM rust:latest

# Install rustup and configure the environment
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install trunk and dependencies
RUN rustup update
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update && apt-get install -y ca-certificates
RUN cargo install --locked trunk

# Create and set working directory
WORKDIR /app

# Copy project files
COPY . .

