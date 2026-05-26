# ─── Stage 1: Build ───
FROM rust:1.78-bookworm AS builder

WORKDIR /app
COPY . .

# Build in release mode with all optimizations
RUN cargo build --release --bin ghostlink-server

# ─── Stage 2: Runtime ───
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/ghostlink-server /app/ghostlink-server

# Non-root user for security
RUN useradd -m -s /bin/bash ghostlink
USER ghostlink

EXPOSE 8080

ENTRYPOINT ["/app/ghostlink-server"]
