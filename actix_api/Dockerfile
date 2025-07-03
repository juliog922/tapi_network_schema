### Multi-stage Dockerfile for API Gateway

# 1. Builder stage
FROM rust:1.86-slim AS builder
WORKDIR /usr/src/actix_api

# Instala dependencias necesarias
RUN apt-get update \
    && apt-get install -y pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copia los manifiestos y código
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
COPY ./data ./data
COPY ./migrations ./migrations

# Compila el binario en release
RUN cargo build --release

# 2. Runtime stage
FROM debian:bookworm-slim

# Instala solo las librerías necesarias en producción
RUN apt-get update \
    && apt-get install -y \
       libssl3 \
       ca-certificates \
       iputils-ping \
    && rm -rf /var/lib/apt/lists/*

# Crea el directorio esperado por el binario
WORKDIR /app

# Copia el binario compilado
COPY --from=builder /usr/src/actix_api/target/release/server /usr/local/bin/server

# Copia la carpeta de datos
COPY --from=builder /usr/src/actix_api/data ./data

# Expone el puerto usado por Actix
EXPOSE 8081

# Usa el binario como punto de entrada
ENTRYPOINT ["/usr/local/bin/server"]
