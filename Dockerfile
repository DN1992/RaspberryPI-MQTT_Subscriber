# -------- Builder stage --------
    FROM rust:1.82 AS builder

    WORKDIR /usr/src/app
    
    # Copy manifests first for better layer caching
    COPY Cargo.toml Cargo.lock* ./
    RUN mkdir src && echo "fn main() {}" > src/main.rs
    RUN cargo build --release
    RUN rm -rf src
    
    # Copy actual source and build
    COPY src ./src
    RUN cargo build --release
    
    # -------- Runtime stage --------
    FROM debian:bullseye-slim
    
    RUN apt-get update \
        && apt-get install -y ca-certificates \
        && rm -rf /var/lib/apt/lists/*
    
    WORKDIR /app
    
    COPY --from=builder /usr/src/app/target/release/mqtt_subscriber .
    
    # SQLite database will live here
    VOLUME ["/app/data"]
    
    CMD ["./mqtt_subscriber"]
    