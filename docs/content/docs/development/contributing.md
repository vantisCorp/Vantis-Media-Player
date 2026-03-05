---
sidebar_position: 5
title: Contributing
sidebar_label: Contributing
---

# Contributing

Guide to contributing to Vantis Media Player project.

## Overview

Thank you for your interest in contributing to Vantis Media Player! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

### Our Pledge

We are committed to making participation in our project a harassment-free experience for everyone, regardless of level of experience, gender, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

### Our Standards

**Positive behaviors include:**
- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

**Unacceptable behaviors include:**
- The use of sexualized language or imagery
- Trolling, insulting/derogatory comments, or personal/political attacks
- Public or private harassment
- Publishing others' private information without permission
- Other unethical or unprofessional conduct

## Getting Started

### Prerequisites

Before contributing, ensure you have:
- Read the project documentation
- Set up your development environment
- Familiarized yourself with the codebase

### Finding Issues to Work On

1. **Good First Issues**: Labeled with `good first issue`
2. **Help Wanted**: Labeled with `help wanted`
3. **Bug Reports**: Labeled with `bug`
4. **Feature Requests**: Labeled with `enhancement`

### Claiming an Issue

Before working on an issue:
1. Comment on the issue to express interest
2. Wait for maintainer approval
3. Update the issue with your progress
4. Mention the issue number in your PR

## Development Workflow

### Fork and Clone

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/Vantis-Media-Player.git
cd Vantis-Media-Player

# Add upstream remote
git remote add upstream https://github.com/vantisCorp/Vantis-Media-Player.git
```

### Create a Branch

```bash
# Fetch latest changes
git fetch upstream

# Create feature branch
git checkout -b feature/your-feature-name

# Or bugfix branch
git checkout -b fix/your-bugfix-name
```

### Make Changes

1. Write clear, descriptive commit messages
2. Follow the code style guidelines
3. Add tests for new features
4. Update documentation
5. Ensure all tests pass

### Commit Guidelines

Follow conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test changes
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

**Examples:**

```bash
# Feature
git commit -m "feat(player): add support for WebM format"

# Bug fix
git commit -m "fix(player): resolve audio sync issue on Safari"

# Documentation
git commit -m "docs(api): update Player API documentation"

# Refactor
git commit -m "refactor(state): simplify state management logic"
```

### Sync with Upstream

```bash
# Fetch upstream changes
git fetch upstream

# Rebase your branch
git rebase upstream/main

# Resolve any conflicts
git add .
git rebase --continue

# Or abort if needed
git rebase --abort
```

## Pull Request Process

### Before Submitting

1. **Test your changes thoroughly**
   ```bash
   pnpm test
   pnpm lint
   pnpm build
   ```

2. **Update documentation**
   - Update README if needed
   - Update CHANGELOG.md
   - Add inline code comments

3. **Add tests**
   - Unit tests for new features
   - Integration tests for APIs
   - E2E tests for user flows

4. **Follow code style**
   ```bash
   pnpm lint
   pnpm format
   ```

### Creating a Pull Request

1. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Create a pull request on GitHub
   - Use a clear title
   - Reference related issues
   - Provide a detailed description
   - Add screenshots for UI changes

3. Fill out the PR template:
   ```markdown
   ## Description
   Brief description of changes
   
   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update
   
   ## Related Issues
   Fixes #123
   
   ## Testing
   Describe testing done
   
   ## Checklist
   - [ ] Tests pass
   - [ ] Linting passes
   - [ ] Documentation updated
   - [ ] No breaking changes (or documented)
   ```

### PR Review Process

1. **Automated Checks**
   - CI/CD pipeline runs
   - Tests must pass
   - Linting must pass

2. **Code Review**
   - Maintainers review your code
   - Address feedback promptly
   - Make requested changes

3. **Approval and Merge**
   - At least one approval required
   - All discussions resolved
   - Maintainer merges PR

## Coding Standards

### Code Style

Follow the project's code style:

```typescript
// Use TypeScript
interface PlayerProps {
  src: string
  autoplay?: boolean
  controls?: boolean
  onPlay?: () => void
}

// Use functional components
export const Player: React.FC<PlayerProps> = ({
  src,
  autoplay = false,
  controls = true,
  onPlay
}) => {
  // Implementation
}

// Use hooks for state and effects
const [isPlaying, setIsPlaying] = useState(false)
useEffect(() => {
  // Effect logic
}, [dependency])
```

### Naming Conventions

- **Components**: PascalCase (`Player`, `VideoControls`)
- **Functions**: camelCase (`playVideo`, `pauseVideo`)
- **Constants**: UPPER_SNAKE_CASE (`MAX_BUFFER_SIZE`)
- **Files**: kebab-case (`player.tsx`, `video-controls.tsx`)

### Documentation

```typescript
/**
 * Plays a video from the specified source
 * @param src - URL of the video source
 * @param options - Playback options
 * @param options.autoplay - Whether to autoplay the video
 * @param options.startTime - Start time in seconds
 * @returns Promise that resolves when video starts playing
 * @throws {Error} If video source is invalid
 * @example
 * ```ts
 * await playVideo('https://example.com/video.mp4', {
 *   autoplay: true,
 *   startTime: 10
 * })
 * ```
 */
async function playVideo(
  src: string,
  options?: { autoplay?: boolean; startTime?: number }
): Promise<void> {
  // Implementation
}
```

## Testing Guidelines

### Test Coverage

- Maintain minimum 80% code coverage
- Test all public APIs
- Test edge cases and error conditions

### Writing Tests

```typescript
describe('Player', () => {
  it('should render video element', () => {
    render(<Player src="test.mp4" />)
    expect(screen.getByTestId('player')).toBeInTheDocument()
  })

  it('should play video when play is called', async () => {
    const { result } = renderHook(() => usePlayer())
    await act(() => result.current.play())
    expect(result.current.isPlaying).toBe(true)
  })
})
```

### Test Files Organization

```
src/
├── player/
│   ├── player.tsx
│   ├── player.test.tsx
│   └── player.stories.tsx
```

## Documentation

### Updating Documentation

- Keep documentation in sync with code changes
- Update API references for new methods
- Add examples for new features
- Update CHANGELOG.md

### Documentation Style

- Use clear, concise language
- Provide code examples
- Include screenshots for UI changes
- Link to related documentation

## Release Process

### Versioning

We follow Semantic Versioning:
- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality
- **PATCH**: Backwards-compatible bug fixes

### Release Checklist

- [ ] All tests passing
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped
- [ ] Release notes prepared
- [ ] Tagged and published

## Community Guidelines

### Communication

- Be respectful and constructive
- Use GitHub issues for bug reports
- Use discussions for questions
- Join our Discord community

### Getting Help

- Read the documentation first
- Search existing issues
- Ask questions in discussions
- Be patient with responses

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md file
- Release notes
- Project website
- Social media

## License

By contributing to Vantis Media Player, you agree that your contributions will be licensed under the MIT License.

## Questions?

- Check existing documentation
- Search GitHub issues
- Start a discussion
- Contact maintainers

Thank you for contributing to Vantis Media Player! 🎉