# Docker Quick Start

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/vantisCorp/VantisMedia.git
cd VantisMedia

# Create media directory
mkdir -p media

# Copy your media files
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

### Using Makefile

```bash
# Include Docker Makefile
include Makefile.docker

# Build image
make build

# Run container
make run

# Start with docker-compose
make compose-up

# View logs
make compose-logs

# Stop services
make compose-down
```

## Features

- ✅ Multi-stage build for smaller image size
- ✅ Non-root user for security
- ✅ Hardware acceleration support (GPU)
- ✅ Persistent volumes for config, cache, and plugins
- ✅ Docker Compose configuration
- ✅ Multi-platform support (amd64, arm64)
- ✅ Automated builds with GitHub Actions

## Documentation

For detailed Docker deployment instructions, see [DOCKER.md](DOCKER.md).

## Support

- [Docker Documentation](DOCKER.md)
- [Main README](README.md)
- [GitHub Issues](https://github.com/vantisCorp/VantisMedia/issues)