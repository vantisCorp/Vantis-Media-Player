# Pages Layer

Complete pages composed from widgets and features.

## Purpose

- Define page layouts
- Compose widgets into pages
- Handle routing
- Page-level data loading

## Structure

```
pages/
├── index.ts
├── player/            # Main player page
│   ├── index.ts
│   ├── ui/
│   └── model.ts
├── library/           # Media library page
├── settings/          # Settings page
├── plugins/           # Plugin marketplace
└── not-found/         # 404 page
```

## Pages

### `player`
Main media player interface:
- Video display area
- Player controls widget
- Playlist panel
- Subtitle overlay

### `library`
Media library management:
- Media grid/list
- Search and filters
- Import/export
- Metadata editing

### `settings`
Application settings:
- Audio settings
- Video settings
- Plugin management
- About section

### `plugins`
Plugin marketplace:
- Browse plugins
- Search and filter
- Install/uninstall
- Plugin details

### `not-found`
404 error page for unknown routes.

## Rules

1. Pages are routing targets
2. Pages compose widgets and features
3. Page handles its own data loading
4. Minimal logic in pages

## Example

```typescript
// pages/player/ui/PlayerPage.tsx
import { PlayerControls } from 'widgets/player-controls';
import { PlaylistPanel } from 'widgets/playlist-panel';
import { SubtitleOverlay } from 'widgets/subtitle-overlay';

export const PlayerPage = () => {
  return (
    <div className="player-page">
      <VideoDisplay />
      <PlayerControls />
      <PlaylistPanel />
      <SubtitleOverlay />
    </div>
  );
};
```