# Vantis Media Player - Source (Feature-Sliced Design)

This directory follows the [Feature-Sliced Design (FSD)](https://feature-sliced.design/) methodology for scalable architecture.

## Layer Structure

Layers are ordered from highest to lowest priority:

```
src/
├── app/        # Application initialization, providers, global styles
├── processes/  # Complex business processes spanning multiple features
├── pages/      # Page composition (routing targets)
├── widgets/    # Self-contained UI blocks with business value
├── features/   # User interactions with business value
├── entities/   # Business domain models
└── shared/     # Reusable infrastructure, UI kit, utilities
```

## Layer Rules

1. **Unidirectional dependency**: Higher layers can import from lower layers, never the reverse.
2. **Slicing**: Each layer is divided by business domain.
3. **Isolation**: Slices on the same layer should not import from each other.

## Dependency Flow

```
app → processes → pages → widgets → features → entities → shared
 ↓                                                             ↑
 └──────────────────── (no circular deps) ────────────────────┘
```

## Layer Descriptions

### `app/`
Application entry point and global configuration.
- Router setup
- Theme provider
- Authentication provider
- Global styles
- Environment configuration

### `processes/`
Complex workflows that span multiple features.
- User onboarding
- Media library synchronization
- Plugin installation wizard
- Update process

### `pages/`
Complete pages composed from widgets and features.
- Player page
- Library page
- Settings page
- Plugin marketplace page

### `widgets/`
Larger UI components with business logic.
- Media player controls
- Playlist panel
- Equalizer widget
- Subtitle overlay
- Media information panel

### `features/`
User interactions that deliver business value.
- Play/pause media
- Add to playlist
- Adjust volume
- Toggle subtitles
- Install plugin

### `entities/`
Business domain models and operations.
- Media item
- Playlist
- Plugin
- User preferences
- Playback state

### `shared/`
Infrastructure and reusable utilities.
- UI kit (buttons, inputs, modals)
- API client
- Storage abstractions
- Utilities and helpers
- Type definitions
- Constants

## Naming Conventions

```
layer/
├── slice/           # Business domain
│   ├── index.ts     # Public API
│   ├── model/       # Business logic
│   ├── ui/          # Components
│   └── api/         # External interactions
```

## Import Rules

```typescript
// ✅ Correct - importing from lower layers
import { Button } from 'shared/ui';
import { usePlayback } from 'features/playback';

// ❌ Wrong - importing from higher layers
import { PlayerPage } from 'pages/player';  // From widget
```

## Migration Status

| Layer | Status | Notes |
|-------|--------|-------|
| shared | ✅ Done | UI kit and utilities extracted |
| entities | 🔄 In Progress | Core domain models |
| features | 🔄 In Progress | User interactions |
| widgets | 📅 Planned | Composition layer |
| pages | 📅 Planned | Page components |
| processes | 📅 Planned | Complex workflows |
| app | 📅 Planned | Entry point |

## Resources

- [Feature-Sliced Design Documentation](https://feature-sliced.design/)
- [FSD Methodology Guide](https://feature-sliced.design/docs/get-started/overview)