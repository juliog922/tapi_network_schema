# Dockerfile for Actix API
FROM rust:latest

# Install rustup and configure the environment
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Rust tools
RUN rustup update
RUN rustup toolchain install stable
RUN rustup default stable

# Create and set working directory
WORKDIR /app

# Copy project files
COPY . .

