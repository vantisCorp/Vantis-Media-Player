#!/bin/bash
# Vantis Media Player - Benchmark Runner
# Runs performance benchmarks for the application

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "► ════ ◄ Vantis Benchmark Suite ► ════ ◄"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Parse arguments
BENCHMARK_TYPE="${1:-all}"
ITERATIONS="${2:-100}"

run_cargo_benchmarks() {
    echo -e "${YELLOW}Running Cargo benchmarks...${NC}"
    cd "$PROJECT_ROOT"
    
    if command -v cargo-criterion &> /dev/null; then
        cargo criterion --bench all 2>/dev/null || cargo bench 2>/dev/null || echo "No benchmarks found"
    else
        cargo bench 2>/dev/null || echo "No benchmarks found"
    fi
}

run_startup_benchmark() {
    echo -e "${YELLOW}Running startup benchmark...${NC}"
    
    local total_time=0
    local count=0
    
    for i in $(seq 1 $ITERATIONS); do
        start=$(date +%s%N)
        cargo build --release --quiet 2>/dev/null || true
        end=$(date +%s%N)
        elapsed=$(( (end - start) / 1000000 ))
        total_time=$((total_time + elapsed))
        count=$((count + 1))
    done
    
    local avg=$((total_time / count))
    echo -e "  Average build time: ${GREEN}${avg}ms${NC}"
}

run_memory_profile() {
    echo -e "${YELLOW}Running memory profile...${NC}"
    
    if command -v valgrind &> /dev/null; then
        cargo build --release 2>/dev/null
        valgrind --tool=massif --massif-out-file=massif.out ./target/release/vantis-player 2>/dev/null || true
        
        if command -v ms_print &> /dev/null; then
            ms_print massif.out 2>/dev/null || echo "  Massif results not available"
        fi
    else
        echo -e "  ${YELLOW}valgrind not installed, skipping memory profile${NC}"
    fi
}

run_code_analysis() {
    echo -e "${YELLOW}Running code analysis...${NC}"
    
    # Count lines of code
    local loc=$(find "$PROJECT_ROOT" -name "*.rs" -exec wc -l {} + | tail -1 | awk '{print $1}')
    echo -e "  Lines of Rust code: ${GREEN}${loc}${NC}"
    
    # Count test coverage
    local test_count=$(grep -r "#\[test\]" "$PROJECT_ROOT" --include="*.rs" | wc -l)
    echo -e "  Test functions: ${GREEN}${test_count}${NC}"
    
    # Check for clippy warnings
    if cargo clippy --quiet 2>&1 | grep -q "warning"; then
        echo -e "  Clippy warnings: ${YELLOW}Found${NC}"
    else
        echo -e "  Clippy warnings: ${GREEN}None${NC}"
    fi
}

print_summary() {
    echo ""
    echo "► ════ ◄ Benchmark Summary ► ════ ◄"
    echo "  Iterations: $ITERATIONS"
    echo "  Type: $BENCHMARK_TYPE"
    echo ""
}

# Main execution
case "$BENCHMARK_TYPE" in
    "cargo"|"bench")
        run_cargo_benchmarks
        ;;
    "startup"|"build")
        run_startup_benchmark
        ;;
    "memory"|"mem")
        run_memory_profile
        ;;
    "analysis"|"code")
        run_code_analysis
        ;;
    "all"|*)
        run_cargo_benchmarks
        echo ""
        run_startup_benchmark
        echo ""
        run_code_analysis
        ;;
esac

print_summary

echo -e "${GREEN}✓ Benchmarks complete${NC}"