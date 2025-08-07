# Build with nightly Rust
FROM rustlang/rust:nightly-slim as builder

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build --release

# Runtime stage with OpenSSL 3 (Debian Bookworm)
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/maid-lang-web-backend .
COPY static ./static

EXPOSE 3000
CMD ["./maid-lang-web-backend"]
