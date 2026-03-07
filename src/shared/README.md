# Shared Layer

The shared layer contains reusable code without business logic.

## Purpose

- Provide reusable UI components
- Offer utility functions
- Define shared types and constants
- Abstract external services

## Structure

```
shared/
├── index.ts           # Public API
├── ui/                # UI components
│   ├── button/
│   ├── input/
│   ├── modal/
│   └── ...
├── api/               # API client and helpers
├── lib/               # Third-party library wrappers
├── utils/             # Utility functions
├── types/             # TypeScript definitions
├── constants/         # Shared constants
└── hooks/             # Generic React hooks
```

## Contents

### `ui/` - UI Kit
Design system components:
- Button, IconButton
- Input, Select, Checkbox
- Modal, Dialog, Drawer
- Slider, Progress
- Tooltip, Popover
- Icons

### `api/` - API Layer
- HTTP client configuration
- Request/response interceptors
- Error handling
- Authentication headers

### `lib/` - Library Wrappers
- Storage abstraction (localStorage, IndexedDB)
- Analytics integration
- Logging utilities

### `utils/` - Utilities
- String manipulation
- Date formatting
- File operations
- Number formatting
- Debounce/throttle

### `types/` - Types
- Common interfaces
- API response types
- Utility types

### `constants/` - Constants
- App configuration
- Default values
- Breakpoints
- Animation durations

## Rules

1. **No business logic** - This layer is generic
2. **Self-contained** - No dependencies on other layers
3. **Stable API** - Changes should be backward compatible
4. **Well-documented** - Each export needs documentation

## Usage

```typescript
// Import from the public API
import { Button, Input } from 'shared/ui';
import { formatDate } from 'shared/utils';
import { API_BASE_URL } from 'shared/constants';
```

## Example Component

```typescript
// shared/ui/button/index.tsx
export const Button = ({ children, variant = 'primary', ...props }) => {
  return (
    <button className={`btn btn-${variant}`} {...props}>
      {children}
    </button>
  );
};
```