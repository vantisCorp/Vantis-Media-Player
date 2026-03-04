# Multi-stage Docker build for Vantis Media Player

# Stage 1: Builder
FROM rust:1.93-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ffmpeg \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source
COPY . .

# Build release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libavcodec58 \
    libavformat58 \
    libavutil56 \
    libswscale5 \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 vantis

# Copy binary from builder
COPY --from=builder /app/target/release/vantis /app/vantis

# Copy documentation
COPY README.md /app/
COPY CHANGELOG.md /app/

# Create directories
RUN mkdir -p /home/vantis/.vantis/plugins \
    /home/vantis/.vantis/cache \
    /home/vantis/.vantis/config \
    && chown -R vantis:vantis /home/vantis/.vantis

# Set permissions
RUN chmod +x /app/vantis

# Switch to non-root user
USER vantis

# Set environment
ENV VANTIS_CONFIG_DIR=/home/vantis/.vantis/config
ENV VANTIS_CACHE_DIR=/home/vantis/.vantis/cache
ENV VANTIS_PLUGIN_DIR=/home/vantis/.vantis/plugins

# Expose port for potential web interface
EXPOSE 8080

# Run the application
ENTRYPOINT ["/app/vantis"]
CMD ["--help"]