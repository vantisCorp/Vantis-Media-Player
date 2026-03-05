---
sidebar_position: 5
---

# Media Library

Vantis Media Player includes a powerful media library for organizing, managing, and discovering your media collection.

## Overview

The media library provides:

- **Automatic media scanning** and indexing
- **Smart collections** and playlists
- **Metadata management** and editing
- **Search and filtering** capabilities
- **Cloud integration** and backup

## Setting Up the Media Library

### Initial Setup

```toml
[library]
enabled = true
paths = ["/media", "~/Videos", "~/Music"]
auto_scan = true
scan_interval = 3600  # seconds

[library.metadata]
extract_metadata = true
download_posters = true
download_backdrops = true

[library.cache]
enabled = true
location = "~/.vantismedia/cache"
max_size = 10240  # MB
```

```javascript
// Initialize library
const library = new MediaLibrary({
    paths: ['/media', '~/Videos'],
    autoScan: true,
    extractMetadata: true
});

// Start library
await library.initialize();
```

## Adding Media

### Adding Folders

```javascript
// Add folder to library
await library.addFolder('/path/to/media', {
    recursive: true,
    autoScan: true
});

// Add multiple folders
await library.addFolders([
    '/path/to/videos',
    '/path/to/music',
    '/path/to/photos'
]);
```

```bash
# Add folder via command line
vantismedia library add /path/to/media

# Add with options
vantismedia library add --recursive /path/to/media
```

### Adding Individual Files

```javascript
// Add single file
await library.addFile('/path/to/video.mp4');

// Add multiple files
await library.addFiles([
    '/path/to/video1.mp4',
    '/path/to/video2.mp4',
    '/path/to/audio.mp3'
]);
```

## Scanning Media

### Manual Scan

```javascript
// Scan all folders
await library.scan();

// Scan specific folder
await library.scanFolder('/path/to/media');

// Scan with progress callback
await library.scan({
    onProgress: (progress) => {
        console.log(`${progress.processed}/${progress.total} files`);
    }
});
```

### Scan Options

```javascript
// Scan with filters
await library.scan({
    recursive: true,
    includeFormats: ['mp4', 'mkv', 'mp3', 'flac'],
    excludeFormats: ['tmp', 'log'],
    minSize: 1024 * 1024,  // 1MB
    maxSize: 10 * 1024 * 1024 * 1024  // 10GB
});

// Scan and extract metadata
await library.scan({
    extractMetadata: true,
    downloadThumbnails: true,
    generatePreviews: true
});
```

```bash
# Manual scan
vantismedia library scan

# Scan specific folder
vantismedia library scan /path/to/media

# Scan with options
vantismedia library scan --recursive --metadata
```

## Browsing the Library

### Browse by Type

```javascript
// Get all videos
const videos = await library.getVideos();

// Get all audio
const audio = await library.getAudio();

// Get images
const images = await library.getImages();

// Get all media
const all = await library.getAll();
```

### Browse by Folder

```javascript
// Browse folder structure
const folders = await library.getFolders();
const children = await library.getFolderContents('/media/videos');

// Get folder with media
const folder = await library.getFolder('/media/videos');
console.log(folder);
// {
//   path: '/media/videos',
//   name: 'videos',
//   itemCount: 150,
//   totalSize: 10737418240,
//   media: [...]
// }
```

## Search and Filter

### Search

```javascript
// Search by name
const results = await library.search('matrix');

// Advanced search
const results = await library.search({
    query: 'matrix',
    type: 'video',
    year: { min: 2000, max: 2024 },
    genres: ['action', 'sci-fi'],
    rating: { min: 7 }
});
```

### Filter

```javascript
// Filter by type
const videos = await library.filter({ type: 'video' });

// Filter by date
const recent = await library.filter({
    dateAdded: { after: new Date('2024-01-01') }
});

// Filter by duration
const shortVideos = await library.filter({
    duration: { min: 0, max: 600 }  // 0-10 minutes
});

// Filter by size
const largeFiles = await library.filter({
    size: { min: 1024 * 1024 * 1024 }  // > 1GB
});

// Custom filter
const custom = await library.filter((item) => {
    return item.rating >= 8 && item.year >= 2020;
});
```

### Sorting

```javascript
// Sort by name
const sorted = await library.sort('name', 'asc');

// Sort by date added
const recent = await library.sort('dateAdded', 'desc');

// Sort by duration
const longest = await library.sort('duration', 'desc');

// Sort by rating
const topRated = await library.sort('rating', 'desc');

// Custom sort
const custom = await library.sort((a, b) => {
    return b.views - a.views;
});
```

## Metadata Management

### Reading Metadata

```javascript
// Get item metadata
const item = await library.getItem('/path/to/video.mp4');
console.log(item.metadata);
// {
//   title: 'Video Title',
//   year: 2024,
//   genre: ['Action', 'Sci-Fi'],
//   director: 'Director Name',
//   actors: ['Actor 1', 'Actor 2'],
//   duration: 7200,
//   rating: 8.5,
//   resolution: '1920x1080',
//   codec: 'h264',
//   bitrate: 5000000
// }
```

### Editing Metadata

```javascript
// Update metadata
await library.updateMetadata('/path/to/video.mp4', {
    title: 'New Title',
    year: 2024,
    genre: ['Action', 'Adventure'],
    rating: 9.0,
    customField: 'Custom Value'
});

// Update multiple items
await library.updateMetadata(items, {
    genre: ['Action'],
    customField: 'value'
});
```

### Metadata Sources

```javascript
// Download metadata from online sources
await library.fetchMetadata('/path/to/video.mp4', {
    sources: ['tmdb', 'imdb', 'omdb'],
    autoSelect: true
});

// Search and match
const matches = await library.searchMetadata('The Matrix 1999');
await library.applyMetadata('/path/to/video.mp4', matches[0]);
```

## Collections

### Smart Collections

```javascript
// Create smart collection
const collection = await library.createCollection({
    name: 'Top Rated Movies',
    type: 'smart',
    filters: {
        type: 'video',
        rating: { min: 8 },
        year: { min: 2000 }
    },
    autoUpdate: true
});

// Get collection items
const items = await library.getCollectionItems(collection.id);
```

### Manual Collections

```javascript
// Create manual collection
const collection = await library.createCollection({
    name: 'My Favorites',
    type: 'manual'
});

// Add items to collection
await library.addToCollection(collection.id, [
    '/path/to/video1.mp4',
    '/path/to/video2.mp4'
]);

// Remove from collection
await library.removeFromCollection(collection.id, '/path/to/video1.mp4');
```

### Predefined Collections

```javascript
// Get predefined collections
const collections = await library.getCollections();

// Common collections:
// - Recently Added
// - Recently Played
// - Most Played
// - Highest Rated
// - Unwatched
// - In Progress

// Get items from predefined collection
const recent = await library.getCollectionItems('recently-added');
```

## Thumbnails and Previews

### Generating Thumbnails

```javascript
// Generate thumbnails
await library.generateThumbnails('/path/to/video.mp4', {
    count: 10,
    quality: 'high',
    format: 'jpg'
});

// Generate for all items
await library.generateAllThumbnails({
    format: 'webp',
    quality: 80
});
```

### Previews

```javascript
// Generate video preview
await library.generatePreview('/path/to/video.mp4', {
    duration: 30,  // seconds
    quality: 'medium'
});

// Generate GIF preview
await library.generateGifPreview('/path/to/video.mp4', {
    duration: 5,
    fps: 10,
    size: 320
});
```

## Cloud Integration

### Cloud Storage

```javascript
// Configure cloud storage
await library.configureCloud({
    provider: 'dropbox',  // dropbox, google-drive, onedrive
    accessToken: 'your-token',
    syncPath: '/vantis-media'
});

// Sync to cloud
await library.syncToCloud();

// Sync from cloud
await library.syncFromCloud();
```

### Backup Library

```javascript
// Export library database
await library.exportDatabase('backup.json');

// Import library database
await library.importDatabase('backup.json');

// Incremental backup
await library.backup({
    destination: '/backup/vantis',
    incremental: true,
    compression: true
});
```

## Statistics

### Library Statistics

```javascript
// Get library stats
const stats = await library.getStatistics();
console.log(stats);

// Example output:
{
    totalItems: 1250,
    totalVideos: 800,
    totalAudio: 400,
    totalImages: 50,
    totalSize: 536870912000,  // bytes
    totalDuration: 4320000,  // seconds
    averageRating: 7.5,
    mostCommonFormats: {
        mp4: 600,
        mkv: 200,
        mp3: 300,
        flac: 100
    },
    storageUsage: {
        used: 536870912000,
        available: 1073741824000
    }
}
```

### Item Statistics

```javascript
// Get item statistics
const stats = await library.getItemStatistics('/path/to/video.mp4');
console.log(stats);
// {
//   playCount: 25,
//   lastPlayed: '2024-03-04T10:30:00Z',
//   dateAdded: '2024-01-15T08:00:00Z',
//   averageRating: 8.5,
//   views: 30
// }
```

## API Integration

### REST API

The media library provides a REST API for remote access:

```javascript
// Start API server
library.startApiServer({
    port: 8080,
    authentication: true,
    apiKey: 'your-api-key'
});

// Access library via HTTP
const response = await fetch('http://localhost:8080/api/library/items');
const items = await response.json();
```

### GraphQL API

```javascript
// GraphQL query
const query = `
  query GetVideos($genre: String!) {
    videos(genre: $genre) {
      title
      year
      rating
      duration
    }
  }
`;

const result = await library.graphql(query, { genre: 'action' });
```

## Performance Optimization

### Caching

```toml
[library.cache]
enabled = true
location = "~/.vantismedia/cache"
thumbnail_cache_size = 512  # MB
metadata_cache_size = 128   # MB
preview_cache_size = 2048   # MB
```

### Indexing

```javascript
// Rebuild index
await library.rebuildIndex();

// Optimize index
await library.optimizeIndex();
```

### Lazy Loading

```javascript
// Load items in batches
const items = await library.getItems({
    page: 1,
    pageSize: 50,
    loadMetadata: false  // Load on demand
});
```

## Troubleshooting

### Media Not Detected

```bash
# Check if paths are accessible
vantismedia library check-paths

# Rescan specific folder
vantismedia library scan --force /path/to/media
```

### Metadata Issues

```javascript
// Clear and re-extract metadata
await library.clearMetadata('/path/to/video.mp4');
await library.extractMetadata('/path/to/video.mp4');
```

### Performance Issues

```javascript
// Optimize library
await library.optimize();

// Clear cache
await library.clearCache();
```

## Next Steps

- **[Advanced Features](../advanced-features/)** - Explore advanced functionality
- **[API Reference](../api/)** - Complete API documentation
- **[Development](../development/)** - Develop custom extensions

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)