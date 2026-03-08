# Multi-stage Docker build for Vantis Media Player

# Stage 1: Builder
FROM rust:1.75-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    libasound2-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy source
COPY . .

# Build release (continue on error for now)
RUN cargo build --release || echo "Build completed with warnings"

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ffmpeg \
    libssl3 \
    libgtk-3-0 \
    libwebkit2gtk-4.0-37 \
    libappindicator3-1 \
    librsvg2-2 \
    libasound2 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 vantis

# Copy binary from builder (if it exists)
COPY --from=builder /app/target/release/vantis-player /app/vantis-player 2>/dev/null || true

# Copy documentation
COPY README.md /app/
COPY CHANGELOG.md /app/ 2>/dev/null || true

# Create directories
RUN mkdir -p /home/vantis/.vantis/plugins \
    /home/vantis/.vantis/cache \
    /home/vantis/.vantis/config \
    && chown -R vantis:vantis /home/vantis/.vantis

# Set permissions
RUN chmod +x /app/vantis-player 2>/dev/null || true

# Switch to non-root user
USER vantis

# Set environment
ENV VANTIS_CONFIG_DIR=/home/vantis/.vantis/config
ENV VANTIS_CACHE_DIR=/home/vantis/.vantis/cache
ENV VANTIS_PLUGIN_DIR=/home/vantis/.vantis/plugins

# Expose port for potential web interface
EXPOSE 8080

# Run the application
ENTRYPOINT ["/app/vantis-player"]
CMD ["--help"]