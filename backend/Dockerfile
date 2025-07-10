# Backend Dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/laune-backend ./
COPY --from=builder /app/migrations ./migrations

# Create non-root user
RUN useradd -r -s /bin/false laune
USER laune

EXPOSE 8080

CMD ["./laune-backend"]
