# App Layer

The application layer is the entry point of the application.

## Purpose

- Initialize the application
- Provide global context/providers
- Set up routing
- Configure global styles
- Handle application-wide error boundaries

## Structure

```
app/
├── index.ts           # Public API
├── App.tsx            # Root component
├── router.tsx         # Route configuration
├── providers.tsx      # Context providers
├── styles/            # Global styles
│   ├── index.css
│   └── variables.css
└── env.ts             # Environment configuration
```

## Contents

### Providers
- Theme provider (dark/light mode)
- Authentication provider
- State management provider
- Media player provider

### Router
- Route definitions
- Route guards
- Layout wrappers

### Styles
- CSS reset
- Design tokens
- Global typography
- Animation definitions

## Usage

```typescript
// app/index.ts
export { App } from './App';
export { router } from './router';
export { providers } from './providers';
```

## Rules

1. This layer should be as thin as possible
2. No business logic here - only composition
3. All imports come from lower layers
4. Single entry point for the application