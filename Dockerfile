# Stage 1: Build the Rust app
FROM rust:latest as builder

WORKDIR /app
COPY . .

# Build the project in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bullseye-slim  # changed from buster-slim

# Install required runtime dependencies (OpenSSL, etc.)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy the compiled binary and static files
COPY --from=builder /app/target/release/maid-lang-web-backend .
COPY static ./static

# Expose Axum listening port
EXPOSE 3000

# Start the server
CMD ["./maid-lang-web-backend"]
