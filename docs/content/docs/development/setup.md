---
sidebar_position: 1
title: Development Setup
sidebar_label: Setup
---

# Development Setup

Set up your development environment for contributing to Vantis Media Player.

## Prerequisites

### Required Software

Install the following software on your development machine:

- **Node.js**: Version 20.x or higher
- **pnpm**: Version 8.x or higher
- **Git**: Version 2.40 or higher
- **Docker**: Version 24.x or higher
- **Python**: Version 3.11 or higher (for build scripts)

### Optional Tools

These tools are recommended but not required:

- **Visual Studio Code**: Recommended IDE with extensions
- **Chrome**: For browser testing
- **Firefox**: For cross-browser testing
- **Safari**: For macOS testing
- **Wireshark**: For network debugging

## Installation

### Clone the Repository

```bash
# Clone the repository
git clone https://github.com/vantisCorp/Vantis-Media-Player.git
cd Vantis-Media-Player

# Verify Git configuration
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

### Install Node Dependencies

```bash
# Install pnpm globally (if not already installed)
npm install -g pnpm@latest

# Install project dependencies
pnpm install

# Verify installation
pnpm --version
node --version
```

### Setup Environment

```bash
# Copy environment template
cp .env.example .env.local

# Edit environment variables
nano .env.local
```

Environment variables:

```env
# Application
NODE_ENV=development
PORT=3000

# API Configuration
API_BASE_URL=http://localhost:3000/api
API_KEY=your_api_key_here

# Feature Flags
ENABLE_EXPERIMENTAL_FEATURES=true
ENABLE_DEBUG_MODE=true

# Logging
LOG_LEVEL=debug

# Testing
COVERAGE_THRESHOLD=80
```

## IDE Setup

### Visual Studio Code

Install the recommended extensions:

```bash
# Install extensions via CLI
code --install-extension dbaeumer.vscode-eslint
code --install-extension esbenp.prettier-vscode
code --install-extension ms-vscode.vscode-typescript-next
code --install-extension bradlc.vscode-tailwindcss
code --install-extension firsttris.vscode-jest-runner
code --install-extension eamodio.gitlens
```

### VSCode Settings

Create `.vscode/settings.json`:

```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": true
  },
  "typescript.tsdk": "node_modules/typescript/lib",
  "typescript.enablePromptUseWorkspaceTsdk": true,
  "eslint.validate": [
    "javascript",
    "javascriptreact",
    "typescript",
    "typescriptreact"
  ],
  "files.eol": "\n",
  "files.trimTrailingWhitespace": true,
  "files.insertFinalNewline": true,
  "jest.jestCommandLine": "pnpm test --",
  "jest.autoRun": "watch"
}
```

### VSCode Extensions

Create `.vscode/extensions.json`:

```json
{
  "recommendations": [
    "dbaeumer.vscode-eslint",
    "esbenp.prettier-vscode",
    "ms-vscode.vscode-typescript-next",
    "bradlc.vscode-tailwindcss",
    "firsttris.vscode-jest-runner",
    "eamodio.gitlens",
    "streetsidesoftware.code-spell-checker",
    "yzhang.markdown-all-in-one"
  ]
}
```

## Running Development Server

### Start Development Server

```bash
# Start development server
pnpm dev

# Start with specific port
PORT=4000 pnpm dev

# Start with turbo watch mode
pnpm dev --filter=vantis-player
```

### Verify Setup

```bash
# Check if server is running
curl http://localhost:3000/health

# Run health check
pnpm run health
```

## Project Structure

Understanding the project structure:

```
Vantis-Media-Player/
├── apps/
│   ├── web/                 # Web application
│   ├── desktop/             # Desktop application (Electron/Tauri)
│   └── mobile/              # Mobile application (React Native)
├── packages/
│   ├── player/              # Core player library
│   ├── ui/                  # UI components
│   ├── config/              # Shared configuration
│   ├── eslint-config/       # ESLint configuration
│   ├── typescript-config/   # TypeScript configuration
│   └── utils/               # Utility functions
├── docs/                    # Documentation
├── scripts/                 # Build and utility scripts
├── tests/                   # E2E tests
├── .github/                 # GitHub workflows
├── turbo.json               # Turborepo configuration
├── package.json
└── pnpm-workspace.yaml
```

## Common Development Tasks

### Add a New Package

```bash
# Create new package
pnpm create vite packages/my-package --template react-ts

# Add to workspace
cd packages/my-package
pnpm install

# Register in turbo.json
```

### Add Dependencies

```bash
# Install to specific package
pnpm add lodash --filter @vantis/player

# Install as dev dependency
pnpm add -D typescript --filter @vantis/player

# Install to all packages
pnpm add -w lodash
```

### Run Scripts

```bash
# List available scripts
pnpm run

# Run script in specific package
pnpm --filter @vantis/player test

# Run script in all packages
pnpm -r test
```

## Testing Setup

### Run Tests

```bash
# Run all tests
pnpm test

# Run tests in watch mode
pnpm test:watch

# Run tests with coverage
pnpm test:coverage

# Run specific test file
pnpm test player.test.ts
```

### E2E Testing

```bash
# Run E2E tests
pnpm test:e2e

# Run E2E tests with UI
pnpm test:e2e:ui

# Run specific E2E test
pnpm test:e2e player.spec.ts
```

## Git Workflow

### Branch Strategy

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Create bugfix branch
git checkout -b fix/your-bugfix-name

# Create hotfix branch
git checkout -b hotfix/your-hotfix-name
```

### Commit Guidelines

Follow conventional commits:

```bash
# Feature
git commit -m "feat: add new video codec support"

# Bug fix
git commit -m "fix: resolve audio synchronization issue"

# Documentation
git commit -m "docs: update API documentation"

# Refactor
git commit -m "refactor: improve player state management"

# Performance
git commit -m "perf: optimize video decoding performance"
```

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test changes
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

## Troubleshooting

### Common Issues

**Port already in use:**
```bash
# Find process using port 3000
lsof -i :3000

# Kill process
kill -9 <PID>

# Or use different port
PORT=4000 pnpm dev
```

**Dependency conflicts:**
```bash
# Clear node_modules
rm -rf node_modules packages/*/node_modules
rm pnpm-lock.yaml

# Reinstall dependencies
pnpm install
```

**TypeScript errors:**
```bash
# Rebuild TypeScript
pnpm build --filter @vantis/player

# Clear TypeScript cache
rm -rf packages/*/tsconfig.tsbuildinfo
```

## Next Steps

- [ ] Set up your IDE with recommended extensions
- [ ] Configure Git hooks for pre-commit checks
- [ ] Explore the project structure
- [ ] Read the architecture documentation
- [ ] Run the development server
- [ ] Create your first contribution