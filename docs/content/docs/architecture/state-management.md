---
sidebar_position: 4
---

# State Management

Vantis Media Player uses a centralized state management system to track and manage all player state. This document describes the state management architecture and patterns.

## State Management Overview

The state management system follows the Flux pattern, ensuring predictable state updates and making it easy to track changes over time.

```
Actions → Dispatcher → Store → Views
                    ↓               ↑
                 State ◄───────────┘
```

## Core Concepts

### State

State is the single source of truth for the player. It represents the entire state of the player at any given time.

```typescript
interface PlayerState {
  // Playback state
  playing: boolean;
  paused: boolean;
  ended: boolean;
  
  // Time
  currentTime: number;
  duration: number;
  playbackRate: number;
  
  // Volume
  volume: number;
  muted: boolean;
  
  // Fullscreen
  fullscreen: boolean;
  
  // Loading
  loading: boolean;
  buffered: number;
  
  // Error
  error: PlayerError | null;
  
  // Media
  mediaSource: MediaSource | null;
  
  // Tracks
  videoTrack: string | null;
  audioTrack: string | null;
  subtitleTrack: string | null;
  
  // Quality
  quality: VideoQuality | null;
  
  // Metadata
  metadata: MediaMetadata | null;
}
```

### Actions

Actions describe what happened in the application. They are plain JavaScript objects with a `type` field.

```typescript
interface Action {
  type: string;
  payload?: any;
}

// Example actions
const playAction: Action = { type: 'PLAY' };
const seekAction: Action = { type: 'SEEK', payload: 120 };
const volumeAction: Action = { type: 'SET_VOLUME', payload: 0.8 };
```

### Reducer

A reducer is a pure function that takes the previous state and an action, and returns the next state.

```typescript
type Reducer<S, A> = (state: S, action: A) => S;

function playerReducer(state: PlayerState, action: Action): PlayerState {
  switch (action.type) {
    case 'PLAY':
      return { ...state, playing: true, paused: false };
    
    case 'PAUSE':
      return { ...state, playing: false, paused: true };
    
    case 'SEEK':
      return { ...state, currentTime: action.payload };
    
    case 'SET_VOLUME':
      return { ...state, volume: action.payload };
    
    default:
      return state;
  }
}
```

### Store

The store holds the entire state tree and provides methods to access and update it.

```typescript
class Store<S, A> {
  private state: S;
  private reducer: Reducer<S, A>;
  private listeners: Set<(state: S) => void> = new Set();
  
  constructor(reducer: Reducer<S, A>, initialState: S) {
    this.reducer = reducer;
    this.state = initialState;
  }
  
  getState(): S {
    return this.state;
  }
  
  dispatch(action: A): void {
    this.state = this.reducer(this.state, action);
    this.notify();
  }
  
  subscribe(listener: (state: S) => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  
  private notify(): void {
    this.listeners.forEach(listener => listener(this.state));
  }
}
```

## Player State Store

Vantis Media Player provides a built-in state store for managing player state.

### Initial State

```typescript
const initialState: PlayerState = {
  playing: false,
  paused: false,
  ended: false,
  currentTime: 0,
  duration: 0,
  playbackRate: 1.0,
  volume: 1.0,
  muted: false,
  fullscreen: false,
  loading: false,
  buffered: 0,
  error: null,
  mediaSource: null,
  videoTrack: null,
  audioTrack: null,
  subtitleTrack: null,
  quality: null,
  metadata: null
};
```

### Reducer Implementation

```typescript
function playerReducer(state: PlayerState, action: Action): PlayerState {
  switch (action.type) {
    // Playback
    case 'SET_PLAYING':
      return { 
        ...state, 
        playing: action.payload, 
        paused: !action.payload 
      };
    
    case 'SET_ENDED':
      return { 
        ...state, 
        ended: action.payload, 
        playing: false 
      };
    
    // Time
    case 'SET_CURRENT_TIME':
      return { ...state, currentTime: action.payload };
    
    case 'SET_DURATION':
      return { ...state, duration: action.payload };
    
    case 'SET_PLAYBACK_RATE':
      return { ...state, playbackRate: action.payload };
    
    // Volume
    case 'SET_VOLUME':
      return { ...state, volume: action.payload, muted: false };
    
    case 'SET_MUTED':
      return { ...state, muted: action.payload };
    
    // Fullscreen
    case 'SET_FULLSCREEN':
      return { ...state, fullscreen: action.payload };
    
    // Loading
    case 'SET_LOADING':
      return { ...state, loading: action.payload };
    
    case 'SET_BUFFERED':
      return { ...state, buffered: action.payload };
    
    // Error
    case 'SET_ERROR':
      return { ...state, error: action.payload };
    
    case 'CLEAR_ERROR':
      return { ...state, error: null };
    
    // Media
    case 'SET_MEDIA_SOURCE':
      return { ...state, mediaSource: action.payload };
    
    // Tracks
    case 'SET_VIDEO_TRACK':
      return { ...state, videoTrack: action.payload };
    
    case 'SET_AUDIO_TRACK':
      return { ...state, audioTrack: action.payload };
    
    case 'SET_SUBTITLE_TRACK':
      return { ...state, subtitleTrack: action.payload };
    
    // Quality
    case 'SET_QUALITY':
      return { ...state, quality: action.payload };
    
    // Metadata
    case 'SET_METADATA':
      return { ...state, metadata: action.payload };
    
    default:
      return state;
  }
}
```

### Store Usage

```typescript
// Create store
const store = new Store(playerReducer, initialState);

// Get state
const state = store.getState();

// Dispatch actions
store.dispatch({ type: 'PLAY' });
store.dispatch({ type: 'SET_VOLUME', payload: 0.8 });

// Subscribe to state changes
const unsubscribe = store.subscribe((state) => {
  console.log('State changed:', state);
});

// Unsubscribe
unsubscribe();
```

## Action Creators

Action creators are functions that create action objects. They help maintain consistency and prevent typos.

```typescript
// Playback
export const setPlaying = (playing: boolean): Action => ({
  type: 'SET_PLAYING',
  payload: playing
});

export const play = (): Action => setPlaying(true);
export const pause = (): Action => setPlaying(false);

export const setEnded = (ended: boolean): Action => ({
  type: 'SET_ENDED',
  payload: ended
});

// Time
export const setCurrentTime = (time: number): Action => ({
  type: 'SET_CURRENT_TIME',
  payload: time
});

export const setDuration = (duration: number): Action => ({
  type: 'SET_DURATION',
  payload: duration
});

export const setPlaybackRate = (rate: number): Action => ({
  type: 'SET_PLAYBACK_RATE',
  payload: rate
});

// Volume
export const setVolume = (volume: number): Action => ({
  type: 'SET_VOLUME',
  payload: volume
});

export const setMuted = (muted: boolean): Action => ({
  type: 'SET_MUTED',
  payload: muted
});

export const toggleMuted = (): Action => {
  const currentMuted = store.getState().muted;
  return setMuted(!currentMuted);
};

// Fullscreen
export const setFullscreen = (fullscreen: boolean): Action => ({
  type: 'SET_FULLSCREEN',
  payload: fullscreen
});

// Loading
export const setLoading = (loading: boolean): Action => ({
  type: 'SET_LOADING',
  payload: loading
});

export const setBuffered = (buffered: number): Action => ({
  type: 'SET_BUFFERED',
  payload: buffered
});

// Error
export const setError = (error: PlayerError): Action => ({
  type: 'SET_ERROR',
  payload: error
});

export const clearError = (): Action => ({
  type: 'CLEAR_ERROR'
});

// Media
export const setMediaSource = (source: MediaSource): Action => ({
  type: 'SET_MEDIA_SOURCE',
  payload: source
});

// Tracks
export const setVideoTrack = (trackId: string): Action => ({
  type: 'SET_VIDEO_TRACK',
  payload: trackId
});

export const setAudioTrack = (trackId: string): Action => ({
  type: 'SET_AUDIO_TRACK',
  payload: trackId
});

export const setSubtitleTrack = (trackId: string | null): Action => ({
  type: 'SET_SUBTITLE_TRACK',
  payload: trackId
});

// Quality
export const setQuality = (quality: VideoQuality): Action => ({
  type: 'SET_QUALITY',
  payload: quality
});

// Metadata
export const setMetadata = (metadata: MediaMetadata): Action => ({
  type: 'SET_METADATA',
  payload: metadata
});
```

## Selectors

Selectors are functions that compute derived data from the store state. They help avoid duplicate logic and can be memoized for performance.

```typescript
// Basic selectors
export const selectPlaying = (state: PlayerState): boolean => 
  state.playing;

export const selectPaused = (state: PlayerState): boolean => 
  state.paused;

export const selectEnded = (state: PlayerState): boolean => 
  state.ended;

export const selectCurrentTime = (state: PlayerState): number => 
  state.currentTime;

export const selectDuration = (state: PlayerState): number => 
  state.duration;

export const selectVolume = (state: PlayerState): number => 
  state.volume;

export const selectMuted = (state: PlayerState): boolean => 
  state.muted;

// Computed selectors
export const selectProgress = (state: PlayerState): number => {
  if (state.duration === 0) return 0;
  return (state.currentTime / state.duration) * 100;
};

export const selectBufferProgress = (state: PlayerState): number => {
  if (state.duration === 0) return 0;
  return (state.buffered / state.duration) * 100;
};

export const selectTimeRemaining = (state: PlayerState): number => {
  return Math.max(0, state.duration - state.currentTime);
};

export const selectFormattedTime = (state: PlayerState) => {
  return {
    current: formatTime(state.currentTime),
    duration: formatTime(state.duration),
    remaining: formatTime(selectTimeRemaining(state))
  };
};

export const selectHasError = (state: PlayerState): boolean => 
  state.error !== null;

export const selectIsLoading = (state: PlayerState): boolean => 
  state.loading;

export const selectCanPlay = (state: PlayerState): boolean => 
  state.mediaSource !== null && 
  !state.loading && 
  !state.error;

export const selectCanPause = (state: PlayerState): boolean => 
  state.playing && 
  !state.loading;

// Helper function
function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}
```

## Middleware

Middleware provides a third-party extension point between dispatching an action and the moment it reaches the reducer.

```typescript
type Middleware<S, A> = (
  store: Store<S, A>
) => (next: (action: A) => void) => (action: A) => void;

// Logging middleware
const loggerMiddleware: Middleware<PlayerState, Action> = (store) => 
  (next) => (action) => {
    console.log('Dispatching:', action);
    const prevState = store.getState();
    const result = next(action);
    const nextState = store.getState();
    console.log('Next state:', nextState);
    return result;
  };

// Error handling middleware
const errorMiddleware: Middleware<PlayerState, Action> = (store) => 
  (next) => (action) => {
    try {
      return next(action);
    } catch (error) {
      console.error('Error in reducer:', error);
      store.dispatch(setError(error));
      return;
    }
  };

// Apply middleware
function applyMiddleware<S, A>(
  ...middlewares: Middleware<S, A>[]
): (reducer: Reducer<S, A>, initialState: S) => Store<S, A> {
  return (reducer, initialState) => {
    const store = new Store(reducer, initialState);
    
    let dispatch = (action: A) => {
      return store.dispatch(action);
    };
    
    const middlewareAPI = {
      getState: store.getState.bind(store),
      dispatch: (action: A) => dispatch(action)
    };
    
    const chain = middlewares.map(middleware => middleware(middlewareAPI));
    dispatch = compose(...chain)(store.dispatch.bind(store));
    
    store.dispatch = dispatch;
    
    return store;
  };
}

function compose<T>(...funcs: ((x: T) => T)[]): (x: T) => {
  if (funcs.length === 0) return (x) => x;
  if (funcs.length === 1) return funcs[0];
  return funcs.reduce((a, b) => (x) => a(b(x)));
}

// Create store with middleware
const store = applyMiddleware(
  loggerMiddleware,
  errorMiddleware
)(playerReducer, initialState);
```

## State Persistence

State can be persisted to localStorage for recovery across sessions.

```typescript
class PersistentStore<S, A> extends Store<S, A> {
  private storageKey: string;
  
  constructor(
    reducer: Reducer<S, A>, 
    initialState: S, 
    storageKey: string
  ) {
    super(reducer, initialState);
    this.storageKey = storageKey;
    this.load();
  }
  
  dispatch(action: A): void {
    super.dispatch(action);
    this.save();
  }
  
  private save(): void {
    const state = this.getState();
    localStorage.setItem(this.storageKey, JSON.stringify(state));
  }
  
  private load(): void {
    const saved = localStorage.getItem(this.storageKey);
    if (saved) {
      try {
        const state = JSON.parse(saved);
        this.state = state;
      } catch (error) {
        console.error('Failed to load state:', error);
      }
    }
  }
}

// Usage
const store = new PersistentStore(
  playerReducer,
  initialState,
  'vantis-player-state'
);
```

## State Time Travel

The state management system can track state changes for debugging and undo/redo functionality.

```typescript
class TimeTravelStore<S, A> extends Store<S, A> {
  private history: S[] = [];
  private future: S[] = [];
  private maxHistory: number = 50;
  
  dispatch(action: A): void {
    this.history.push(this.getState());
    if (this.history.length > this.maxHistory) {
      this.history.shift();
    }
    this.future = [];
    super.dispatch(action);
  }
  
  undo(): void {
    if (this.history.length === 0) return;
    
    const previous = this.history.pop();
    this.future.push(this.getState());
    this.state = previous;
    this.notify();
  }
  
  redo(): void {
    if (this.future.length === 0) return;
    
    const next = this.future.pop();
    this.history.push(this.getState());
    this.state = next;
    this.notify();
  }
  
  canUndo(): boolean {
    return this.history.length > 0;
  }
  
  canRedo(): boolean {
    return this.future.length > 0;
  }
}
```

## Best Practices

1. **Immutability**: Always return new state objects, never mutate existing state
2. **Pure reducers**: Reducers should be pure functions with no side effects
3. **Action creators**: Use action creators for consistency
4. **Selectors**: Use selectors for computed state
5. **Middleware**: Use middleware for cross-cutting concerns
6. **Type safety**: Use TypeScript for type safety
7. **Testing**: Test reducers and selectors in isolation

## Related Documentation

- [Overview](./overview) - High-level architecture
- [Components](./components) - Component details
- [Event System](./event-system) - Event system details
- [Plugin System](./plugin-system) - Plugin architecture