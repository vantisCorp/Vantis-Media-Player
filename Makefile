# Vantis Media Player - Makefile
# One source of truth for all operations

.PHONY: help setup install build test lint format clean release docker deploy docs

# Default target
help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# Setup
setup: ## Initialize development environment
	@echo "🚀 Setting up Vantis Media Player..."
	@cp .env.example .env
	@pnpm install
	@echo "✅ Setup complete!"

# Install
install: ## Install dependencies
	@echo "📦 Installing dependencies..."
	@pnpm install
	@echo "✅ Dependencies installed!"

# Build
build: ## Build all packages
	@echo "🔨 Building Vantis Media Player..."
	@pnpm build
	@echo "✅ Build complete!"

# Test
test: ## Run all tests
	@echo "🧪 Running tests..."
	@pnpm test
	@echo "✅ Tests passed!"

# Lint
lint: ## Run linting
	@echo "🔍 Running linters..."
	@pnpm lint
	@echo "✅ Linting complete!"

# Format
format: ## Format code
	@echo "✨ Formatting code..."
	@pnpm format
	@echo "✅ Formatting complete!"

# Clean
clean: ## Clean build artifacts
	@echo "🧹 Cleaning..."
	@rm -rf node_modules .turbo dist build target
	@echo "✅ Clean complete!"

# Release
release: ## Create a new release
	@echo "📦 Creating release..."
	@pnpm release
	@echo "✅ Release complete!"

# Docker
docker: ## Build Docker image
	@echo "🐳 Building Docker image..."
	@docker build -t vantis/vantis-player:latest .
	@echo "✅ Docker image built!"

# Deploy
deploy: ## Deploy to production
	@echo "🚀 Deploying..."
	@pnpm deploy
	@echo "✅ Deploy complete!"

# Docs
docs: ## Build documentation
	@echo "📚 Building documentation..."
	@pnpm docs
	@echo "✅ Documentation built!"

# Dev
dev: ## Start development server
	@echo "🎮 Starting development server..."
	@pnpm dev

# Watch
watch: ## Watch for changes and rebuild
	@echo "👀 Watching for changes..."
	@pnpm watch

# Audit
audit: ## Audit dependencies
	@echo "🔒 Auditing dependencies..."
	@pnpm audit
	@echo "✅ Audit complete!"

# Security
security: ## Run security checks
	@echo "🛡️ Running security checks..."
	@pnpm security
	@echo "✅ Security checks complete!"

# Update
update: ## Update dependencies
	@echo "⬆️ Updating dependencies..."
	@pnpm update
	@echo "✅ Dependencies updated!"

# CI
ci: lint test security ## Run CI pipeline locally
	@echo "✅ CI pipeline complete!"
