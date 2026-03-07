# Widgets Layer

Self-contained UI blocks with business value.

## Purpose

- Compose features and entities into UI blocks
- Provide reusable widget components
- Handle widget-level state
- Implement widget-specific layouts

## Structure

```
widgets/
├── index.ts
├── player-controls/     # Main player controls widget
│   ├── index.ts
│   ├── ui/
│   └── model.ts
├── playlist-panel/      # Side panel with playlist
├── equalizer/           # Audio equalizer widget
├── subtitle-overlay/    # Subtitle display widget
├── media-info/          # Media information panel
└── navigation/          # App navigation widget
```

## Widgets

### `player-controls`
- Play/pause button
- Progress bar
- Volume controls
- Fullscreen toggle

### `playlist-panel`
- Track list
- Drag and drop reordering
- Search/filter

### `equalizer`
- Frequency bands
- Presets
- Visualization

### `subtitle-overlay`
- Subtitle rendering
- Position adjustment
- Style customization

### `media-info`
- Metadata display
- Codec information
- Stream selection

### `navigation`
- Main navigation
- Breadcrumbs
- Quick actions

## Rules

1. Widgets are composition units
2. Can use features and entities
3. Should be independently usable
4. Have their own state management

## Example

```typescript
// widgets/player-controls/index.ts
export { PlayerControls } from './ui';
export type { PlayerControlsProps } from './types';
```