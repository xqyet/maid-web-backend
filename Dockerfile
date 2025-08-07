# Stage 1: Build the Rust app
FROM rust:latest as builder

WORKDIR /app
COPY . .

# Build the project in release mode
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:buster-slim

# Create app directory in container
WORKDIR /app

# Copy only the final executable from the builder stage
COPY --from=builder /app/target/release/maid-lang-web-backend .

# Copy the static files directory
COPY static ./static

# Expose the port your Axum app listens on
EXPOSE 3000

# Start the web server
CMD ["./maid-lang-web-backend"]
