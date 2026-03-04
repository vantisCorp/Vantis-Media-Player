#!/bin/bash
# DevContainer Post-Create Script for Vantis Media Player
# This script runs after the dev container is created

set -e

echo "🚀 Setting up Vantis Media Player DevContainer..."

# ============================================
# Update and upgrade packages
# ============================================
echo "📦 Updating packages..."
apt-get update -y
apt-get upgrade -y

# ============================================
# Install additional tools
# ============================================
echo "🔧 Installing additional tools..."
apt-get install -y \
    curl \
    wget \
    git \
    vim \
    tmux \
    htop \
    tree \
    jq \
    ripgrep \
    fd-find \
    bat \
    exa \
    zsh \
    zsh-autosuggestions \
    zsh-syntax-highlighting \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    ffmpeg \
    protobuf-compiler

# ============================================
# Install Rust toolchain
# ============================================
echo "🦀 Installing Rust toolchain..."
if ! command -v rustup &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Add Rust components
rustup component add rustfmt clippy rust-src rust-analyzer

# ============================================
# Install Node.js tools
# ============================================
echo "📜 Installing Node.js tools..."
npm install -g \
    pnpm \
    yarn \
    typescript \
    ts-node \
    @vercel/ncc \
    prettier \
    eslint \
    @typescript-eslint/parser \
    @typescript-eslint/eslint-plugin

# ============================================
# Install Python tools
# ============================================
echo "🐍 Installing Python tools..."
pip3 install --upgrade pip
pip3 install \
    black \
    isort \
    flake8 \
    mypy \
    pytest \
    pytest-cov \
    pylint

# ============================================
# Install additional CLI tools
# ============================================
echo "🛠️  Installing CLI tools..."

# Install latest GitHub CLI
if ! command -v gh &> /dev/null; then
    curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
    echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null
    apt-get update
    apt-get install gh -y
fi

# Install Just command runner
if ! command -v just &> /dev/null; then
    curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin
fi

# Install watchexec for file watching
if ! command -v watchexec &> /dev/null; then
    cargo install watchexec-cli
fi

# Install cargo-binstall for faster binary installs
cargo install cargo-binstall

# Install useful cargo tools
cargo binstall -y \
    cargo-edit \
    cargo-outdated \
    cargo-update \
    cargo-audit \
    cargo-license \
    cargo-tree \
    cargo-make \
    cross

# ============================================
# Configure Git
# ============================================
echo "⚙️  Configuring Git..."
git config --global core.autocrlf input
git config --global core.eol lf
git config --global init.defaultBranch main
git config --global fetch.prune true
git config --global rebase.autoStash true

# ============================================
# Setup Oh My Zsh
# ============================================
echo "🎨 Setting up Oh My Zsh..."
if [ ! -d "$HOME/.oh-my-zsh" ]; then
    sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
fi

# Add custom zsh configuration
cat >> ~/.zshrc << 'EOF'

# Vantis Media Player DevContainer Configuration
export PATH="$HOME/.cargo/bin:$PATH"
export RUST_BACKTRACE=1

# Aliases
alias ll='exa -la --git'
alias lg='lazygit'
alias cat='bat'
alias find='fd'
alias grep='rg'
alias dc='docker compose'
alias k='kubectl'

# Cargo aliases
alias cb='cargo build'
alias cr='cargo run'
alias ct='cargo test'
alias cc='cargo clippy'
alias cf='cargo fmt'
alias ca='cargo add'
alias cu='cargo update'

# Node aliases
alias ni='npm install'
alias nr='npm run'
alias nt='npm test'
alias nb='npm run build'

# Python aliases
alias pi='pip install'
alias pt='pytest'

EOF

# ============================================
# Install VS Code extensions settings
# ============================================
echo "💻 Setting up VS Code settings..."
mkdir -p ~/.vscode

# ============================================
# Setup project dependencies
# ============================================
echo "📥 Installing project dependencies..."
if [ -f "Cargo.toml" ]; then
    cargo fetch
fi

if [ -f "package.json" ]; then
    npm install
fi

# ============================================
# Create workspace directories
# ============================================
echo "📁 Creating workspace directories..."
mkdir -p \
    /workspace/logs \
    /workspace/data \
    /workspace/temp \
    /workspace/cache

# ============================================
# Setup Git hooks
# ============================================
echo "🪝 Setting up Git hooks..."
if [ -f ".git/hooks/pre-commit" ]; then
    chmod +x .git/hooks/pre-commit
fi

# ============================================
# Display welcome message
# ============================================
echo ""
echo "✅ DevContainer setup complete!"
echo ""
echo "🎉 Welcome to Vantis Media Player Development Environment!"
echo ""
echo "📚 Quick Start:"
echo "  - Rust: cargo build --release"
echo "  - Run: cargo run"
echo "  - Test: cargo test"
echo "  - Format: cargo fmt"
echo "  - Lint: cargo clippy"
echo ""
echo "🔗 Useful Links:"
echo "  - Documentation: https://vantis.media/docs"
echo "  - Discord: https://discord.gg/A5MzwsRj7D"
echo "  - GitHub: https://github.com/vantisCorp/VantisMedia"
echo ""
echo "Happy coding! 🚀"
echo ""