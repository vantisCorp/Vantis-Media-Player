---
sidebar_position: 2
---

# Advanced Player Example

This example demonstrates advanced features of Vantis Media Player including adaptive streaming, multiple audio tracks, subtitles, and advanced configuration options.

## Complete Advanced Player

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Advanced Vantis Player</title>
  <link rel="stylesheet" href="https://cdn.vantis.media/player/latest/vantis.min.css">
  <style>
    * {
      box-sizing: border-box;
    }
    
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
      margin: 0;
      padding: 20px;
      background: #0f0f1a;
      color: #e0e0e0;
    }
    
    .container {
      max-width: 1200px;
      margin: 0 auto;
    }
    
    h1 {
      text-align: center;
      margin-bottom: 30px;
      color: #fff;
    }
    
    .player-wrapper {
      display: grid;
      grid-template-columns: 1fr 300px;
      gap: 20px;
    }
    
    @media (max-width: 900px) {
      .player-wrapper {
        grid-template-columns: 1fr;
      }
    }
    
    #player-container {
      width: 100%;
      aspect-ratio: 16 / 9;
      background: #000;
      border-radius: 8px;
      overflow: hidden;
    }
    
    .sidebar {
      background: #1a1a2e;
      border-radius: 8px;
      padding: 20px;
    }
    
    .panel {
      margin-bottom: 20px;
    }
    
    .panel h3 {
      margin: 0 0 15px 0;
      font-size: 14px;
      text-transform: uppercase;
      color: #888;
    }
    
    .quality-list, .track-list {
      list-style: none;
      padding: 0;
      margin: 0;
    }
    
    .quality-list li, .track-list li {
      padding: 8px 12px;
      margin-bottom: 4px;
      background: #2d2d44;
      border-radius: 4px;
      cursor: pointer;
      transition: background 0.2s;
    }
    
    .quality-list li:hover, .track-list li:hover {
      background: #3d3d54;
    }
    
    .quality-list li.active, .track-list li.active {
      background: #6366f1;
    }
    
    .stats {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 10px;
    }
    
    .stat {
      background: #2d2d44;
      padding: 12px;
      border-radius: 4px;
      text-align: center;
    }
    
    .stat-value {
      font-size: 18px;
      font-weight: bold;
      color: #6366f1;
    }
    
    .stat-label {
      font-size: 11px;
      color: #888;
      text-transform: uppercase;
    }
    
    .btn-group {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
    }
    
    .btn {
      padding: 8px 16px;
      background: #2d2d44;
      border: none;
      border-radius: 4px;
      color: #fff;
      cursor: pointer;
      font-size: 13px;
    }
    
    .btn:hover {
      background: #3d3d54;
    }
    
    .btn.primary {
      background: #6366f1;
    }
    
    .btn.primary:hover {
      background: #4f46e5;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Advanced Vantis Player</h1>
    
    <div class="player-wrapper">
      <div id="player-container"></div>
      
      <div class="sidebar">
        <div class="panel">
          <h3>Video Quality</h3>
          <ul class="quality-list" id="quality-list">
            <li data-quality="auto" class="active">Auto</li>
          </ul>
        </div>
        
        <div class="panel">
          <h3>Audio Tracks</h3>
          <ul class="track-list" id="audio-tracks"></ul>
        </div>
        
        <div class="panel">
          <h3>Subtitles</h3>
          <ul class="track-list" id="subtitle-tracks"></ul>
        </div>
        
        <div class="panel">
          <h3>Statistics</h3>
          <div class="stats">
            <div class="stat">
              <div class="stat-value" id="stat-bitrate">0 kbps</div>
              <div class="stat-label">Bitrate</div>
            </div>
            <div class="stat">
              <div class="stat-value" id="stat-fps">0</div>
              <div class="stat-label">FPS</div>
            </div>
            <div class="stat">
              <div class="stat-value" id="stat-dropped">0</div>
              <div class="stat-label">Dropped</div>
            </div>
            <div class="stat">
              <div class="stat-value" id="stat-buffer">0s</div>
              <div class="stat-label">Buffer</div>
            </div>
          </div>
        </div>
        
        <div class="panel">
          <h3>Actions</h3>
          <div class="btn-group">
            <button class="btn" id="btn-screenshot">Screenshot</button>
            <button class="btn" id="btn-pip">PiP</button>
            <button class="btn" id="btn-fullscreen">Fullscreen</button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <script src="https://cdn.vantis.media/player/latest/vantis.min.js"></script>
  <script>
    // Initialize advanced player
    const player = new VantisPlayer('#player-container', {
      // Core settings
      autoplay: false,
      muted: false,
      volume: 0.8,
      playbackRate: 1.0,
      
      // UI settings
      controls: true,
      loading: {
        spinner: true,
        color: '#6366f1'
      },
      
      // Adaptive streaming
      streaming: {
        abr: {
          enabled: true,
          initialBitrate: 2000000,
          minBitrate: 300000,
          maxBitrate: 8000000
        },
        buffer: {
          forwardDuration: 30,
          backwardDuration: 10
        }
      },
      
      // Hardware acceleration
      hardwareAcceleration: true,
      
      // Subtitle settings
      subtitles: {
        enabled: true,
        style: {
          fontFamily: 'Arial, sans-serif',
          fontSize: '24px',
          color: '#ffffff',
          backgroundColor: 'rgba(0, 0, 0, 0.7)',
          textShadow: '1px 1px 2px black'
        }
      }
    });
    
    // Load HLS stream with multiple tracks
    async function loadAdvancedContent() {
      await player.load({
        type: 'hls',
        src: 'https://test-streams.mux.dev/x36xh99/x36xh99.m3u8',
        title: 'Test Stream',
        
        // Additional audio tracks
        audioTracks: [
          { id: 'en', label: 'English', language: 'en' },
          { id: 'es', label: 'Spanish', language: 'es' }
        ],
        
        // Subtitle tracks
        subtitleTracks: [
          { id: 'en', label: 'English', language: 'en', src: 'subtitles/en.vtt' },
          { id: 'es', label: 'Spanish', language: 'es', src: 'subtitles/es.vtt' }
        ]
      });
      
      updateQualityList();
      updateAudioTracks();
      updateSubtitleTracks();
    }
    
    // Update quality list
    function updateQualityList() {
      const qualityList = document.getElementById('quality-list');
      const qualities = player.getQualities();
      
      qualityList.innerHTML = '<li data-quality="auto" class="active">Auto</li>';
      
      qualities.forEach(q => {
        const li = document.createElement('li');
        li.dataset.quality = q.id;
        li.textContent = q.label;
        qualityList.appendChild(li);
      });
      
      qualityList.onclick = (e) => {
        const quality = e.target.dataset.quality;
        if (quality) {
          player.setQuality(quality);
          document.querySelectorAll('#quality-list li').forEach(l => l.classList.remove('active'));
          e.target.classList.add('active');
        }
      };
    }
    
    // Update audio tracks
    function updateAudioTracks() {
      const trackList = document.getElementById('audio-tracks');
      const tracks = player.getAudioTracks();
      
      trackList.innerHTML = '';
      
      tracks.forEach(track => {
        const li = document.createElement('li');
        li.dataset.trackId = track.id;
        li.textContent = track.label;
        if (track.enabled) li.classList.add('active');
        trackList.appendChild(li);
      });
      
      trackList.onclick = (e) => {
        const trackId = e.target.dataset.trackId;
        if (trackId) {
          player.setAudioTrack(trackId);
          document.querySelectorAll('#audio-tracks li').forEach(l => l.classList.remove('active'));
          e.target.classList.add('active');
        }
      };
    }
    
    // Update subtitle tracks
    function updateSubtitleTracks() {
      const trackList = document.getElementById('subtitle-tracks');
      const tracks = player.getSubtitleTracks();
      
      trackList.innerHTML = '<li data-track-id="off">Off</li>';
      
      tracks.forEach(track => {
        const li = document.createElement('li');
        li.dataset.trackId = track.id;
        li.textContent = track.label;
        if (track.enabled) li.classList.add('active');
        trackList.appendChild(li);
      });
      
      trackList.onclick = (e) => {
        const trackId = e.target.dataset.trackId;
        if (trackId === 'off') {
          player.disableSubtitles();
        } else if (trackId) {
          player.setSubtitleTrack(trackId);
        }
        document.querySelectorAll('#subtitle-tracks li').forEach(l => l.classList.remove('active'));
        e.target.classList.add('active');
      };
    }
    
    // Update statistics
    function updateStats() {
      const stats = player.getStats();
      
      document.getElementById('stat-bitrate').textContent = 
        Math.round(stats.bitrate / 1000) + ' kbps';
      document.getElementById('stat-fps').textContent = 
        Math.round(stats.fps);
      document.getElementById('stat-dropped').textContent = 
        stats.droppedFrames;
      document.getElementById('stat-buffer').textContent = 
        Math.round(stats.bufferLength) + 's';
    }
    
    // Event listeners
    player.on('player:ready', () => {
      console.log('Player ready');
      loadAdvancedContent();
    });
    
    player.on('streaming:qualitychange', (data) => {
      console.log('Quality changed:', data);
      document.querySelectorAll('#quality-list li').forEach(l => {
        l.classList.toggle('active', l.dataset.quality === data.quality);
      });
    });
    
    player.on('player:timeupdate', updateStats);
    
    // Button actions
    document.getElementById('btn-screenshot').onclick = () => {
      player.takeScreenshot().then(blob => {
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'screenshot.png';
        a.click();
      });
    };
    
    document.getElementById('btn-pip').onclick = () => {
      player.togglePictureInPicture();
    };
    
    document.getElementById('btn-fullscreen').onclick = () => {
      player.toggleFullscreen();
    };
  </script>
</body>
</html>
```

## Adaptive Streaming Configuration

### HLS Configuration

```javascript
const hlsConfig = {
  streaming: {
    hls: {
      // Segment loading
      maxBufferLength: 30,
      maxMaxBufferLength: 60,
      maxBufferSize: 60 * 1000 * 1000,
      maxBufferHole: 0.5,
      
      // ABR settings
      abr: {
        enabled: true,
        initialBitrate: 2000000,
        minBitrate: 300000,
        maxBitrate: 8000000,
        bandwidthSafetyFactor: 0.9,
        useDefaultBitrateEstimate: false
      },
      
      // Live settings
      live: {
        backBufferLength: 10,
        maxLiveSyncOffset: 15,
        lowLatencyMode: true
      },
      
      // Error handling
      fragLoadingTimeOut: 20000,
      fragLoadingMaxRetry: 6,
      manifestLoadingTimeOut: 10000,
      manifestLoadingMaxRetry: 6
    }
  }
};

const player = new VantisPlayer('#player', hlsConfig);
```

### DASH Configuration

```javascript
const dashConfig = {
  streaming: {
    dash: {
      // Segment settings
      bufferToKeep: 20,
      bufferAheadToKeep: 40,
      bufferPruningInterval: 10,
      
      // ABR settings
      abr: {
        enabled: true,
        initialBitrate: { video: 2000000 },
        minBitrate: { video: 300000 },
        maxBitrate: { video: 8000000 },
        bandwidthSafetyFactor: 0.9,
        useDefaultBitrateEstimate: false
      },
      
      // Live settings
      liveDelayFragmentCount: 4,
      liveDelay: 16,
      
      // Error handling
      retryAttempts: 3,
      retryInterval: 500
    }
  }
};

const player = new VantisPlayer('#player', dashConfig);
```

## Multi-Audio Track Handling

```javascript
// Load video with multiple audio tracks
await player.load({
  type: 'video',
  src: 'https://example.com/video.mp4',
  audioTracks: [
    {
      id: 'main',
      label: 'Original',
      language: 'en',
      default: true
    },
    {
      id: 'commentary',
      label: 'Director Commentary',
      language: 'en'
    },
    {
      id: 'dubbed-es',
      label: 'Spanish Dub',
      language: 'es'
    }
  ]
});

// Get available audio tracks
const audioTracks = player.getAudioTracks();
console.log('Available audio tracks:', audioTracks);

// Switch audio track
player.setAudioTrack('commentary');

// Listen for audio track changes
player.on('audio:trackchange', (data) => {
  console.log('Audio track changed to:', data.trackId);
});
```

## Advanced Subtitle Management

```javascript
// Load with multiple subtitle tracks
await player.load({
  type: 'video',
  src: 'https://example.com/video.mp4',
  subtitles: [
    {
      id: 'en',
      label: 'English',
      language: 'en',
      src: 'https://example.com/subs/en.vtt',
      default: true
    },
    {
      id: 'es',
      label: 'Spanish',
      language: 'es',
      src: 'https://example.com/subs/es.vtt'
    },
    {
      id: 'forced',
      label: 'Forced Narrative',
      language: 'en',
      src: 'https://example.com/subs/forced.vtt',
      forced: true
    }
  ]
});

// Style subtitles dynamically
player.setSubtitleStyle({
  fontFamily: 'Arial, sans-serif',
  fontSize: '24px',
  fontColor: '#ffffff',
  backgroundColor: 'rgba(0, 0, 0, 0.7)',
  textShadow: '2px 2px 4px rgba(0, 0, 0, 0.8)',
  edgeStyle: 'raised',
  windowColor: 'transparent',
  windowOpacity: 0
});

// Sync subtitles
player.on('subtitle:sync', (offset) => {
  console.log('Subtitle sync offset:', offset);
});

// Adjust subtitle timing
player.setSubtitleOffset(2.5); // Delay by 2.5 seconds
```

## Hardware Acceleration

```javascript
// Enable hardware acceleration
const player = new VantisPlayer('#player', {
  hardwareAcceleration: true,
  
  // GPU-specific settings
  gpu: {
    enabled: true,
    decoder: 'auto', // 'auto', 'd3d11', 'videotoolbox', 'vaapi', 'nvdec'
    zeroCopy: true,
    multiThreaded: true
  }
});

// Check hardware acceleration status
player.on('hardware:initialized', (info) => {
  console.log('Hardware acceleration:', info);
  console.log('Decoder:', info.decoder);
  console.log('GPU:', info.gpu);
});

// Fallback to software decoding
player.on('hardware:error', (error) => {
  console.warn('Hardware acceleration failed, falling back to software:', error);
});
```

## Playback Rate Control

```javascript
// Set playback rate
player.setPlaybackRate(1.5);

// Available rates
const availableRates = player.getAvailablePlaybackRates();
// [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0]

// Smooth rate change (for variable speed)
player.setPlaybackRateSmooth(2.0, {
  duration: 500, // ms
  easing: 'ease-in-out'
});

// Listen for rate changes
player.on('player:ratechange', (rate) => {
  console.log('Playback rate:', rate);
});
```

## Audio Processing

```javascript
// Enable audio effects
const player = new VantisPlayer('#player', {
  audio: {
    // Equalizer
    equalizer: {
      enabled: true,
      bands: [
        { frequency: 60, gain: 3 },    // Bass
        { frequency: 230, gain: 2 },
        { frequency: 910, gain: 0 },
        { frequency: 3600, gain: 1 },
        { frequency: 14000, gain: 2 }  // Treble
      ]
    },
    
    // Compressor
    compressor: {
      enabled: true,
      threshold: -24,
      knee: 30,
      ratio: 12,
      attack: 0.003,
      release: 0.25
    },
    
    // Stereo enhancement
    stereoEnhancer: {
      enabled: true,
      width: 1.5
    }
  }
});

// Dynamically adjust audio
player.setAudioEffect('equalizer', {
  bands: [
    { frequency: 60, gain: 5 }
  ]
});
```

## Performance Monitoring

```javascript
// Get performance metrics
setInterval(() => {
  const metrics = player.getPerformanceMetrics();
  
  console.log({
    fps: metrics.fps,
    droppedFrames: metrics.droppedFrames,
    decodeTime: metrics.decodeTime,
    bufferHealth: metrics.bufferHealth,
    bitrate: metrics.bitrate,
    latency: metrics.latency
  });
}, 1000);

// Enable detailed logging
player.enableDebugLogging(true);
```

## Error Recovery

```javascript
// Configure error recovery
const player = new VantisPlayer('#player', {
  errorRecovery: {
    enabled: true,
    maxRetries: 3,
    retryDelay: 1000,
    fallbackToSoftware: true,
    
    onError: (error) => {
      console.error('Player error:', error);
      return true; // Return true to attempt recovery
    }
  }
});

// Handle specific errors
player.on('player:error', (error) => {
  switch (error.code) {
    case 'NETWORK_ERROR':
      // Network error - retry
      player.retry();
      break;
      
    case 'DECODE_ERROR':
      // Decode error - switch to software
      player.switchToSoftwareDecoder();
      break;
      
    case 'SOURCE_ERROR':
      // Source error - reload
      player.reload();
      break;
      
    default:
      console.error('Unknown error:', error);
  }
});
```

## Next Steps

- [Custom Controls](./custom-controls) - Build your own player UI
- [Plugin Development](./plugin-development) - Extend player functionality
- [Integration Examples](./integration) - Integrate with frameworks