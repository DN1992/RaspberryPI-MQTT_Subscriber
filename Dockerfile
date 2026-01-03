FROM rust:latest as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 rustuser
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/mqtt-subscriber /app/mqtt-subscriber
COPY --from=builder /usr/src/app/migrations /app/migrations
RUN chown -R rustuser:rustuser /app
USER rustuser
CMD ["./mqtt-subscriber"]