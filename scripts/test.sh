#!/bin/bash
# Vantis Media Player - Test Runner Script
# Runs all tests with coverage

set -e

echo "🧪 Running Vantis Media Player Test Suite..."
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

# Parse arguments
RUN_COVERAGE=false
RUN_BENCHMARKS=false
SKIP_SLOW=false

for arg in "$@"; do
    case $arg in
        --coverage)
            RUN_COVERAGE=true
            shift
            ;;
        --benchmarks)
            RUN_BENCHMARKS=true
            shift
            ;;
        --skip-slow)
            SKIP_SLOW=true
            shift
            ;;
        *)
            ;;
    esac
done

# 1. Unit Tests
echo -e "${BLUE}📋 Running unit tests...${NC}"
if [ "$SKIP_SLOW" = true ]; then
    cargo test --workspace --lib --bins --exclude slow-tests
else
    cargo test --workspace --lib --bins
fi
echo -e "${GREEN}✅ Unit tests passed${NC}"
echo ""

# 2. Integration Tests
echo -e "${BLUE}🔗 Running integration tests...${NC}"
if [ -d "tests" ]; then
    cargo test --workspace --test '*'
    echo -e "${GREEN}✅ Integration tests passed${NC}"
else
    echo -e "${YELLOW}⚠️  No integration tests found${NC}"
fi
echo ""

# 3. Doc Tests
echo -e "${BLUE}📚 Running documentation tests...${NC}"
cargo test --workspace --doc
echo -e "${GREEN}✅ Doc tests passed${NC}"
echo ""

# 4. Coverage (optional)
if [ "$RUN_COVERAGE" = true ]; then
    echo -e "${BLUE}📊 Running tests with coverage...${NC}"
    if command_exists cargo-tarpaulin; then
        cargo tarpaulin --workspace --out Html --output-dir ./coverage
        echo -e "${GREEN}✅ Coverage report generated in ./coverage/index.html${NC}"
    elif command_exists grcov; then
        cargo clean
        export CARGO_INCREMENTAL=0
        export RUSTFLAGS="-Cinstrument-coverage"
        export LLVM_PROFILE_FILE="cargo-test-%p-%m.profraw"
        cargo test --workspace
        grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing -o ./coverage/
        echo -e "${GREEN}✅ Coverage report generated in ./coverage/index.html${NC}"
    else
        echo -e "${YELLOW}⚠️  Coverage tool not found. Install cargo-tarpaulin or grcov${NC}"
    fi
    echo ""
fi

# 5. Benchmarks (optional)
if [ "$RUN_BENCHMARKS" = true ]; then
    echo -e "${BLUE}⚡ Running benchmarks...${NC}"
    if [ -d "benches" ]; then
        cargo bench --workspace
        echo -e "${GREEN}✅ Benchmarks completed${NC}"
    else
        echo -e "${YELLOW}⚠️  No benchmarks found${NC}"
    fi
    echo ""
fi

echo -e "${GREEN}✨ All tests passed successfully!${NC}"