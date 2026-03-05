---
sidebar_position: 4
---

# Playlists

Vantis Media Player provides powerful playlist management capabilities for organizing and playing multiple media files.

## Playlist Formats

Vantis Media Player supports multiple playlist formats:

### M3U Format

```m3u
#EXTM3U
#EXTINF:180,Artist - Song Title
/path/to/song1.mp3
#EXTINF:240,Another Artist - Another Song
/path/to/song2.mp3
/path/to/video.mp4
```

### PLS Format

```pls
[playlist]
NumberOfEntries=2
File1=/path/to/song1.mp3
Title1=Artist - Song Title
Length1=180
File2=/path/to/song2.mp3
Title2=Another Artist - Another Song
Length2=240
```

### ASX Format

```asx
<asx version="3.0">
  <entry>
    <title>Song Title</title>
    <ref href="/path/to/song.mp3"/>
  </entry>
</asx>
```

## Loading Playlists

### From File

```javascript
// Load playlist from file
await player.loadPlaylist('playlist.m3u');

// Load and auto-play
await player.loadPlaylist('playlist.m3u', {
    autoplay: true,
    shuffle: false
});
```

```bash
# Play playlist
vantismedia playlist.m3u

# Play with shuffle
vantismedia --shuffle playlist.m3u
```

### Creating Playlists Programmatically

```javascript
// Create playlist
const playlist = new Playlist();

// Add items
playlist.add({
    url: 'video1.mp4',
    title: 'Video 1',
    duration: 3600
});

playlist.add({
    url: 'video2.mp4',
    title: 'Video 2',
    duration: 2400
});

// Load playlist
player.setPlaylist(playlist);
```

## Playlist Management

### Adding Items

```javascript
// Add single item
player.addToPlaylist('video.mp4', {
    title: 'My Video',
    duration: 3600
});

// Add multiple items
player.addToPlaylist([
    { url: 'video1.mp4', title: 'Video 1' },
    { url: 'video2.mp4', title: 'Video 2' },
    { url: 'video3.mp4', title: 'Video 3' }
]);

// Add from directory
player.addDirectoryToPlaylist('/path/to/media', {
    recursive: true,
    formats: ['mp4', 'mkv', 'mp3']
});
```

### Removing Items

```javascript
// Remove by index
player.removeFromPlaylist(0);

// Remove current item
player.removeCurrentItem();

// Remove all
player.clearPlaylist();
```

### Reordering Items

```javascript
// Move item to new position
player.movePlaylistItem(0, 5);  // Move item 0 to position 5

// Swap items
player.swapPlaylistItems(0, 1);
```

## Playlist Navigation

### Basic Navigation

```javascript
// Play next item
player.next();

// Play previous item
player.previous();

// Play specific item
player.playItem(3);

// Get current item index
const currentIndex = player.getCurrentPlaylistIndex();
```

### Keyboard Shortcuts

| Action | Windows/Linux | macOS |
|--------|--------------|-------|
| Next Track | `Ctrl+→` | `Cmd+→` |
| Previous Track | `Ctrl+←` | `Cmd+←` |
| First Track | `Ctrl+Home` | `Cmd+Home` |
| Last Track | `Ctrl+End` | `Cmd+End` |

## Playlist Modes

### Loop Mode

```javascript
// Loop current item
player.setLoop('item');

// Loop entire playlist
player.setLoop('playlist');

// No loop
player.setLoop('none');

// Get current loop mode
const loopMode = player.getLoop();
```

### Shuffle Mode

```javascript
// Enable shuffle
player.setShuffle(true);

// Disable shuffle
player.setShuffle(false);

// Get shuffle status
const isShuffled = player.isShuffled();
```

### Repeat Mode

```javascript
// Repeat current item
player.setRepeat('one');

// Repeat all
player.setRepeat('all');

// No repeat
player.setRepeat('none');
```

## Playlist Information

### Getting Playlist Info

```javascript
// Get entire playlist
const playlist = player.getPlaylist();
console.log(playlist);

// Get playlist length
const length = player.getPlaylistLength();

// Get current item
const currentItem = player.getCurrentItem();

// Get item at index
const item = player.getPlaylistItem(2);
```

### Item Metadata

```javascript
// Get item metadata
const item = player.getPlaylistItem(0);
console.log(item);
// {
//   url: 'video.mp4',
//   title: 'Video 1',
//   duration: 3600,
//   artist: 'Artist',
//   album: 'Album',
//   metadata: { ... }
// }
```

## Saving Playlists

### Export Playlist

```javascript
// Export to M3U
await player.savePlaylist('playlist.m3u', 'm3u');

// Export to PLS
await player.savePlaylist('playlist.pls', 'pls');

// Export to custom format
const playlist = player.getPlaylist();
const json = JSON.stringify(playlist, null, 2);
await writeFile('playlist.json', json);
```

### Auto-save

```toml
[player]
save_playlist = true
playlist_location = "~/.vantismedia/playlists"
```

```javascript
// Enable auto-save
player.setAutoSavePlaylist(true);
```

## Smart Playlists

### Create Smart Playlist

```javascript
// Create smart playlist based on criteria
const smartPlaylist = player.createSmartPlaylist({
    genre: 'rock',
    year: { min: 2000, max: 2024 },
    rating: { min: 4 },
    duration: { min: 180, max: 600 }
});

// Load smart playlist
player.setPlaylist(smartPlaylist);
```

### Dynamic Playlists

```javascript
// Create dynamic playlist from directory
const dynamicPlaylist = player.createDynamicPlaylist({
    directory: '/path/to/music',
    recursive: true,
    autoUpdate: true,
    filters: {
        format: ['mp3', 'flac'],
        bitrate: { min: 320 }
    }
});

player.setPlaylist(dynamicPlaylist);
```

## Queue Management

### Play Queue

```javascript
// Get queue
const queue = player.getQueue();

// Add to queue (play next)
player.addToQueue('video.mp4');

// Clear queue
player.clearQueue();

// Jump queue
player.jumpQueue(0);
```

### Up Next

```javascript
// Get upcoming items
const upcoming = player.getUpcoming(5);  // Next 5 items

// Get previously played
const history = player.getHistory(10);  // Last 10 items
```

## Playlist Sharing

### Export for Sharing

```javascript
// Export with absolute paths
await player.savePlaylist('playlist.m3u', 'm3u', {
    absolutePaths: true
});

// Export with relative paths
await player.savePlaylist('playlist.m3u', 'm3u', {
    relativePaths: true,
    basePath: '/media'
});

// Export with URLs
await player.savePlaylist('playlist.m3u', 'm3u', {
    useUrls: true
});
```

### Import Playlists

```javascript
// Import from URL
await player.loadPlaylist('https://example.com/playlist.m3u');

// Import from string
const m3uContent = `#EXTM3U\n#EXTINF:180,Song\n/path/to/song.mp3`;
await player.loadPlaylistFromContent(m3uContent, 'm3u');
```

## Playlist Events

### Event Listeners

```javascript
// Item changed
player.on('playlistItemChanged', (index, item) => {
    console.log(`Now playing: ${item.title}`);
});

// Playlist ended
player.on('playlistEnded', () => {
    console.log('Playlist finished');
});

// Item added
player.on('playlistItemAdded', (item, index) => {
    console.log(`Added: ${item.title}`);
});

// Item removed
player.on('playlistItemRemoved', (index, item) => {
    console.log(`Removed: ${item.title}`);
});

// Playlist shuffled
player.on('playlistShuffled', () => {
    console.log('Playlist shuffled');
});
```

## Playlist Synchronization

### Cloud Sync

```javascript
// Sync with cloud service
await player.syncPlaylist({
    provider: 'dropbox',
    accessToken: 'your-token',
    path: '/playlists/my-playlist.m3u'
});
```

### Multi-device Sync

```javascript
// Export playlist for sync
const playlist = player.getPlaylist();
const syncData = {
    playlist: playlist,
    timestamp: Date.now(),
    version: '1.0'
};

// Import on another device
await player.importPlaylist(syncData);
```

## Playlist Statistics

### Statistics

```javascript
// Get playlist statistics
const stats = player.getPlaylistStats();
console.log(stats);

// Example output:
{
    totalDuration: 43200,  // seconds
    totalSize: 2147483648, // bytes
    totalFiles: 50,
    formats: {
        mp4: 30,
        mp3: 15,
        flac: 5
    },
    averageDuration: 864,
    totalPlays: 1250
}
```

## Troubleshooting

### Playlist Won't Load

```bash
# Check playlist format
vantismedia --check-playlist playlist.m3u

# Validate playlist
vantismedia --validate-playlist playlist.m3u
```

### Items Not Found

```javascript
// Check if items exist
const playlist = player.getPlaylist();
playlist.forEach((item, index) => {
    if (!item.exists) {
        console.log(`Item ${index} not found: ${item.url}`);
    }
});
```

## Next Steps

- **[Media Library](./media-library)** - Organize your media collection
- **[Advanced Features](../advanced-features/)** - Explore advanced functionality
- **[API Reference](../api/)** - Complete API documentation

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)