# Stage 1: Build the Rust app
FROM rust:1.77 as builder

WORKDIR /app
COPY . .

# Install OpenSSL dev libs so Rust can link against OpenSSL 1.1
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build --release

# Stage 2: Runtime image
FROM debian:bullseye-slim

# Install OpenSSL 1.1 for runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/maid-lang-web-backend .
COPY static ./static

EXPOSE 3000

CMD ["./maid-lang-web-backend"]
