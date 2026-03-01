#!/bin/bash

# Build Installers Script for Vantis Media Player
# This script builds installers for Windows, macOS, and Linux

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Variables
VERSION=${VERSION:-"1.0.0"}
OUTPUT_DIR="installers"
BUILD_DIR="target/release"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Vantis Media Player - Build Installers${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Function to build for Linux
build_linux() {
    echo -e "${YELLOW}Building for Linux...${NC}"
    
    # Build release
    cargo build --release --target x86_64-unknown-linux-gnu
    
    # Create installer directory
    mkdir -p "$OUTPUT_DIR/vantis-player-linux-x64"
    
    # Copy files
    cp "$BUILD_DIR/vantis" "$OUTPUT_DIR/vantis-player-linux-x64/"
    cp README.md "$OUTPUT_DIR/vantis-player-linux-x64/"
    cp LICENSE "$OUTPUT_DIR/vantis-player-linux-x64/"
    cp CHANGELOG.md "$OUTPUT_DIR/vantis-player-linux-x64/"
    
    # Create tarball
    cd "$OUTPUT_DIR"
    tar -czf "vantis-player-linux-x64-$VERSION.tar.gz" vantis-player-linux-x64
    cd ..
    
    echo -e "${GREEN}✓ Linux installer built: $OUTPUT_DIR/vantis-player-linux-x64-$VERSION.tar.gz${NC}"
}

# Function to build for macOS
build_macos() {
    echo -e "${YELLOW}Building for macOS...${NC}"
    
    # Build for x86_64
    cargo build --release --target x86_64-apple-darwin
    
    # Build for aarch64 (if on Apple Silicon)
    if [[ $(uname -m) == 'arm64' ]]; then
        cargo build --release --target aarch64-apple-darwin
        
        # Create universal binary
        lipo -create \
            target/x86_64-apple-darwin/release/vantis \
            target/aarch64-apple-darwin/release/vantis \
            -output target/vantis-universal
    else
        cp target/x86_64-apple-darwin/release/vantis target/vantis-universal
    fi
    
    # Create app bundle
    mkdir -p "$OUTPUT_DIR/vantis-player.app/Contents/MacOS"
    mkdir -p "$OUTPUT_DIR/vantis-player.app/Contents/Resources"
    
    cp target/vantis-universal "$OUTPUT_DIR/vantis-player.app/Contents/MacOS/vantis"
    chmod +x "$OUTPUT_DIR/vantis-player.app/Contents/MacOS/vantis"
    
    cp README.md "$OUTPUT_DIR/vantis-player.app/Contents/Resources/"
    cp LICENSE "$OUTPUT_DIR/vantis-player.app/Contents/Resources/"
    cp CHANGELOG.md "$OUTPUT_DIR/vantis-player.app/Contents/Resources/"
    
    # Create Info.plist
    cat > "$OUTPUT_DIR/vantis-player.app/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>vantis</string>
    <key>CFBundleIdentifier</key>
    <string>com.vantismedia.player</string>
    <key>CFBundleName</key>
    <string>Vantis Media Player</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF
    
    # Create DMG
    hdiutil create -volname "Vantis Media Player" -srcfolder "$OUTPUT_DIR/vantis-player.app" -ov -format UDZO "$OUTPUT_DIR/vantis-player-macos-universal-$VERSION.dmg"
    
    echo -e "${GREEN}✓ macOS installer built: $OUTPUT_DIR/vantis-player-macos-universal-$VERSION.dmg${NC}"
}

# Function to build for Windows (cross-compilation)
build_windows() {
    echo -e "${YELLOW}Building for Windows (cross-compilation)...${NC}"
    
    # Check if cross-compilation tools are installed
    if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
        echo -e "${RED}✗ Windows cross-compilation tools not found${NC}"
        echo -e "${YELLOW}Install with: sudo apt-get install mingw-w64${NC}"
        return 1
    fi
    
    # Add Windows target
    rustup target add x86_64-pc-windows-gnu
    
    # Build release
    cargo build --release --target x86_64-pc-windows-gnu
    
    # Create installer directory
    mkdir -p "$OUTPUT_DIR/vantis-player-windows-x64"
    
    # Copy files
    cp "target/x86_64-pc-windows-gnu/release/vantis.exe" "$OUTPUT_DIR/vantis-player-windows-x64/"
    cp README.md "$OUTPUT_DIR/vantis-player-windows-x64/"
    cp LICENSE "$OUTPUT_DIR/vantis-player-windows-x64/"
    cp CHANGELOG.md "$OUTPUT_DIR/vantis-player-windows-x64/"
    
    # Create ZIP
    cd "$OUTPUT_DIR"
    zip -r "vantis-player-windows-x64-$VERSION.zip" vantis-player-windows-x64
    cd ..
    
    echo -e "${GREEN}✓ Windows installer built: $OUTPUT_DIR/vantis-player-windows-x64-$VERSION.zip${NC}"
}

# Main build logic
case "${1:-all}" in
    linux)
        build_linux
        ;;
    macos)
        build_macos
        ;;
    windows)
        build_windows
        ;;
    all)
        # Detect OS
        OS=$(uname -s)
        case "$OS" in
            Linux*)
                build_linux
                build_windows
                ;;
            Darwin*)
                build_macos
                ;;
            MINGW*|MSYS*|CYGWIN*)
                echo -e "${YELLOW}Building for Windows...${NC}"
                cargo build --release
                mkdir -p "$OUTPUT_DIR/vantis-player-windows-x64"
                cp "$BUILD_DIR/vantis.exe" "$OUTPUT_DIR/vantis-player-windows-x64/"
                cp README.md "$OUTPUT_DIR/vantis-player-windows-x64/"
                cp LICENSE "$OUTPUT_DIR/vantis-player-windows-x64/"
                cp CHANGELOG.md "$OUTPUT_DIR/vantis-player-windows-x64/"
                cd "$OUTPUT_DIR"
                zip -r "vantis-player-windows-x64-$VERSION.zip" vantis-player-windows-x64
                cd ..
                echo -e "${GREEN}✓ Windows installer built: $OUTPUT_DIR/vantis-player-windows-x64-$VERSION.zip${NC}"
                ;;
            *)
                echo -e "${RED}Unknown OS: $OS${NC}"
                exit 1
                ;;
        esac
        ;;
    *)
        echo -e "${RED}Usage: $0 [linux|macos|windows|all]${NC}"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "${BLUE}Installers are in: $OUTPUT_DIR${NC}"