# Start from a lightweight Rust image
FROM rust:latest AS builder

# Install required packages
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    clang \
    pkg-config \
    cmake \
    libssl-dev \
    libblas-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new directory for the project
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock first to leverage Docker caching
COPY ./Cargo.toml .

COPY ./src src

# Build dependencies separately to optimize layer caching
RUN cargo fetch

# Copy the rest of the source code
COPY . .

# Build the project in release mode
RUN cargo build --release

# Final stage: create a minimal image with the compiled binary
FROM debian:buster-slim

# Install any runtime dependencies (if needed)
RUN apt-get update && \
    apt-get install -y libssl1.1 && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/bitnet-rs /usr/local/bin/bitnet-rs

# Specify the binary as the entrypoint
ENTRYPOINT ["/usr/local/bin/bitnet-rs"]

