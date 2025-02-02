# Dockerfile for Actix API
FROM rustlang/rust:nightly-slim

# Create and set working directory
WORKDIR /app

# Configure the environment
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy project files
COPY . .
RUN cargo fetch

EXPOSE 8080