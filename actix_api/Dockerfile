# Dockerfile for Actix API
FROM rustlang/rust:nightly-slim

# Create and set working directory
WORKDIR /app

# Configure the environment
COPY Cargo.toml Cargo.lock ./
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo fetch

# Copy project files
COPY . .

EXPOSE 8080