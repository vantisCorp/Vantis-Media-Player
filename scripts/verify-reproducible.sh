#!/bin/bash
# =============================================================================
# Reproducible Build Verification Script
# =============================================================================
# This script verifies that builds are reproducible by comparing binary hashes
# across multiple builds. It implements the reproducible builds specification.

set -euo pipefail

# Configuration
PROJECT_NAME="vantis-player"
BUILD_DIR="target/reproducible-test"
LOG_FILE="reproducible-build.log"
NUM_BUILDS=3

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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
# BUILD FUNCTIONS
# =============================================================================

clean_build() {
    log_info "Cleaning previous build artifacts..."
    cargo clean
    rm -rf "$BUILD_DIR"
    mkdir -p "$BUILD_DIR"
}

build_release() {
    local build_num=$1
    log_info "Building release #$build_num..."
    
    # Set reproducible build environment
    export SOURCE_DATE_EPOCH=0
    export RUSTFLAGS="--remap-path-prefix=${PWD}=/build --remap-path-prefix=${CARGO_HOME}=/cargo"
    
    # Build with deterministic settings
    cargo build --release --locked 2>&1 | tee -a "$LOG_FILE"
    
    # Copy binary to unique location
    cp "target/release/${PROJECT_NAME}" "${BUILD_DIR}/${PROJECT_NAME}-${build_num}"
    
    # Generate hash
    sha256sum "${BUILD_DIR}/${PROJECT_NAME}-${build_num}" > "${BUILD_DIR}/${PROJECT_NAME}-${build_num}.sha256"
    
    # Generate file info
    file "${BUILD_DIR}/${PROJECT_NAME}-${build_num}" > "${BUILD_DIR}/${PROJECT_NAME}-${build_num}.info"
    
    # Strip and save stripped version
    strip "${BUILD_DIR}/${PROJECT_NAME}-${build_num}" -o "${BUILD_DIR}/${PROJECT_NAME}-${build_num}-stripped"
    sha256sum "${BUILD_DIR}/${PROJECT_NAME}-${build_num}-stripped" > "${BUILD_DIR}/${PROJECT_NAME}-${build_num}-stripped.sha256"
}

# =============================================================================
# VERIFICATION FUNCTIONS
# =============================================================================

verify_hashes() {
    log_info "Verifying build reproducibility..."
    
    local hashes=()
    for i in $(seq 1 $NUM_BUILDS); do
        hash=$(cat "${BUILD_DIR}/${PROJECT_NAME}-${i}.sha256" | cut -d' ' -f1)
        hashes+=("$hash")
        log_info "Build #$i hash: ${hash:0:16}..."
    done
    
    # Compare all hashes
    local first_hash="${hashes[0]}"
    local all_match=true
    
    for i in "${!hashes[@]}"; do
        if [[ "${hashes[$i]}" != "$first_hash" ]]; then
            log_error "Build #$((i+1)) hash mismatch!"
            all_match=false
        fi
    done
    
    if $all_match; then
        log_success "All builds are reproducible! ✓"
        log_info "Deterministic hash: ${first_hash}"
        return 0
    else
        log_error "Builds are NOT reproducible!"
        log_info "Attempting to identify differences..."
        analyze_differences
        return 1
    fi
}

analyze_differences() {
    log_info "Analyzing binary differences..."
    
    # Use objdump to compare sections
    for i in $(seq 2 $NUM_BUILDS); do
        log_info "Comparing build #1 vs build #$i..."
        
        # Compare sizes
        size1=$(stat -c%s "${BUILD_DIR}/${PROJECT_NAME}-1")
        size2=$(stat -c%s "${BUILD_DIR}/${PROJECT_NAME}-${i}")
        
        if [[ "$size1" != "$size2" ]]; then
            log_warning "Size difference: $size1 vs $size2 bytes"
        fi
        
        # Compare stripped versions
        hash1=$(cat "${BUILD_DIR}/${PROJECT_NAME}-1-stripped.sha256" | cut -d' ' -f1)
        hash2=$(cat "${BUILD_DIR}/${PROJECT_NAME}-${i}-stripped.sha256" | cut -d' ' -f1)
        
        if [[ "$hash1" == "$hash2" ]]; then
            log_info "Stripped binaries match - difference is in debug info"
        else
            log_warning "Stripped binaries also differ"
            
            # Try to find where they differ
            if command -v cmp &> /dev/null; then
                cmp -l "${BUILD_DIR}/${PROJECT_NAME}-1-stripped" "${BUILD_DIR}/${PROJECT_NAME}-${i}-stripped" 2>&1 | head -20 | tee -a "$LOG_FILE"
            fi
        fi
    done
}

generate_report() {
    local report_file="${BUILD_DIR}/reproducibility-report.txt"
    
    log_info "Generating reproducibility report..."
    
    cat > "$report_file" << EOF
================================================================================
Vantis Media Player - Reproducible Build Report
================================================================================
Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Builds: $NUM_BUILDS

Build Environment:
  - Rust: $(rustc --version)
  - OS: $(uname -a)
  - SOURCE_DATE_EPOCH: ${SOURCE_DATE_EPOCH:-not set}

Binary Information:
EOF

    for i in $(seq 1 $NUM_BUILDS); do
        cat "${BUILD_DIR}/${PROJECT_NAME}-${i}.info" >> "$report_file"
        echo "  - SHA256: $(cat "${BUILD_DIR}/${PROJECT_NAME}-${i}.sha256")" >> "$report_file"
        echo "" >> "$report_file"
    done

    log_success "Report saved to: $report_file"
}

# =============================================================================
# MAIN
# =============================================================================

main() {
    log_info "Starting reproducible build verification..."
    log_info "Number of builds: $NUM_BUILDS"
    
    # Clean up
    rm -f "$LOG_FILE"
    
    # Perform multiple builds
    for i in $(seq 1 $NUM_BUILDS); do
        clean_build
        build_release "$i"
    done
    
    # Verify reproducibility
    verify_hashes
    local result=$?
    
    # Generate report
    generate_report
    
    if [[ $result -eq 0 ]]; then
        log_success "Reproducible build verification PASSED"
        exit 0
    else
        log_error "Reproducible build verification FAILED"
        exit 1
    fi
}

# Run main function
main "$@"