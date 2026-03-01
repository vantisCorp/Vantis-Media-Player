# Docker Deployment Guide

This guide explains how to deploy and run Vantis Media Player using Docker.

## Table of Contents

- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Building the Image](#building-the-image)
- [Running the Container](#running-the-container)
- [Docker Compose](#docker-compose)
- [Configuration](#configuration)
- [Volumes](#volumes)
- [Hardware Acceleration](#hardware-acceleration)
- [Multi-Platform Builds](#multi-platform-builds)
- [Troubleshooting](#troubleshooting)

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisMedia.git
cd VantisMedia

# Create media directory
mkdir -p media

# Copy your media files to media/
cp /path/to/your/video.mp4 media/

# Start the container
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the container
docker-compose down
```

### Using Docker CLI

```bash
# Pull the image (when available)
docker pull vantismedia/vantis-player:latest

# Or build locally
docker build -t vantismedia/vantis-player:latest .

# Run the container
docker run -d \
  --name vantis-player \
  -v $(pwd)/media:/media:ro \
  -v vantis-config:/config \
  -v vantis-cache:/cache \
  -v vantis-plugins:/plugins \
  -p 8080:8080 \
  --device /dev/dri:/dev/dri \
  vantismedia/vantis-player:latest \
  play /media
```

## Prerequisites

### Required

- Docker Engine 20.10 or later
- Docker Compose 2.0 or later (for docker-compose commands)

### Optional (for hardware acceleration)

- Linux host with GPU support
- Intel/AMD/NVIDIA GPU drivers installed
- `/dev/dri` device access

## Building the Image

### Standard Build

```bash
docker build -t vantismedia/vantis-player:latest .
```

### Build Without Cache

```bash
docker build --no-cache -t vantismedia/vantis-player:latest .
```

### Using Makefile

```bash
# Include the Docker Makefile
include Makefile.docker

# Build
make build

# Build without cache
make build-no-cache

# Build specific version
make build-v1.0.0
```

## Running the Container

### Basic Run

```bash
docker run -d \
  --name vantis-player \
  -v $(pwd)/media:/media:ro \
  vantismedia/vantis-player:latest \
  play /media/video.mp4
```

### Run with All Features

```bash
docker run -d \
  --name vantis-player \
  -v $(pwd)/media:/media:ro \
  -v vantis-config:/config \
  -v vantis-cache:/cache \
  -v vantis-plugins:/plugins \
  -p 8080:8080 \
  --device /dev/dri:/dev/dri \
  -e RUST_LOG=info \
  --restart unless-stopped \
  vantismedia/vantis-player:latest \
  play /media
```

### Interactive Run

```bash
docker run -it --rm \
  --name vantis-player \
  -v $(pwd)/media:/media:ro \
  vantismedia/vantis-player:latest \
  play /media/video.mp4
```

### Run with Bash Shell

```bash
docker run -it --rm \
  --name vantis-player \
  -v $(pwd)/media:/media:ro \
  vantismedia/vantis-player:latest \
  bash
```

## Docker Compose

### Basic Usage

```bash
# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down

# Restart services
docker-compose restart

# View running containers
docker-compose ps
```

### Using Makefile

```bash
# Start services
make compose-up

# View logs
make compose-logs

# Stop services
make compose-down

# Restart services
make compose-restart
```

### Profiles

```bash
# Start with web UI (when implemented)
docker-compose --profile web up -d

# Start only the player
docker-compose up -d
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `VANTIS_CONFIG_DIR` | `/config` | Configuration directory |
| `VANTIS_CACHE_DIR` | `/cache` | Cache directory |
| `VANTIS_PLUGIN_DIR` | `/plugins` | Plugin directory |
| `RUST_LOG` | `info` | Log level (error, warn, info, debug, trace) |

### Setting Environment Variables

#### Docker CLI

```bash
docker run -d \
  -e RUST_LOG=debug \
  -e VANTIS_CONFIG_DIR=/config \
  vantismedia/vantis-player:latest
```

#### Docker Compose

```yaml
environment:
  - RUST_LOG=debug
  - VANTIS_CONFIG_DIR=/config
```

### Configuration File

Mount your configuration file to `/config/config.toml`:

```bash
docker run -d \
  -v $(pwd)/config.toml:/config/config.toml:ro \
  vantismedia/vantis-player:latest
```

## Volumes

### Persistent Volumes

| Volume | Description |
|--------|-------------|
| `vantis-config` | Configuration files |
| `vantis-cache` | Cached data |
| `vantis-plugins` | Installed plugins |

### Media Volume

Mount your media directory:

```bash
docker run -d \
  -v $(pwd)/media:/media:ro \
  vantismedia/vantis-player:latest
```

### Custom Volumes

```bash
docker run -d \
  -v my-config:/config \
  -v my-cache:/cache \
  -v my-plugins:/plugins \
  vantismedia/vantis-player:latest
```

## Hardware Acceleration

### GPU Access

The Docker image supports hardware-accelerated video decoding using `/dev/dri`.

### Enable GPU Access

```bash
docker run -d \
  --device /dev/dri:/dev/dri \
  vantismedia/vantis-player:latest
```

### Check GPU Access

```bash
# Run with bash
docker run -it --rm \
  --device /dev/dri:/dev/dri \
  vantismedia/vantis-player:latest \
  bash

# Check for GPU devices
ls -la /dev/dri/
```

### NVIDIA GPU (Optional)

For NVIDIA GPUs, you may need to use the NVIDIA Container Toolkit:

```bash
# Install NVIDIA Container Toolkit
# See: https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html

# Run with NVIDIA runtime
docker run -d \
  --gpus all \
  vantismedia/vantis-player:latest
```

## Multi-Platform Builds

### Build for Multiple Platforms

```bash
# Using Makefile
make build-multi

# Using docker buildx
docker buildx create --use --name vantis-builder
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t vantismedia/vantis-player:latest \
  --push \
  .
```

### Supported Platforms

- `linux/amd64` - x86_64 (Intel/AMD)
- `linux/arm64` - ARM64 (Apple Silicon, ARM servers)

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs vantis-player

# Check container status
docker ps -a

# Run in foreground to see errors
docker run --rm vantismedia/vantis-player:latest
```

### No Sound Output

```bash
# Check audio device access
docker run -it --rm \
  --device /dev/snd:/dev/snd \
  vantismedia/vantis-player:latest \
  bash

# List audio devices
aplay -l
```

### Video Not Playing

```bash
# Check media files are mounted
docker run -it --rm \
  -v $(pwd)/media:/media:ro \
  vantismedia/vantis-player:latest \
  ls -la /media

# Check file permissions
ls -la media/
```

### Performance Issues

```bash
# Enable hardware acceleration
docker run -d \
  --device /dev/dri:/dev/dri \
  vantismedia/vantis-player:latest

# Increase resource limits
docker run -d \
  --cpus="4" \
  --memory="4g" \
  vantismedia/vantis-player:latest
```

### Permission Issues

```bash
# Fix volume permissions
docker run -it --rm \
  -v vantis-config:/config \
  vantismedia/vantis-player:latest \
  chown -R vantis:vantis /config
```

## Advanced Usage

### Custom Entrypoint

```bash
docker run -d \
  --entrypoint="/app/vantis" \
  vantismedia/vantis-player:latest \
  play /media/video.mp4
```

### Multiple Instances

```bash
# Instance 1
docker run -d \
  --name vantis-player-1 \
  -p 8080:8080 \
  vantismedia/vantis-player:latest

# Instance 2
docker run -d \
  --name vantis-player-2 \
  -p 8081:8080 \
  vantismedia/vantis-player:latest
```

### Health Checks

```bash
# Check container health
docker inspect --format='{{.State.Health.Status}}' vantis-player

# View health check logs
docker inspect --format='{{range .State.Health.Log}}{{.Output}}{{end}}' vantis-player
```

## Security

### Run as Non-Root User

The Docker image runs as a non-root user (`vantis`) by default for security.

### Read-Only Filesystem

```bash
docker run -d \
  --read-only \
  --tmpfs /tmp \
  vantismedia/vantis-player:latest
```

### Resource Limits

```bash
docker run -d \
  --cpus="2" \
  --memory="2g" \
  --memory-swap="2g" \
  vantismedia/vantis-player:latest
```

## Support

For issues or questions:

- Check the [Troubleshooting](docs/troubleshooting.html) page
- Search [GitHub Issues](https://github.com/vantisCorp/VantisMedia/issues)
- Create a new issue with detailed information

## License

MIT License - See [LICENSE](LICENSE) for details.