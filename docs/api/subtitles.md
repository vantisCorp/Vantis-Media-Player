---
sidebar_position: 5
---

# Subtitles API

The Subtitles API provides methods for loading, managing, and customizing subtitles.

## Subtitle Track Management

### getSubtitleTracks()

Get all available subtitle tracks.

```javascript
const tracks = player.getSubtitleTracks();
// [
//   { id: 1, language: 'en', label: 'English', enabled: true },
//   { id: 2, language: 'es', label: 'Spanish', enabled: false },
//   { id: 3, language: 'fr', label: 'French', enabled: false }
// ]
```

### setSubtitleTrack()

Select a subtitle track.

```javascript
// Select by track ID
player.setSubtitleTrack(1);

// Select by language code
player.setSubtitleTrackByLanguage('en');

// Select by label
player.setSubtitleTrackByLabel('English');
```

### getCurrentSubtitleTrack()

Get the currently selected subtitle track.

```javascript
const track = player.getCurrentSubtitleTrack();
// { id: 1, language: 'en', label: 'English', enabled: true }
```

### disableSubtitles()

Disable all subtitles.

```javascript
player.disableSubtitles();
```

### areSubtitlesEnabled()

Check if subtitles are enabled.

```javascript
const enabled = player.areSubtitlesEnabled();
```

## Loading Subtitles

### loadSubtitles()

Load external subtitles.

```javascript
// Load from file
await player.loadSubtitles('subtitles.srt');

// Load from URL
await player.loadSubtitles('https://example.com/subs.srt');

// Load with options
await player.loadSubtitles('subtitles.srt', {
    language: 'en',
    label: 'English',
    default: true
});
```

### loadSubtitlesFromContent()

Load subtitles from text content.

```javascript
const srtContent = `1
00:00:01,000 --> 00:00:04,000
Hello, world!

2
00:00:05,000 --> 00:00:08,000
This is a test.`;

await player.loadSubtitlesFromContent(srtContent, 'srt', {
    language: 'en',
    label: 'English'
});
```

### unloadSubtitles()

Remove loaded subtitles.

```javascript
await player.unloadSubtitles(1);  // Remove by track ID
await player.unloadAllSubtitles();  // Remove all
```

## Subtitle Formatting

### Font Settings

```javascript
// Set font family
player.setSubtitleFont('Roboto');

// Set font size
player.setSubtitleFontSize(24);

// Set font weight
player.setSubtitleFontWeight('bold');  // 'normal', 'bold'

// Set font style
player.setSubtitleFontStyle('italic');  // 'normal', 'italic'
```

### Colors

```javascript
// Set text color
player.setSubtitleColor('#FFFFFF');

// Set background color
player.setSubtitleBackgroundColor('#000000');

// Set background opacity (0.0 to 1.0)
player.setSubtitleBackgroundOpacity(0.7);

// Set outline color
player.setSubtitleOutlineColor('#000000');

// Set outline width
player.setSubtitleOutlineWidth(2);
```

### Position and Layout

```javascript
// Set vertical position (0-100, from top)
player.setSubtitlePosition(90);  // Near bottom

// Set horizontal alignment
player.setSubtitleAlignment('center');  // 'left', 'center', 'right'

// Set margins
player.setSubtitleMargin({
    bottom: 50,
    left: 20,
    right: 20
});
```

### Advanced Styling

```javascript
// Enable shadow
player.setSubtitleShadow(true);
player.setSubtitleShadowColor('#000000');
player.setSubtitleShadowBlur(4);

// Set border radius
player.setSubtitleBorderRadius(8);

// Set padding
player.setSubtitlePadding({
    vertical: 10,
    horizontal: 20
});
```

### Reset Styling

```javascript
player.resetSubtitleStyle();
```

## Subtitle Sync

### setSubtitleDelay()

Adjust subtitle timing.

```javascript
// Delay subtitles by 500ms
player.setSubtitleDelay(500);

// Advance subtitles by 300ms
player.setSubtitleDelay(-300);

// Reset delay
player.setSubtitleDelay(0);
```

### getSubtitleDelay()

Get current subtitle delay.

```javascript
const delay = player.getSubtitleDelay();  // milliseconds
```

### adjustSubtitleSync()

Adjust sync incrementally.

```javascript
player.adjustSubtitleSync(100);   // Add 100ms delay
player.adjustSubtitleSync(-100);  // Remove 100ms delay
```

### setSubtitleAutoSync()

Enable auto-sync.

```javascript
player.setSubtitleAutoSync(true);
```

## Subtitle Content

### getCurrentSubtitleText()

Get the currently displayed subtitle text.

```javascript
const text = player.getCurrentSubtitleText();
// "Hello, world!"
```

### getSubtitleAtTime()

Get subtitle at specific time.

```javascript
const subtitle = player.getSubtitleAtTime(30);  // At 30 seconds
// { start: 28, end: 32, text: "Hello, world!" }
```

### getAllSubtitles()

Get all subtitle entries.

```javascript
const subtitles = player.getAllSubtitles();
// [
//   { start: 0, end: 4, text: "Hello, world!" },
//   { start: 5, end: 8, text: "This is a test." }
// ]
```

## Subtitle Editing

### addSubtitle()

Add a subtitle entry.

```javascript
player.addSubtitle({
    start: 10,
    end: 14,
    text: "New subtitle"
});
```

### updateSubtitle()

Update a subtitle entry.

```javascript
player.updateSubtitle(1, {
    text: "Updated subtitle text"
});
```

### removeSubtitle()

Remove a subtitle entry.

```javascript
player.removeSubtitle(1);  // Remove by index
```

### clearSubtitles()

Clear all subtitles.

```javascript
player.clearSubtitles();
```

## Subtitle Export

### exportSubtitles()

Export subtitles to a file format.

```javascript
// Export to SRT
const srt = player.exportSubtitles('srt');

// Export to WebVTT
const vtt = player.exportSubtitles('vtt');

// Export to ASS
const ass = player.exportSubtitles('ass');

// Download as file
const srt = player.exportSubtitles('srt');
downloadFile('subtitles.srt', srt);
```

### getSubtitlesAsJSON()

Get subtitles as JSON.

```javascript
const json = player.getSubtitlesAsJSON();
const subtitles = JSON.parse(json);
```

## Subtitle Search

### searchOnlineSubtitles()

Search for subtitles online.

```javascript
const results = await player.searchOnlineSubtitles({
    query: 'Movie Title 2024',
    language: 'en'
});
// [
//   { id: 1, title: 'Movie Title', language: 'en', downloads: 1000, rating: 8.5 },
//   { id: 2, title: 'Movie Title', language: 'en', downloads: 500, rating: 7.0 }
// ]
```

### downloadOnlineSubtitles()

Download subtitles from search results.

```javascript
await player.downloadOnlineSubtitles(result.id, {
    language: 'en',
    autoLoad: true
});
```

### findLocalSubtitles()

Find subtitles in local directories.

```javascript
const subtitles = await player.findLocalSubtitles('/path/to/video.mp4', {
    directories: ['./subtitles', './subs'],
    formats: ['srt', 'vtt', 'ass']
});
```

## Closed Captions

### setClosedCaptions()

Enable closed captions.

```javascript
player.setClosedCaptions(true);
```

### setCaptionService()

Set caption service for CEA-608/708.

```javascript
player.setCaptionService('CC1');  // CC1, CC2, CC3, CC4
```

### getCaptionServices()

Get available caption services.

```javascript
const services = player.getCaptionServices();
// ['CC1', 'CC2', 'CC3', 'CC4']
```

## Dual Subtitles

### setDualSubtitles()

Enable dual subtitles for language learning.

```javascript
player.setDualSubtitles(true);
player.setPrimarySubtitleLanguage('en');
player.setSecondarySubtitleLanguage('es');
```

### getDualSubtitleConfig()

Get dual subtitle configuration.

```javascript
const config = player.getDualSubtitleConfig();
// { enabled: true, primary: 'en', secondary: 'es' }
```

## Subtitle Settings

### setSubtitleConfig()

Set comprehensive subtitle configuration.

```javascript
player.setSubtitleConfig({
    enabled: true,
    font: {
        family: 'Roboto',
        size: 24,
        weight: 'normal',
        style: 'normal'
    },
    color: '#FFFFFF',
    backgroundColor: '#000000',
    backgroundOpacity: 0.7,
    outlineColor: '#000000',
    outlineWidth: 2,
    position: 90,
    alignment: 'center',
    margin: {
        bottom: 50,
        left: 20,
        right: 20
    }
});
```

### getSubtitleConfig()

Get current subtitle configuration.

```javascript
const config = player.getSubtitleConfig();
```

## Subtitle Events

```javascript
// Subtitle track changed
player.on('subtitletrackchange', (track) => {
    console.log('Subtitle track:', track.label);
});

// Subtitle displayed
player.on('subtitledisplay', (subtitle) => {
    console.log('Subtitle:', subtitle.text);
});

// Subtitle hidden
player.on('subtitlehide', () => {
    console.log('Subtitle hidden');
});

// Subtitles loaded
player.on('subtitlesloaded', (tracks) => {
    console.log('Subtitles loaded:', tracks.length);
});

// Subtitle sync changed
player.on('subtitlesyncchange', (delay) => {
    console.log('Subtitle delay:', delay);
});
```

## Supported Formats

### SRT Format

```srt
1
00:00:01,000 --> 00:00:04,000
Hello, world!

2
00:00:05,000 --> 00:00:08,000
This is a subtitle example.
```

### WebVTT Format

```vtt
WEBVTT

00:00:01.000 --> 00:00:04.000
Hello, world!

00:00:05.000 --> 00:00:08.000
This is a subtitle example.
```

### ASS Format

```ass
[Script Info]
ScriptType: v4.00+

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour
Style: Default,Arial,20,&H00FFFFFF

[Events]
Format: Layer, Start, End, Style, Text
Dialogue: 0,0:00:01.00,0:00:04.00,Default,Hello, world!
```

## Rust API

```rust
use vantismedia::subtitles::{SubtitleConfig, SubtitleStyle};

// Get tracks
let tracks = player.subtitle_tracks();

// Set track
player.set_subtitle_track(1)?;

// Load external subtitles
player.load_subtitles("subtitles.srt").await?;

// Configure style
let style = SubtitleStyle {
    font_family: "Roboto".to_string(),
    font_size: 24,
    color: "#FFFFFF".to_string(),
    background_color: "#000000".to_string(),
    background_opacity: 0.7,
};

player.set_subtitle_style(style)?;

// Subtitle delay
player.set_subtitle_delay(Duration::from_millis(500));

// Export subtitles
let srt = player.export_subtitles(SubtitleFormat::Srt)?;
```

## Next Steps

- **[Events API](./events)** - Event system
- **[Plugins API](./plugins)** - Plugin system
- **[Audio API](./audio)** - Audio features

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)