---
sidebar_position: 3
---

# Subtitles and Captions

Vantis Media Player provides comprehensive subtitle and caption support with extensive customization options.

## Supported Subtitle Formats

Vantis Media Player supports the following subtitle formats:

### Text Formats

- **SRT** (SubRip Text)
- **VTT** (WebVTT)
- **ASS** (Advanced SubStation Alpha)
- **SSA** (SubStation Alpha)
- **SUB** (MicroDVD)
- **TXT** (Plain text)

### Embedded Formats

- **MKV** embedded subtitles
- **MP4** embedded subtitles
- **MPEG-TS** embedded subtitles

## Loading Subtitles

### External Subtitle Files

```javascript
// Load video with external subtitles
player.load('video.mp4', {
    subtitle: 'subtitles.srt'
});

// Load subtitles after video
await player.load('video.mp4');
player.loadSubtitles('subtitles.srt');
```

```bash
# Play with subtitles
vantismedia video.mp4 --subtitles subtitles.srt

# Specify subtitle language
vantismedia video.mkv --subtitle-lang en
```

### Embedded Subtitles

```javascript
// Get embedded subtitle tracks
const tracks = player.getSubtitleTracks();
console.log(tracks);

// Select embedded subtitle
player.setSubtitleTrack(1);
```

### Auto-detect Subtitles

```toml
[subtitle]
auto_load = true
search_paths = ["./subtitles", "~/.vantismedia/subtitles"]
```

## Subtitle Customization

### Font Settings

```javascript
// Set font
player.setSubtitleFont('Roboto');

// Set font size
player.setSubtitleFontSize(24);

// Set font weight
player.setSubtitleFontWeight('bold');

// Set font style
player.setSubtitleFontStyle('italic');
```

```toml
[subtitle]
font = "Roboto"
size = 24
weight = "normal"
style = "normal"
```

### Colors and Styling

```javascript
// Set text color
player.setSubtitleColor('#FFFFFF');

// Set background color
player.setSubtitleBackgroundColor('#000000');

// Set background opacity
player.setSubtitleBackgroundOpacity(0.7);

// Set outline color
player.setSubtitleOutlineColor('#000000');

// Set outline width
player.setSubtitleOutlineWidth(2);
```

### Position and Alignment

```javascript
// Set vertical position (0-100, from top)
player.setSubtitlePosition(90);

// Set horizontal alignment
player.setSubtitleAlignment('center');  // left, center, right

// Set margin
player.setSubtitleMargin({
    bottom: 50,
    left: 20,
    right: 20
});
```

### Advanced Styling

```javascript
// Enable text shadow
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

## Subtitle Languages

### Language Detection

```javascript
// Auto-detect subtitle language
player.setSubtitleLanguage('auto');

// Set specific language (ISO 639-1 code)
player.setSubtitleLanguage('en');
player.setSubtitleLanguage('pl');
player.setSubtitleLanguage('de');
```

### Multiple Languages

```javascript
// Get all available subtitle tracks with languages
const tracks = player.getSubtitleTracks();
tracks.forEach(track => {
    console.log(`Track ${track.index}: ${track.language} (${track.label})`);
});

// Example output:
// Track 0: English (English)
// Track 1: Polish (Polski)
// Track 2: German (Deutsch)
```

## Subtitle Search

### Online Subtitle Search

```javascript
// Search for subtitles online
const results = await player.searchSubtitles('Movie Title 2024');
console.log(results);

// Download and load subtitles
await player.downloadSubtitles(results[0].url);
```

### Local Subtitle Search

```javascript
// Search local directories for subtitles
const subtitles = await player.findSubtitles('video.mp4', {
    directories: ['./subtitles', './subs'],
    formats: ['srt', 'vtt', 'ass']
});
```

## Subtitle Editing

### Edit Subtitles

```javascript
// Get subtitle text
const text = player.getSubtitleText(10.5);  // at 10.5 seconds

// Set subtitle text
player.setSubtitleText({
    start: 10.0,
    end: 12.0,
    text: "Custom subtitle text"
});

// Shift subtitles
player.shiftSubtitles(2.0);  // shift by 2 seconds

// Scale subtitles
player.scaleSubtitles(1.1);  // scale timing by 10%
```

### Create Subtitles

```javascript
// Create new subtitle file
const subtitles = new Subtitles();

// Add subtitle entry
subtitles.add({
    start: 0,
    end: 5,
    text: "Hello, world!"
});

// Save to file
subtitles.save('subtitles.srt');
```

## Subtitle Sync

### Manual Sync Adjustment

```javascript
// Adjust subtitle delay (in milliseconds)
player.setSubtitleDelay(500);  // delay by 500ms
player.setSubtitleDelay(-250); // advance by 250ms
```

### Auto-sync

```javascript
// Enable auto-sync
player.setSubtitleAutoSync(true);

// Adjust sync incrementally
player.adjustSubtitleSync(100);  // +100ms
player.adjustSubtitleSync(-100); // -100ms
```

## Closed Captions (CC)

### CEA-608/708 Captions

```javascript
// Enable closed captions
player.setClosedCaptions(true);

// Set caption service
player.setCaptionService('CC1');  // CC1, CC2, CC3, CC4
```

### WebVTT Captions

```javascript
// Load WebVTT captions
player.loadCaptions('captions.vtt');
```

## Subtitle Display Modes

### Display Modes

```javascript
// Always show subtitles
player.setSubtitleDisplay('always');

// Show only when audio is muted
player.setSubtitleDisplay('mute');

// Never show
player.setSubtitleDisplay('never');
```

### Dual Subtitles

```javascript
// Enable dual subtitles (two languages at once)
player.setDualSubtitles(true);
player.setPrimarySubtitle('en');
player.setSecondarySubtitle('pl');
```

## Subtitle Formats

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

### ASS/SSA Format

```ass
[Script Info]
ScriptType: v4.00+

[V4+ Styles]
Format: Name, Fontname, Fontsize, PrimaryColour, Bold, Italic
Style: Default,Arial,20,&H00FFFFFF,-1,0

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:01.00,0:00:04.00,Default,,0,0,0,,Hello, world!
Dialogue: 0,0:00:05.00,0:00:08.00,Default,,0,0,0,,This is a subtitle example.
```

## Subtitle Shortcuts

### Keyboard Shortcuts

| Action | Windows/Linux | macOS |
|--------|--------------|-------|
| Toggle Subtitles | `V` | `V` |
| Next Subtitle | `Ctrl+Shift+→` | `Cmd+Shift+→` |
| Previous Subtitle | `Ctrl+Shift+←` | `Cmd+Shift+←` |
| Increase Delay | `+` | `+` |
| Decrease Delay | `-` | `-` |

## Subtitle Export

### Export Subtitles

```javascript
// Export to different format
const subtitles = await player.exportSubtitles('srt');
const vtt = await player.exportSubtitles('vtt');
const ass = await player.exportSubtitles('ass');
```

### Burn Subtitles into Video

```javascript
// Burn subtitles into video (requires re-encoding)
await player.burnSubtitles('output.mp4');
```

## Accessibility

### Hearing Impaired Captions

```javascript
// Enable hearing impaired format (includes sound effects)
player.setSubtitleHearingImpaired(true);
```

### Screen Reader Support

Vantis Media Player provides ARIA attributes for subtitle elements to ensure compatibility with screen readers.

## Troubleshooting

### Subtitles Not Showing

```javascript
// Check if subtitles are loaded
const isLoaded = player.areSubtitlesLoaded();

// Check if subtitles are enabled
const isEnabled = player.areSubtitlesEnabled();

// Force enable
player.setSubtitlesEnabled(true);
```

### Subtitles Out of Sync

```javascript
// Reset sync
player.setSubtitleDelay(0);

// Try auto-sync
player.setSubtitleAutoSync(true);
```

### Wrong Character Encoding

```toml
[subtitle]
encoding = "utf-8"
```

```javascript
// Set encoding
player.setSubtitleEncoding('utf-8');

// Auto-detect encoding
player.setSubtitleEncoding('auto');
```

## Next Steps

- **[Playlists](./playlists)** - Manage media playlists
- **[Advanced Features](../advanced-features/)** - Explore advanced functionality
- **[API Reference](../api/)** - Complete API documentation

## Need Help?

- [Troubleshooting Guide](../reference/troubleshooting)
- [FAQ](../reference/faq)
- [GitHub Discussions](https://github.com/vantisCorp/VantisMedia/discussions)