#!/bin/bash
# Vantis Media Player - Pre-commit Hook
# Runs checks before committing

echo "🔍 Running pre-commit checks..."
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Staged files
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(rs|toml|md)$' || true)

if [ -z "$STAGED_FILES" ]; then
    echo -e "${YELLOW}⚠️  No relevant files staged${NC}"
    exit 0
fi

echo "📋 Staged files:"
echo "$STAGED_FILES"
echo ""

# 1. Rust format check
echo "📝 Checking code formatting..."
if ! cargo fmt --all -- --check; then
    echo -e "${RED}❌ Code formatting issues found${NC}"
    echo "Run 'cargo fmt --all' to fix"
    exit 1
fi
echo -e "${GREEN}✅ Code is properly formatted${NC}"
echo ""

# 2. Clippy check (only if Rust files are staged)
if echo "$STAGED_FILES" | grep -q '\.rs$'; then
    echo "🔧 Running Clippy..."
    if ! cargo clippy --all-targets --all-features -- -D warnings; then
        echo -e "${RED}❌ Clippy found issues${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ No Clippy warnings${NC}"
    echo ""
fi

# 3. Build check
echo "🔨 Checking if code compiles..."
if ! cargo check --all-targets --all-features; then
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Code compiles successfully${NC}"
echo ""

# 4. Unit tests (fast)
echo "🧪 Running unit tests..."
if ! cargo test --workspace --lib --bins --quiet; then
    echo -e "${RED}❌ Tests failed${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Tests passed${NC}"
echo ""

# 5. Check for TODO/FIXME in staged files
echo "📝 Checking for TODO/FIXME comments..."
if git diff --cached | grep -iE '^\+.*TODO|^\+.*FIXME|^\+.*HACK|^\+.*XXX' > /dev/null; then
    echo -e "${YELLOW}⚠️  Warning: TODO/FIXME comments found in staged files${NC}"
    git diff --cached | grep -iE '^\+.*TODO|^\+.*FIXME|^\+.*HACK|^\+.*XXX' | head -5
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi
echo ""

echo -e "${GREEN}✨ All pre-commit checks passed!${NC}"
exit 0