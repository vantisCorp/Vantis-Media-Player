---
sidebar_position: 3
title: Testing
sidebar_label: Testing
---

# Testing

Comprehensive testing strategies for Vantis Media Player including unit, integration, and end-to-end tests.

## Overview

Vantis Media Player uses a multi-layered testing approach to ensure code quality and reliability. The testing stack includes Jest for unit tests, React Testing Library for component testing, Playwright for E2E tests, and Vitest for fast unit testing.

## Testing Stack

### Core Testing Tools

- **Jest**: JavaScript testing framework
- **React Testing Library**: Component testing utilities
- **Playwright**: E2E testing framework
- **Vitest**: Fast unit test runner
- **MSW**: API mocking for tests
- **Testing Library**: User-centric testing utilities

### Installation

```bash
# Install testing dependencies
pnpm add -D jest @types/jest
pnpm add -D @testing-library/react @testing-library/jest-dom
pnpm add -D @playwright/test
pnpm add -D vitest @vitest/ui
pnpm add -D msw msw-node
```

## Unit Testing

### Jest Configuration

Configure Jest in `jest.config.js`:

```javascript
const nextJest = require('next/jest')

const createJestConfig = nextJest({
  dir: './',
})

const customJestConfig = {
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
  testEnvironment: 'jest-environment-jsdom',
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
  },
  collectCoverageFrom: [
    'src/**/*.{js,jsx,ts,tsx}',
    '!src/**/*.d.ts',
    '!src/**/*.stories.{js,jsx,ts,tsx}',
    '!src/**/__tests__/**',
  ],
  testMatch: [
    '**/__tests__/**/*.[jt]s?(x)',
    '**/?(*.)+(spec|test).[jt]s?(x)',
  ],
  transform: {
    '^.+\\.(js|jsx|ts|tsx)$': ['@swc/jest', {}],
  },
}

module.exports = createJestConfig(customJestConfig)
```

### Writing Unit Tests

```typescript
// Player.test.ts
import { render, screen, fireEvent } from '@testing-library/react'
import { Player } from './Player'

describe('Player Component', () => {
  const mockProps = {
    src: 'https://example.com/video.mp4',
    autoplay: true,
    controls: true,
  }

  beforeEach(() => {
    jest.clearAllMocks()
  })

  it('renders player element', () => {
    render(<Player {...mockProps} />)
    const player = screen.getByTestId('player')
    expect(player).toBeInTheDocument()
  })

  it('handles play action', () => {
    const onPlay = jest.fn()
    render(<Player {...mockProps} onPlay={onPlay} />)
    
    const playButton = screen.getByRole('button', { name: /play/i })
    fireEvent.click(playButton)
    
    expect(onPlay).toHaveBeenCalled()
  })

  it('handles pause action', () => {
    const onPause = jest.fn()
    render(<Player {...mockProps} onPause={onPause} />)
    
    const pauseButton = screen.getByRole('button', { name: /pause/i })
    fireEvent.click(pauseButton)
    
    expect(onPause).toHaveBeenCalled()
  })

  it('updates current time', () => {
    render(<Player {...mockProps} />)
    const player = screen.getByTestId('player')
    
    fireEvent.timeUpdate(player, {
      target: { currentTime: 10 }
    })
    
    expect(screen.getByText('0:10')).toBeInTheDocument()
  })
})
```

### Running Unit Tests

```bash
# Run all tests
pnpm test

# Run tests in watch mode
pnpm test:watch

# Run tests with coverage
pnpm test:coverage

# Run specific test file
pnpm test Player.test.ts

# Run tests matching pattern
pnpm test -- --testNamePattern="Player"
```

### Testing Utilities

Create custom testing utilities in `src/test-utils.tsx`:

```typescript
import { render, RenderOptions } from '@testing-library/react'
import { ReactElement } from 'react'

const AllTheProviders = ({ children }: { children: React.ReactNode }) => {
  return (
    <ThemeProvider theme={theme}>
      <PlayerProvider>{children}</PlayerProvider>
    </ThemeProvider>
  )
}

const customRender = (
  ui: ReactElement,
  options?: Omit<RenderOptions, 'wrapper'>
) => render(ui, { wrapper: AllTheProviders, ...options })

export * from '@testing-library/react'
export { customRender as render }
```

## Integration Testing

### Component Integration Tests

```typescript
// PlayerControls.test.tsx
import { render, screen, fireEvent } from '@testing-library/react'
import { PlayerControls } from './PlayerControls'
import { PlayerProvider, usePlayer } from '../context/PlayerContext'

const TestWrapper = ({ children }: { children: React.ReactNode }) => {
  return (
    <PlayerProvider
      initialState={{
        isPlaying: false,
        currentTime: 0,
        duration: 100,
        volume: 1
      }}
    >
      {children}
    </PlayerProvider>
  )
}

describe('PlayerControls Integration', () => {
  it('integrates with player context', () => {
    render(
      <TestWrapper>
        <PlayerControls />
      </TestWrapper>
    )

    const playButton = screen.getByRole('button', { name: /play/i })
    fireEvent.click(playButton)

    expect(screen.getByRole('button', { name: /pause/i })).toBeInTheDocument()
  })
})
```

### API Integration Tests

```typescript
// api.test.ts
import { setupServer } from 'msw/node'
import { rest } from 'msw'
import { fetchVideoInfo } from './api'

const server = setupServer(
  rest.get('/api/video/:id', (req, res, ctx) => {
    return res(
      ctx.json({
        id: req.params.id,
        title: 'Test Video',
        duration: 120,
        format: 'mp4'
      })
    )
  })
)

beforeAll(() => server.listen())
afterEach(() => server.resetHandlers())
afterAll(() => server.close())

describe('API Integration', () => {
  it('fetches video info successfully', async () => {
    const result = await fetchVideoInfo('123')
    expect(result).toEqual({
      id: '123',
      title: 'Test Video',
      duration: 120,
      format: 'mp4'
    })
  })

  it('handles API errors', async () => {
    server.use(
      rest.get('/api/video/:id', (req, res, ctx) => {
        return res(ctx.status(500))
      })
    )

    await expect(fetchVideoInfo('123')).rejects.toThrow()
  })
})
```

## End-to-End Testing

### Playwright Configuration

Configure Playwright in `playwright.config.ts`:

```typescript
import { defineConfig, devices } from '@playwright/test'

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
  ],
  webServer: {
    command: 'pnpm dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
})
```

### Writing E2E Tests

```typescript
// player.spec.ts
import { test, expect } from '@playwright/test'

test.describe('Player E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('loads player successfully', async ({ page }) => {
    await expect(page.locator('[data-testid="player"]')).toBeVisible()
  })

  test('plays video', async ({ page }) => {
    const playButton = page.locator('[data-testid="play-button"]')
    await playButton.click()
    
    await expect(page.locator('[data-testid="pause-button"]')).toBeVisible()
  })

  test('seeks video', async ({ page }) => {
    const playButton = page.locator('[data-testid="play-button"]')
    await playButton.click()
    
    const progressBar = page.locator('[data-testid="progress-bar"]')
    await progressBar.click({ position: { x: 50, y: 0 } })
    
    await expect(page.locator('[data-testid="current-time"]')).toContainText('0:30')
  })

  test('adjusts volume', async ({ page }) => {
    const volumeSlider = page.locator('[data-testid="volume-slider"]')
    await volumeSlider.fill('50')
    
    await expect(volumeSlider).toHaveValue('50')
  })

  test('toggles fullscreen', async ({ page }) => {
    const fullscreenButton = page.locator('[data-testid="fullscreen-button"]')
    await fullscreenButton.click()
    
    const player = page.locator('[data-testid="player"]')
    await expect(player).toHaveClass(/fullscreen/)
  })
})
```

### Running E2E Tests

```bash
# Run E2E tests
pnpm test:e2e

# Run E2E tests in UI mode
pnpm test:e2e:ui

# Run E2E tests in headed mode
pnpm test:e2e --headed

# Run specific test file
pnpm test:e2e player.spec.ts

# Run tests on specific browser
pnpm test:e2e --project=chromium
```

## Visual Regression Testing

### Playwright Visual Tests

```typescript
// player-visual.spec.ts
import { test, expect } from '@playwright/test'

test('player visual regression', async ({ page }) => {
  await page.goto('/')
  await page.waitForLoadState('networkidle')
  
  await expect(page).toHaveScreenshot('player.png')
})

test('player playing state', async ({ page }) => {
  await page.goto('/')
  await page.click('[data-testid="play-button"]')
  await page.waitForTimeout(1000)
  
  await expect(page).toHaveScreenshot('player-playing.png')
})
```

## Performance Testing

### Lighthouse CI

```bash
# Install Lighthouse CI
npm install -g @lhci/cli

# Run Lighthouse CI
lhci autorun
```

### Custom Performance Tests

```typescript
// performance.test.ts
import { test, expect } from '@playwright/test'

test.describe('Performance Tests', () => {
  test('loads within performance budget', async ({ page }) => {
    const startTime = Date.now()
    await page.goto('/')
    const loadTime = Date.now() - startTime
    
    expect(loadTime).toBeLessThan(3000)
  })

  test('has good LCP', async ({ page }) => {
    const metrics = await page.evaluate(() => {
      return new Promise((resolve) => {
        new PerformanceObserver((list) => {
          const entries = list.getEntries()
          resolve(entries[0])
        }).observe({ entryTypes: ['largest-contentful-paint'] })
      })
    })
    
    expect(metrics).toHaveProperty('startTime')
    expect(metrics.startTime).toBeLessThan(2500)
  })
})
```

## Accessibility Testing

### Axe-core Integration

```typescript
// accessibility.test.ts
import { test, expect } from '@playwright/test'
import AxeBuilder from '@axe-core/playwright'

test('accessibility checks', async ({ page }) => {
  await page.goto('/')
  
  const accessibilityScanResults = await new AxeBuilder({ page })
    .withTags(['wcag2a', 'wcag2aa', 'wcag21aa'])
    .analyze()
  
  expect(accessibilityScanResults.violations).toEqual([])
})
```

## Test Coverage

### Coverage Configuration

```javascript
// jest.config.js
module.exports = {
  collectCoverageFrom: [
    'src/**/*.{js,jsx,ts,tsx}',
    '!src/**/*.d.ts',
    '!src/**/*.stories.{js,jsx,ts,tsx}',
    '!src/**/__tests__/**',
  ],
  coverageThresholds: {
    global: {
      branches: 80,
      functions: 80,
      lines: 80,
      statements: 80,
    },
  },
}
```

### Generating Coverage Reports

```bash
# Generate coverage report
pnpm test:coverage

# Generate coverage with HTML report
pnpm test:coverage -- --coverage-reporters=html

# Open coverage report
open coverage/lcov-report/index.html
```

## Testing Best Practices

1. **Write tests first** (Test-Driven Development)
2. **Test user behavior**, not implementation details
3. **Keep tests independent** and isolated
4. **Use descriptive test names**
5. **Mock external dependencies**
6. **Test edge cases** and error conditions
7. **Maintain test coverage** above 80%
8. **Run tests in CI/CD** pipeline
9. **Fix flaky tests** immediately
10. **Use page objects** for E2E tests

## Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - uses: pnpm/action-setup@v2
        with:
          version: 8
      
      - uses: actions/setup-node@v3
        with:
          node-version: 20
          cache: 'pnpm'
      
      - run: pnpm install
      - run: pnpm lint
      - run: pnpm test
      - run: pnpm test:e2e
      - run: pnpm test:coverage
      
      - uses: codecov/codecov-action@v3
        with:
          files: ./coverage/lcov.info
```

## Next Steps

- [ ] Set up testing infrastructure
- [ ] Write unit tests for core components
- [ ] Implement integration tests
- [ ] Create E2E test suite
- [ ] Configure test coverage reporting
- [ ] Set up CI/CD testing pipeline