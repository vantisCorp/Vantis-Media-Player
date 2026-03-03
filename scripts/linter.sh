#!/bin/bash
# Vantis Media Player - Code Linter Script
# Runs all linters and formatters

set -e

echo "🔍 Running Vantis Media Player Linters..."
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check Rust installation
if ! command_exists rustc; then
    echo -e "${RED}❌ Rust not found. Please install Rust first.${NC}"
    exit 1
fi

echo "🦀 Rust version: $(rustc --version)"
echo ""

# 1. Rust Format Check
echo "📝 Checking code formatting with rustfmt..."
if command_exists rustfmt; then
    if cargo fmt --all -- --check; then
        echo -e "${GREEN}✅ Code is properly formatted${NC}"
    else
        echo -e "${RED}❌ Code formatting issues found. Run 'cargo fmt --all' to fix.${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  rustfmt not installed. Run 'rustup component add rustfmt'${NC}"
fi
echo ""

# 2. Clippy Linting
echo "🔧 Running Clippy linter..."
if command_exists cargo-clippy; then
    if cargo clippy --all-targets --all-features -- -D warnings; then
        echo -e "${GREEN}✅ No Clippy warnings${NC}"
    else
        echo -e "${RED}❌ Clippy found issues${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️  Clippy not installed. Run 'rustup component add clippy'${NC}"
fi
echo ""

# 3. Check for unused dependencies
echo "📦 Checking for unused dependencies..."
if command_exists cargo-udeps; then
    cargo +nightly udeps
else
    echo -e "${YELLOW}⚠️  cargo-udeps not installed. Install with 'cargo install cargo-udeps --locked'${NC}"
fi
echo ""

# 4. Check for security vulnerabilities
echo "🔒 Checking for security vulnerabilities..."
if command_exists cargo-audit; then
    cargo audit
else
    echo -e "${YELLOW}⚠️  cargo-audit not installed. Install with 'cargo install cargo-audit'${NC}"
fi
echo ""

# 5. Check for outdated dependencies
echo "📅 Checking for outdated dependencies..."
if command_exists cargo-outdated; then
    cargo outdated
else
    echo -e "${YELLOW}⚠️  cargo-outdated not installed. Install with 'cargo install cargo-outdated'${NC}"
fi
echo ""

echo -e "${GREEN}✨ All linting checks passed!${NC}"