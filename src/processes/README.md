# Processes Layer

Complex business processes spanning multiple features.

## Purpose

- Coordinate multi-step workflows
- Orchestrate features and entities
- Handle complex state transitions
- Implement business processes

## Structure

```
processes/
├── index.ts
├── onboarding/         # New user onboarding
│   ├── index.ts
│   ├── model.ts
│   └── ui/
├── media-sync/         # Library synchronization
├── plugin-setup/       # Plugin installation wizard
└── update/             # Application update process
```

## Processes

### `onboarding`
New user setup workflow:
- Welcome screen
- Library setup
- Default settings
- Feature introduction

### `media-sync`
Library synchronization:
- Scan directories
- Parse metadata
- Generate thumbnails
- Update database

### `plugin-setup`
Plugin installation wizard:
- Select plugins
- Configure options
- Download and install
- Verify installation

### `update`
Application update handling:
- Check for updates
- Download update
- Install and restart
- Rollback on failure

## Rules

1. Processes span multiple features
2. Processes have clear start/end states
3. Can use features, entities, shared
4. Should handle errors gracefully

## Example

```typescript
// processes/onboarding/index.ts
export { useOnboarding } from './model';
export { OnboardingWizard } from './ui';
export type { OnboardingStep } from './types';
```