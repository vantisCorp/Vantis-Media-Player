# Features Layer

User interactions that deliver business value.

## Purpose

- Implement user actions
- Handle user interactions
- Coordinate entity changes
- Provide feature-specific UI

## Structure

```
features/
├── index.ts
├── playback/           # Play/pause/stop
│   ├── index.ts
│   ├── model.ts        # Feature logic
│   ├── ui/             # UI components
│   └── api/            # External calls
├── volume/             # Volume control
├── subtitles/          # Subtitle controls
├── playlist-add/       # Add to playlist
├── plugin-install/     # Install plugins
└── settings/           # Settings management
```

## Features

### `playback`
- Play/pause media
- Seek to position
- Speed control
- Loop mode

### `volume`
- Adjust volume
- Mute/unmute
- Audio device selection

### `subtitles`
- Toggle subtitles
- Select subtitle track
- Subtitle delay

### `playlist-add`
- Add media to playlist
- Remove from playlist
- Reorder playlist

### `plugin-install`
- Browse plugins
- Install/uninstall
- Configure plugins

## Rules

1. Each feature is a single user action
2. Features should be small and focused
3. Features can import from entities and shared
4. Features on the same layer should not import from each other

## Example

```typescript
// features/playback/index.ts
export { usePlayback } from './model';
export { PlayButton, PlaybackControls } from './ui';
```