#!/bin/bash
# Release automation script for Vantis Media Player

set -e

VERSION=${1:-$(grep '^version' Cargo.toml | head -1 | awk -F'"' '{print $2}')}

echo "🚀 Releasing Vantis Media Player v${VERSION}"

# Check if version is provided
if [ -z "$VERSION" ]; then
    echo "❌ Error: Version not found"
    exit 1
fi

echo "📦 Version: ${VERSION}"

# Run tests
echo "🧪 Running tests..."
cargo test --all-features

# Run clippy
echo "🔍 Running clippy..."
cargo clippy --all-features -- -D warnings

# Build release
echo "🔨 Building release..."
cargo build --release

# Strip binary
echo "✂️  Stripping binary..."
strip target/release/vantis

# Create release directory
RELEASE_DIR="release/v${VERSION}"
mkdir -p "${RELEASE_DIR}"

# Copy files
echo "📋 Copying files..."
cp target/release/vantis "${RELEASE_DIR}/"
cp README.md "${RELEASE_DIR}/"
cp CHANGELOG.md "${RELEASE_DIR}/"

# Create archives
echo "📦 Creating archives..."

# Linux x86_64
tar -czf "vantis-${VERSION}-linux-x86_64.tar.gz" -C release "v${VERSION}"

# Calculate checksums
echo "🔐 Calculating checksums..."
sha256sum "vantis-${VERSION}-linux-x86_64.tar.gz" > "vantis-${VERSION}-checksums.txt"

echo "✅ Release prepared successfully!"
echo ""
echo "Files created:"
echo "  - vantis-${VERSION}-linux-x86_64.tar.gz"
echo "  - vantis-${VERSION}-checksums.txt"