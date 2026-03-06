#!/bin/bash
# Universal Build System for Vantis Media Player
# Inspired by VideoLAN's multi-platform build system
# Supports: Linux, macOS, Windows (via cross-compilation), Android, iOS, WebAssembly

set -euo pipefail

# =============================================================================
# CONFIGURATION
# =============================================================================

VERSION="2.0.0"
PROJECT_NAME="vantis-player"
BUILD_DIR="target/build"
DIST_DIR="target/dist"
LOG_FILE="build.log"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# =============================================================================
# UTILITY FUNCTIONS
# =============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" | tee -a "$LOG_FILE"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" | tee -a "$LOG_FILE"
}

# =============================================================================
# PLATFORM DETECTION
# =============================================================================

detect_platform() {
    local os="$(uname -s)"
    local arch="$(uname -m)"
    
    case "$os" in
        Linux*)
            PLATFORM="linux"
            ;;
        Darwin*)
            PLATFORM="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            PLATFORM="windows"
            ;;
        *)
            log_error "Unsupported platform: $os"
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        armv7l)
            ARCH="armv7"
            ;;
        *)
            log_error "Unsupported architecture: $arch"
            exit 1
            ;;
    esac
    
    log_info "Detected platform: $PLATFORM-$ARCH"
}

# =============================================================================
# BUILD FUNCTIONS
# =============================================================================

build_native() {
    log_info "Building for native platform ($PLATFORM-$ARCH)..."
    
    # Clean previous builds
    rm -rf "$BUILD_DIR" "$DIST_DIR"
    mkdir -p "$BUILD_DIR" "$DIST_DIR"
    
    # Build with cargo
    cargo build --release --target-dir="$BUILD_DIR" 2>&1 | tee -a "$LOG_FILE"
    
    # Copy artifacts
    if [ "$PLATFORM" = "windows" ]; then
        cp "$BUILD_DIR/release/$PROJECT_NAME.exe" "$DIST_DIR/"
    else
        cp "$BUILD_DIR/release/$PROJECT_NAME" "$DIST_DIR/"
    fi
    
    log_success "Native build completed!"
}

build_cross_linux() {
    local target="$1"
    log_info "Cross-compiling for Linux ($target)..."
    
    # Install cross-compilation toolchain if needed
    rustup target add "$target" 2>&1 | tee -a "$LOG_FILE"
    
    # Install cross if not available
    if ! command -v cross &> /dev/null; then
        log_info "Installing cross for cross-compilation..."
        cargo install cross 2>&1 | tee -a "$LOG_FILE"
    fi
    
    # Build with cross
    cross build --release --target="$target" --target-dir="$BUILD_DIR" 2>&1 | tee -a "$LOG_FILE"
    
    # Copy artifacts
    mkdir -p "$DIST_DIR/$target"
    cp "$BUILD_DIR/$target/release/$PROJECT_NAME" "$DIST_DIR/$target/"
    
    log_success "Linux cross-build for $target completed!"
}

build_cross_android() {
    local arch="$1" # arm64-v8a, armeabi-v7a, x86_64, x86
    log_info "Cross-compiling for Android ($arch)..."
    
    # Setup Android NDK paths
    if [ -z "${ANDROID_NDK_HOME:-}" ]; then
        log_error "ANDROID_NDK_HOME environment variable not set"
        exit 1
    fi
    
    # Map Android architecture to Rust target
    case "$arch" in
        arm64-v8a)
            target="aarch64-linux-android"
            ;;
        armeabi-v7a)
            target="armv7-linux-androideabi"
            ;;
        x86_64)
            target="x86_64-linux-android"
            ;;
        x86)
            target="i686-linux-android"
            ;;
        *)
            log_error "Unsupported Android architecture: $arch"
            exit 1
            ;;
    esac
    
    # Install target
    rustup target add "$target" 2>&1 | tee -a "$LOG_FILE"
    
    # Setup cross-compilation environment
    export CARGO_TARGET_${target//-/_}_LINKER="${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/${target}-21-clang"
    
    # Build
    cargo build --release --target="$target" --target-dir="$BUILD_DIR" 2>&1 | tee -a "$LOG_FILE"
    
    # Copy artifacts
    mkdir -p "$DIST_DIR/android-$arch"
    cp "$BUILD_DIR/$target/release/$PROJECT_NAME" "$DIST_DIR/android-$arch/"
    
    log_success "Android cross-build for $arch completed!"
}

build_cross_ios() {
    local target="$1"
    log_info "Cross-compiling for iOS ($target)..."
    
    # Install target
    rustup target add "$target" 2>&1 | tee -a "$LOG_FILE"
    
    # Setup cross-compilation environment for iOS
    case "$target" in
        aarch64-apple-ios)
            export CC="clang"
            export CXX="clang++"
            export CFLAGS="-arch arm64 -isysroot $(xcrun --sdk iphoneos --show-sdk-path)"
            export CXXFLAGS="-arch arm64 -isysroot $(xcrun --sdk iphoneos --show-sdk-path)"
            ;;
        aarch64-apple-ios-sim)
            export CC="clang"
            export CXX="clang++"
            export CFLAGS="-arch arm64 -isysroot $(xcrun --sdk iphonesimulator --show-sdk-path)"
            export CXXFLAGS="-arch arm64 -isysroot $(xcrun --sdk iphonesimulator --show-sdk-path)"
            ;;
    esac
    
    # Build
    cargo build --release --target="$target" --target-dir="$BUILD_DIR" 2>&1 | tee -a "$LOG_FILE"
    
    # Copy artifacts
    mkdir -p "$DIST_DIR/$target"
    cp "$BUILD_DIR/$target/release/lib${PROJECT_NAME}.a" "$DIST_DIR/$target/"
    
    log_success "iOS cross-build for $target completed!"
}

build_wasm() {
    log_info "Building for WebAssembly..."
    
    # Install wasm32 target
    rustup target add wasm32-wasi 2>&1 | tee -a "$LOG_FILE"
    
    # Build wasm
    cargo build --release --target=wasm32-wasi --target-dir="$BUILD_DIR" 2>&1 | tee -a "$LOG_FILE"
    
    # Copy artifacts
    mkdir -p "$DIST_DIR/wasm"
    cp "$BUILD_DIR/wasm32-wasi/release/$PROJECT_NAME.wasm" "$DIST_DIR/wasm/"
    
    # Optimize wasm with wasm-opt if available
    if command -v wasm-opt &> /dev/null; then
        log_info "Optimizing WebAssembly binary..."
        wasm-opt -O3 "$DIST_DIR/wasm/$PROJECT_NAME.wasm" -o "$DIST_DIR/wasm/$PROJECT_NAME.opt.wasm" 2>&1 | tee -a "$LOG_FILE"
    fi
    
    log_success "WebAssembly build completed!"
}

# =============================================================================
# RELEASE FUNCTIONS
# =============================================================================

create_package() {
    local platform="$1"
    local arch="${2:-}"
    
    log_info "Creating release package for $platform-$arch..."
    
    case "$platform" in
        linux)
            # Create tar.gz
            cd "$DIST_DIR"
            tar -czf "${PROJECT_NAME}-${VERSION}-${platform}-${arch}.tar.gz" "$PROJECT_NAME"
            cd - > /dev/null
            log_success "Created ${PROJECT_NAME}-${VERSION}-${platform}-${arch}.tar.gz"
            ;;
        macos)
            # Create DMG
            log_info "Creating DMG package..."
            # DMG creation would go here with hdiutil
            log_warning "DMG creation not implemented yet"
            ;;
        windows)
            # Create ZIP
            cd "$DIST_DIR"
            zip -r "${PROJECT_NAME}-${VERSION}-${platform}-${arch}.zip" "$PROJECT_NAME.exe"
            cd - > /dev/null
            log_success "Created ${PROJECT_NAME}-${VERSION}-${platform}-${arch}.zip"
            ;;
        android|ios|wasm)
            log_info "Package already in $DIST_DIR directory"
            ;;
    esac
}

# =============================================================================
# CLEANUP
# =============================================================================

clean() {
    log_info "Cleaning build artifacts..."
    rm -rf "$BUILD_DIR" "$DIST_DIR"
    cargo clean
    log_success "Cleanup completed!"
}

# =============================================================================
# HELPERS
# =============================================================================

show_help() {
    cat << EOF
Universal Build System for Vantis Media Player v${VERSION}

USAGE:
    $0 [OPTIONS] [COMMAND]

COMMANDS:
    native          Build for native platform
    linux-all       Build for all Linux architectures (x86_64, aarch64, armv7)
    android-all     Build for all Android architectures
    ios-all         Build for all iOS architectures (arm64, simulator)
    wasm            Build for WebAssembly
    clean           Clean build artifacts
    help            Show this help message

OPTIONS:
    -v, --verbose   Enable verbose output
    -d, --debug     Build in debug mode (default: release)
    --target TRIPLE  Build for specific Rust target triple

EXAMPLES:
    $0 native                    # Build for current platform
    $0 linux-all                 # Build for all Linux architectures
    $0 android-all               # Build for all Android architectures
    $0 wasm                      # Build for WebAssembly
    $0 --target aarch64-linux-gnu # Build for specific target

For more information, see BUILD_SYSTEM.md
EOF
}

# =============================================================================
# MAIN
# =============================================================================

main() {
    local command="${1:-help}"
    local mode="release"
    
    # Parse options
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--verbose)
                set -x
                shift
                ;;
            -d|--debug)
                mode="debug"
                shift
                ;;
            *)
                command="$1"
                shift
                ;;
        esac
    done
    
    # Detect platform
    detect_platform
    
    # Execute command
    case "$command" in
        native)
            build_native
            create_package "$PLATFORM" "$ARCH"
            ;;
        linux-all)
            build_cross_linux "x86_64-unknown-linux-gnu"
            build_cross_linux "aarch64-unknown-linux-gnu"
            build_cross_linux "armv7-unknown-linux-gnueabihf"
            ;;
        android-all)
            build_cross_android "arm64-v8a"
            build_cross_android "armeabi-v7a"
            build_cross_android "x86_64"
            build_cross_android "x86"
            ;;
        ios-all)
            if [ "$PLATFORM" = "macos" ]; then
                build_cross_ios "aarch64-apple-ios"
                build_cross_ios "aarch64-apple-ios-sim"
            else
                log_error "iOS builds require macOS"
                exit 1
            fi
            ;;
        wasm)
            build_wasm
            ;;
        clean)
            clean
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

main "$@"