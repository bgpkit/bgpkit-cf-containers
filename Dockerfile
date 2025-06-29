# ---- Build Stage ----
FROM rust:1.86 AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Cache dependencies
COPY container-src/Cargo.lock container-src/Cargo.toml ./
COPY container-src/src ./src
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the statically linked binary from the builder
COPY --from=builder /app/target/release/bgpkit-cf-container /app/bgpkit-cf-container

EXPOSE 3000

CMD ["/app/bgpkit-cf-container"]