# Entities Layer

Business domain models representing core concepts of the application.

## Purpose

- Define business entities
- Implement entity-specific logic
- Provide data access methods
- Handle entity state

## Structure

```
entities/
├── index.ts
├── media/           # Media file entity
│   ├── index.ts
│   ├── model.ts     # State and logic
│   ├── types.ts     # TypeScript types
│   └── ui/          # Entity components
├── playlist/        # Playlist entity
├── plugin/          # Plugin entity
├── user/            # User preferences
└── playback/        # Playback state entity
```

## Entities

### `media`
- Media file representation
- Metadata handling
- Format detection
- Thumbnail generation

### `playlist`
- Playlist management
- Track ordering
- Import/export

### `plugin`
- Plugin information
- Installation status
- Configuration

### `user`
- User preferences
- Saved settings
- Profile data

### `playback`
- Current playback state
- Queue management
- History

## Rules

1. Each entity is a separate slice
2. Entities should not depend on each other
3. Business logic belongs here
4. Expose clean public API

## Example

```typescript
// entities/media/index.ts
export { useMedia } from './model';
export { MediaCard } from './ui';
export type { Media, MediaFormat } from './types';
```