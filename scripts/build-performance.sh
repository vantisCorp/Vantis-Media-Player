#!/bin/bash
# =============================================================================
# Build Performance Monitoring Script
# =============================================================================
# Tracks build times, resource usage, and performance metrics for regression
# detection and CI optimization.

set -euo pipefail

# Configuration
PROJECT_NAME="vantis-player"
PERF_DIR="target/performance"
HISTORY_FILE="${PERF_DIR}/build-history.json"
METRICS_FILE="${PERF_DIR}/metrics.json"
THRESHOLD_WARNING=10  # Percentage increase threshold for warning
THRESHOLD_FAILURE=25  # Percentage increase threshold for failure

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# =============================================================================
# INITIALIZATION
# =============================================================================

init_performance_dir() {
    mkdir -p "$PERF_DIR"
    
    if [[ ! -f "$HISTORY_FILE" ]]; then
        echo '{"builds": []}' > "$HISTORY_FILE"
    fi
}

# =============================================================================
# PERFORMANCE MEASUREMENT
# =============================================================================

measure_build() {
    local build_type="${1:-release}"
    local start_time end_time duration
    
    log_info "Measuring ${build_type} build performance..."
    
    # Clean build
    cargo clean 2>/dev/null || true
    
    # Measure build time and resources
    local start_mem=$(free -m | awk '/Mem:/ {print $3}')
    local start_time=$(date +%s.%N)
    
    # Run build with timing
    if [[ "$build_type" == "release" ]]; then
        /usr/bin/time -v cargo build --release 2>&1 | tee "${PERF_DIR}/build-output.txt" || true
    else
        /usr/bin/time -v cargo build 2>&1 | tee "${PERF_DIR}/build-output.txt" || true
    fi
    
    local end_time=$(date +%s.%N)
    local end_mem=$(free -m | awk '/Mem:/ {print $3}')
    
    # Calculate duration
    duration=$(echo "$end_time - $start_time" | bc)
    
    # Extract metrics from time output
    local max_rss=$(grep "Maximum resident set size" "${PERF_DIR}/build-output.txt" | awk '{print $NF}' || echo "0")
    local cpu_percent=$(grep "Percent of CPU this job got" "${PERF_DIR}/build-output.txt" | awk '{print $NF}' || echo "0")
    local major_pagefaults=$(grep "Major page faults" "${PERF_DIR}/build-output.txt" | awk '{print $NF}' || echo "0")
    
    # Output metrics
    echo "BUILD_TIME=${duration}" > "${PERF_DIR}/current-metrics.env"
    echo "MAX_RSS=${max_rss}" >> "${PERF_DIR}/current-metrics.env"
    echo "CPU_PERCENT=${cpu_percent}" >> "${PERF_DIR}/current-metrics.env"
    echo "MAJOR_PAGEFAULTS=${major_pagefaults}" >> "${PERF_DIR}/current-metrics.env"
    
    log_info "Build time: ${duration}s"
    log_info "Max RSS: ${max_rss} KB"
    log_info "CPU utilization: ${cpu_percent}%"
    
    # Return values
    BUILD_TIME=$duration
    MAX_RSS=$max_rss
    CPU_PERCENT=$cpu_percent
    MAJOR_PAGEFAULTS=$major_pagefaults
}

# =============================================================================
# HISTORY MANAGEMENT
# =============================================================================

save_to_history() {
    local build_time=$1
    local max_rss=$2
    local cpu_percent=$3
    local major_pagefaults=$4
    
    local timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
    local commit=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
    local branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")
    
    local entry=$(cat <<EOF
{
    "timestamp": "${timestamp}",
    "commit": "${commit}",
    "branch": "${branch}",
    "build_time": ${build_time},
    "max_rss": ${max_rss},
    "cpu_percent": ${cpu_percent},
    "major_pagefaults": ${major_pagefaults}
}
EOF
)
    
    # Add to history (using jq if available)
    if command -v jq &> /dev/null; then
        local tmp=$(mktemp)
        jq ".builds += [${entry}]" "$HISTORY_FILE" > "$tmp" && mv "$tmp" "$HISTORY_FILE"
    else
        # Fallback: append to file manually
        echo "$entry" >> "${PERF_DIR}/history-entries.txt"
    fi
    
    log_info "Saved build metrics to history"
}

get_baseline() {
    # Get the average of last 5 builds for baseline
    if command -v jq &> /dev/null && [[ -f "$HISTORY_FILE" ]]; then
        local avg_time=$(jq '.builds[-5:] | map(.build_time) | add / length' "$HISTORY_FILE" 2>/dev/null || echo "0")
        echo "$avg_time"
    else
        echo "0"
    fi
}

# =============================================================================
# REGRESSION DETECTION
# =============================================================================

check_regression() {
    local current_time=$1
    local baseline=$2
    
    if [[ "$baseline" == "0" ]] || [[ -z "$baseline" ]]; then
        log_warning "No baseline available for comparison"
        return 0
    fi
    
    local percent_change=$(echo "scale=2; (($current_time - $baseline) / $baseline) * 100" | bc)
    
    log_info "Build time change: ${percent_change}%"
    log_info "Current: ${current_time}s, Baseline: ${baseline}s"
    
    # Check against thresholds
    local abs_change=${percent_change#-}
    
    if (( $(echo "$abs_change > $THRESHOLD_FAILURE" | bc -l) )); then
        log_error "PERFORMANCE REGRESSION DETECTED: ${percent_change}% increase!"
        log_error "This exceeds the failure threshold of ${THRESHOLD_FAILURE}%"
        return 1
    elif (( $(echo "$abs_change > $THRESHOLD_WARNING" | bc -l) )); then
        log_warning "Performance warning: ${percent_change}% change from baseline"
        return 0
    else
        log_success "Build performance within acceptable range"
        return 0
    fi
}

# =============================================================================
# REPORTING
# =============================================================================

generate_report() {
    local report_file="${PERF_DIR}/performance-report.txt"
    
    cat > "$report_file" << EOF
================================================================================
Vantis Media Player - Build Performance Report
================================================================================
Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

Current Build Metrics:
  - Build Time: ${BUILD_TIME}s
  - Peak Memory: ${MAX_RSS} KB
  - CPU Utilization: ${CPU_PERCENT}%
  - Major Page Faults: ${MAJOR_PAGEFAULTS}

Build Environment:
  - Rust: $(rustc --version 2>/dev/null || echo 'N/A')
  - OS: $(uname -s) $(uname -m)
  - CPU Cores: $(nproc 2>/dev/null || echo 'N/A')
  - Total Memory: $(free -h | awk '/Mem:/ {print $2}')

Historical Data:
  - Total builds tracked: $(jq '.builds | length' "$HISTORY_FILE" 2>/dev/null || echo 'N/A')
  - Baseline (avg last 5): $(get_baseline)s
  - Performance trend: $(get_trend)

================================================================================
EOF
    
    log_info "Performance report saved to: $report_file"
    cat "$report_file"
}

get_trend() {
    if command -v jq &> /dev/null && [[ -f "$HISTORY_FILE" ]]; then
        local last_5_avg=$(jq '.builds[-5:] | map(.build_time) | add / length' "$HISTORY_FILE" 2>/dev/null || echo "0")
        local prev_5_avg=$(jq '.builds[-10:-5] | map(.build_time) | add / length' "$HISTORY_FILE" 2>/dev/null || echo "0")
        
        if [[ "$prev_5_avg" != "0" ]] && [[ "$prev_5_avg" != "null" ]]; then
            local change=$(echo "scale=1; (($last_5_avg - $prev_5_avg) / $prev_5_avg) * 100" | bc 2>/dev/null || echo "0")
            echo "${change}%"
        else
            echo "insufficient data"
        fi
    else
        echo "no history"
    fi
}

# =============================================================================
# UTILITY FUNCTIONS
# =============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# =============================================================================
# MAIN
# =============================================================================

main() {
    local build_type="${1:-release}"
    
    log_info "Starting build performance monitoring..."
    log_info "Build type: $build_type"
    
    # Initialize
    init_performance_dir
    
    # Measure build
    measure_build "$build_type"
    
    # Save to history
    save_to_history "$BUILD_TIME" "$MAX_RSS" "$CPU_PERCENT" "$MAJOR_PAGEFAULTS"
    
    # Check for regression
    local baseline=$(get_baseline)
    check_regression "$BUILD_TIME" "$baseline"
    local regression_result=$?
    
    # Generate report
    generate_report
    
    exit $regression_result
}

# Run main function with provided arguments
main "$@"