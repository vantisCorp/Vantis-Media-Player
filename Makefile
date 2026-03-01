# Vantis Media Player - Makefile
#
# This Makefile provides convenient commands for building, testing,
# and developing Vantis Media Player.

# Variables
CARGO = cargo
RUSTUP = rustup
TARGET = target/release
PROJECT_NAME = vantis-player

# Colors
RESET = \033[0m
BOLD = \033[1m
GREEN = \033[32m
YELLOW = \033[33m
BLUE = \033[34m
RED = \033[31m

.PHONY: help
help:
	@echo "$(BOLD)Vantis Media Player - Makefile$(RESET)"
	@echo ""
	@echo "$(BOLD)Available targets:$(RESET)"
	@echo ""
	@echo "$(GREEN)Development:$(RESET)"
	@echo "  $(BLUE)make build$(RESET)           - Build the project (debug)"
	@echo "  $(BLUE)make release$(RESET)         - Build the project (release)"
	@echo "  $(BLUE)make clean$(RESET)           - Clean build artifacts"
	@echo "  $(BLUE)make check$(RESET)           - Run cargo check"
	@echo "  $(BLUE)make fmt$(RESET)             - Format code"
	@echo "  $(BLUE)make clippy$(RESET)          - Run clippy linter"
	@echo ""
	@echo "$(GREEN)Testing:$(RESET)"
	@echo "  $(BLUE)make test$(RESET)            - Run all tests"
	@echo "  $(BLUE)make test-verbose$(RESET)    - Run tests with output"
	@echo "  $(BLUE)make test-integration$(RESET)- Run integration tests"
	@echo "  $(BLUE)make bench$(RESET)           - Run benchmarks"
	@echo ""
	@echo "$(GREEN)Documentation:$(RESET)"
	@echo "  $(BLUE)make docs$(RESET)            - Generate documentation"
	@echo "  $(BLUE)make docs-open$(RESET)       - Generate and open docs"
	@echo ""
	@echo "$(GREEN)Examples:$(RESET)"
	@echo "  $(BLUE)make examples$(RESET)        - Build all examples"
	@echo "  $(BLUE)make run-example$(RESET)     - Run specific example (NAME=name)"
	@echo ""
	@echo "$(GREEN)Plugins:$(RESET)"
	@echo "  $(BLUE)make build-plugin$(RESET)    - Build plugin example (DIR=dir)"
	@echo "  $(BLUE)make build-wat$(RESET)       - Build WAT plugin"
	@echo ""
	@echo "$(GREEN)Docker:$(RESET)"
	@echo "  $(BLUE)make docker-build$(RESET)    - Build Docker image"
	@echo "  $(BLUE)make docker-run$(RESET)      - Run Docker container"
	@echo ""
	@echo "$(GREEN)CI/CD:$(RESET)"
	@echo "  $(BLUE)make ci$(RESET)              - Run CI checks"
	@echo "  $(BLUE)make audit$(RESET)           - Security audit"
	@echo "  $(BLUE)make deny$(RESET)            - License check"
	@echo ""
	@echo "$(GREEN)Installation:$(RESET)"
	@@echo "  $(BLUE)make install$(RESET)        - Install to local system"
	@echo "  $(BLUE)make uninstall$(RESET)      - Uninstall from local system"
	@echo ""
	@echo "$(GREEN)Misc:$(RESET)"
	@echo "  $(BLUE)make update$(RESET)          - Update dependencies"
	@echo "  $(BLUE)make outdated$(RESET)        - Check outdated deps"
	@echo "  $(BLUE)make tree$(RESET)            - Show project tree"

# Build targets
.PHONY: build
build:
	@echo "$(GREEN)Building project (debug)...$(RESET)"
	$(CARGO) build

.PHONY: release
release:
	@echo "$(GREEN)Building project (release)...$(RESET)"
	$(CARGO) build --release

.PHONY: clean
clean:
	@echo "$(GREEN)Cleaning build artifacts...$(RESET)"
	$(CARGO) clean
	rm -rf $(TARGET)

.PHONY: check
check:
	@echo "$(GREEN)Running cargo check...$(RESET)"
	$(CARGO) check --all-targets

.PHONY: fmt
fmt:
	@echo "$(GREEN)Formatting code...$(RESET)"
	$(CARGO) fmt

.PHONY: clippy
clippy:
	@echo "$(GREEN)Running clippy...$(RESET)"
	$(CARGO) clippy --all-targets -- -D warnings

# Testing targets
.PHONY: test
test:
	@echo "$(GREEN)Running tests...$(RESET)"
	$(CARGO) test

.PHONY: test-verbose
test-verbose:
	@echo "$(GREEN)Running tests (verbose)...$(RESET)"
	$(CARGO) test -- --nocapture

.PHONY: test-integration
test-integration:
	@echo "$(GREEN)Running integration tests...$(RESET)"
	$(CARGO) test --test integration_tests

.PHONY: bench
bench:
	@echo "$(GREEN)Running benchmarks...$(RESET)"
	$(CARGO) bench

# Documentation targets
.PHONY: docs
docs:
	@echo "$(GREEN)Generating documentation...$(RESET)"
	$(CARGO) doc --no-deps

.PHONY: docs-open
docs-open: docs
	@echo "$(GREEN)Opening documentation...$(RESET)"
	$(CARGO) doc --no-deps --open

# Example targets
.PHONY: examples
examples:
	@echo "$(GREEN)Building examples...$(RESET)"
	$(CARGO) build --examples

.PHONY: run-example
run-example:
	@if [ -z "$(NAME)" ]; then \
		echo "$(RED)Error: NAME parameter required$(RESET)"; \
		echo "Usage: make run-example NAME=example_name"; \
		exit 1; \
	fi
	@echo "$(GREEN)Running example: $(NAME)$(RESET)"
	$(CARGO) run --example $(NAME)

# Plugin targets
.PHONY: build-plugin
build-plugin:
	@if [ -z "$(DIR)" ]; then \
		echo "$(RED)Error: DIR parameter required$(RESET)"; \
		echo "Usage: make build-plugin DIR=path/to/plugin"; \
		exit 1; \
	fi
	@echo "$(GREEN)Building plugin: $(DIR)$(RESET)"
	cd $(DIR) && $(CARGO) build --release --target wasm32-unknown-unknown

.PHONY: build-wat
build-wat:
	@echo "$(GREEN)Building WAT plugin...$(RESET)"
	wat2wasm examples/wat_plugin_template.wat -o examples/wat_plugin.wasm

# Docker targets
.PHONY: docker-build
docker-build:
	@echo "$(GREEN)Building Docker image...$(RESET)"
	docker build -t $(PROJECT_NAME):latest .

.PHONY: docker-run
docker-run:
	@echo "$(GREEN)Running Docker container...$(RESET)"
	docker run -it --rm \
		-v $(PWD):/workspace \
		--device /dev/snd \
		$(PROJECT_NAME):latest

# CI/CD targets
.PHONY: ci
ci: fmt clippy test
	@echo "$(GREEN)CI checks passed!$(RESET)"

.PHONY: audit
audit:
	@echo "$(GREEN)Running security audit...$(RESET)"
	cargo audit

.PHONY: deny
deny:
	@echo "$(GREEN)Running license check...$(RESET)"
	cargo deny check

# Installation targets
.PHONY: install
install:
	@echo "$(GREEN)Installing to local system...$(RESET)"
	$(CARGO) install --path .

.PHONY: uninstall
uninstall:
	@echo "$(GREEN)Uninstalling from local system...$(RESET)"
	$(CARGO) uninstall $(PROJECT_NAME)

# Miscellaneous targets
.PHONY: update
update:
	@echo "$(GREEN)Updating dependencies...$(RESET)"
	$(CARGO) update

.PHONY: outdated
outdated:
	@echo "$(GREEN)Checking outdated dependencies...$(RESET)"
	$(CARGO) outdated

.PHONY: tree
tree:
	@echo "$(GREEN)Project tree:$(RESET)"
	tree -I 'target|.git' -L 3

# Development helpers
.PHONY: watch
watch:
	@echo "$(GREEN)Watching for changes...$(RESET)"
	cargo watch -x check -x test -x clippy

.PHONY: run
run:
	@echo "$(GREEN)Running project...$(RESET)"
	$(CARGO) run

.PHONY: run-release
run-release: release
	@echo "$(GREEN)Running release build...$(RESET)"
	$(TARGET)/$(PROJECT_NAME)

# Pre-commit hooks
.PHONY: pre-commit
pre-commit: fmt clippy test
	@echo "$(GREEN)Pre-commit checks passed!$(RESET)"

# Release helpers
.PHONY: release-check
release-check: clean fmt clippy test docs
	@echo "$(GREEN)Release checks passed!$(RESET)"

.PHONY: package
package: release
	@echo "$(GREEN)Creating release package...$(RESET)"
	cd $(TARGET) && tar -czf $(PROJECT_NAME)-$(VERSION).tar.gz $(PROJECT_NAME)

# Quick commands
.PHONY: q
q: build
	@echo "$(GREEN)Quick build complete!$(RESET)"

.PHONY: qt
qt: test
	@echo "$(GREEN)Quick test complete!$(RESET)"

.PHONY: qr
qr: release
	@echo "$(GREEN)Quick release build complete!$(RESET)"