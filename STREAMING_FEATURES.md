# Vantis Streaming - Comprehensive Guide

## Overview

Vantis Streaming is an advanced network streaming module that provides comprehensive streaming capabilities for the Vantis Media Player. Built with Rust and leveraging modern networking libraries, it offers intelligent features for adaptive streaming, P2P distribution, caching, and recording.

## Features

### 1. Adaptive Streaming

Automatic quality selection based on network conditions and buffer health.

#### Supported Quality Levels

- **Auto**: Automatic quality selection
- **Low**: 360p @ 500 kbps
- **Medium**: 480p @ 1 Mbps
- **High**: 720p @ 2.5 Mbps
- **Full HD**: 1080p @ 5 Mbps
- **Ultra**: 4K @ 20 Mbps

#### Usage Example

```rust
use vantis_streaming::{StreamingEngine, StreamingConfig, AdaptiveConfig};

// Create streaming engine
let mut engine = StreamingEngine::new()?;

// Configure adaptive streaming
let adaptive_config = AdaptiveConfig {
    enabled: true,
    min_quality: QualityLevel::Medium,
    max_quality: QualityLevel::FullHD,
    buffer_upgrade_threshold: 0.8,
    buffer_downgrade_threshold: 0.3,
    ..Default::default()
};

// Select quality based on conditions
let quality = engine.adaptive_streamer()
    .unwrap()
    .select_quality(3_000_000, 0.9)?;
```

#### Adaptive Features

- **Bandwidth Monitoring**: Real-time bandwidth measurement
- **Buffer Health Tracking**: Monitor buffer levels
- **Predictive Quality Selection**: Anticipate network changes
- **Smooth Transitions**: Gradual quality changes
- **Configurable Thresholds**: Customize upgrade/downgrade triggers

### 2. Bandwidth Monitoring

Real-time bandwidth measurement and statistics for optimal streaming.

#### Features

- **Current Bandwidth**: Instantaneous bandwidth measurement
- **Average Bandwidth**: Rolling average over time window
- **Peak/Minimum**: Track bandwidth extremes
- **Variance/Std Dev**: Measure bandwidth stability
- **Prediction**: Predict future bandwidth trends
- **Stability Score**: Assess connection quality

#### Usage Example

```rust
use vantis_streaming::{BandwidthMonitor, BandwidthMonitorConfig};

// Create bandwidth monitor
let mut monitor = BandwidthMonitor::new()?;

// Record bytes received
monitor.record_bytes(1024 * 1024); // 1 MB

// Update statistics
let stats = monitor.update()?;

println!("Current bandwidth: {} bps", stats.current_bandwidth);
println!("Average bandwidth: {} bps", stats.average_bandwidth);
println!("Stability score: {:.2}", monitor.stability_score());
```

### 3. Stream Caching

Intelligent caching system for improved performance and reduced bandwidth usage.

#### Features

- **LRU/LFU/FIFO Eviction**: Multiple cache eviction policies
- **Size-based Eviction**: Evict largest entries first
- **Expiration**: Time-based cache expiration
- **Compression**: Optional data compression
- **Encryption**: Optional data encryption
- **Statistics**: Cache hit rate and utilization

#### Usage Example

```rust
use vantis_streaming::{StreamCache, CacheConfig, EvictionPolicy};

// Create cache
let config = CacheConfig {
    max_cache_size: 2 * 1024 * 1024 * 1024, // 2 GB
    max_cache_duration_secs: 7 * 24 * 60 * 60, // 7 days
    eviction_policy: EvictionPolicy::LRU,
    ..Default::default()
};

let mut cache = StreamCache::new(config)?;

// Put data in cache
cache.put("key1".to_string(), vec![1, 2, 3])?;

// Get data from cache
if let Some(data) = cache.get("key1")? {
    println!("Cache hit: {:?}", data);
}

// Get statistics
let stats = cache.stats();
println!("Hit rate: {:.2}", stats.hit_rate);
println!("Utilization: {:.2}", stats.utilization);
```

### 4. Stream Recording

Record live streams for offline viewing.

#### Supported Formats

- **MP4**: H.264/H.265 video with AAC audio
- **MKV**: Matroska container with multiple codec support
- **WebM**: VP8/VP9 video with Vorbis/Opus audio
- **TS**: MPEG-TS transport stream
- **Raw**: Uncompressed raw data

#### Features

- **Automatic Splitting**: Split by time or size
- **Metadata Embedding**: Include stream metadata
- **Chapter Markers**: Add chapter points
- **Multiple Formats**: Support for various container formats
- **Buffered Writing**: Efficient buffered I/O

#### Usage Example

```rust
use vantis_streaming::{StreamRecorder, RecorderConfig, RecordingFormat, RecordingMetadata};

// Create recorder
let config = RecorderConfig {
    output_dir: "recordings".to_string(),
    format: RecordingFormat::MP4,
    enable_auto_split: true,
    split_interval_secs: 3600, // Split every hour
    ..Default::default()
};

let mut recorder = StreamRecorder::new(config)?;

// Start recording
let metadata = RecordingMetadata {
    title: "Live Stream Recording".to_string(),
    stream_url: "https://example.com/stream.m3u8".to_string(),
    ..Default::default()
};

let file_path = recorder.start(metadata)?;

// Write data
recorder.write(&video_data)?;

// Add chapter marker
recorder.add_chapter("Chapter 1".to_string(), 0.0)?;

// Stop recording
recorder.stop()?;
```

### 5. P2P Streaming

Peer-to-peer streaming using libp2p for distributed content delivery.

#### Features

- **DHT**: Distributed hash table for content discovery
- **Gossipsub**: Pub/sub messaging for peer coordination
- **mDNS**: Local network peer discovery
- **NAT Traversal**: Connect through NAT/firewalls
- **Piece-based Download**: Efficient content distribution
- **Peer Selection**: Choose best peers based on latency/speed

#### Usage Example

```rust
use vantis_streaming::{P2PStreamer, P2PConfig, ContentManifest};

// Create P2P streamer
let mut streamer = P2PStreamer::new()?;

// Start P2P networking
streamer.start()?;

// Connect to peer
streamer.connect("peer_id".to_string(), "/ip4/127.0.0.1/tcp/1234".to_string())?;

// Publish content
let manifest = ContentManifest {
    content_id: "content_123".to_string(),
    name: "Example Content".to_string(),
    total_size: 1024 * 1024 * 1024,
    piece_size: 256 * 1024,
    piece_count: 4096,
    root_hash: "abc123".to_string(),
    piece_hashes: vec![],
    metadata: HashMap::new(),
};

streamer.publish_content(manifest)?;

// Subscribe to content
streamer.subscribe_content("content_123".to_string())?;

// Request piece
let piece = streamer.request_piece("content_123", 0)?;

// Check download progress
let progress = streamer.download_progress("content_123");
println!("Download progress: {:.2}%", progress * 100.0);
```

### 6. Protocol Support

Support for multiple streaming protocols.

#### Supported Protocols

- **HTTP/HTTPS**: Progressive download
- **HLS**: HTTP Live Streaming (m3u8)
- **DASH**: Dynamic Adaptive Streaming (mpd)
- **RTSP**: Real Time Streaming Protocol
- **RTMP**: Real-Time Messaging Protocol
- **WebRTC**: Web Real-Time Communication
- **P2P**: Peer-to-Peer streaming

#### Usage Example

```rust
use vantis_streaming::{StreamSource, StreamProtocol, Authentication};

// Create stream source
let source = StreamSource {
    url: "https://example.com/stream.m3u8".to_string(),
    protocol: StreamProtocol::HLS,
    is_live: true,
    auth: Some(Authentication::Bearer {
        token: "your_token".to_string()
    }),
    headers: {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "VantisPlayer/1.0".to_string());
        headers
    },
    proxy: None,
};

// Detect protocol from URL
let protocol = StreamProtocol::from_url("https://example.com/stream.m3u8");
assert_eq!(protocol, StreamProtocol::HLS);
```

## Configuration

### Streaming Configuration

```rust
use vantis_streaming::StreamingConfig;

let config = StreamingConfig {
    enable_adaptive: true,
    enable_p2p: false,
    enable_caching: true,
    enable_bandwidth_monitoring: true,
    enable_recording: false,
    buffer_size: 16 * 1024 * 1024, // 16 MB
    max_buffer_duration_secs: 30.0,
    max_reconnect_attempts: 5,
    reconnect_delay_secs: 2.0,
    user_agent: "VantisMediaPlayer/1.0".to_string(),
    enable_http2: true,
    enable_tls_verification: true,
    custom_headers: HashMap::new(),
};
```

### Adaptive Configuration

```rust
use vantis_streaming::AdaptiveConfig;

let config = AdaptiveConfig {
    enabled: true,
    min_quality: QualityLevel::Low,
    max_quality: QualityLevel::Ultra,
    buffer_upgrade_threshold: 0.8,
    buffer_downgrade_threshold: 0.3,
    bandwidth_upgrade_threshold: 5_000_000,
    bandwidth_downgrade_threshold: 1_000_000,
    min_quality_change_interval_secs: 10.0,
    enable_prediction: true,
    prediction_window_size: 10,
};
```

### Cache Configuration

```rust
use vantis_streaming::{CacheConfig, EvictionPolicy};

let config = CacheConfig {
    enabled: true,
    cache_dir: ".cache/streaming".to_string(),
    max_cache_size: 2 * 1024 * 1024 * 1024, // 2 GB
    max_cache_duration_secs: 7 * 24 * 60 * 60, // 7 days
    enable_compression: false,
    compression_level: 6,
    enable_encryption: false,
    eviction_policy: EvictionPolicy::LRU,
};
```

### Recorder Configuration

```rust
use vantis_streaming::{RecorderConfig, RecordingFormat};

let config = RecorderConfig {
    output_dir: "recordings".to_string(),
    format: RecordingFormat::MP4,
    enable_auto_split: false,
    split_interval_secs: 0,
    split_size_bytes: 0,
    enable_metadata: true,
    enable_chapters: true,
    buffer_size: 8 * 1024 * 1024, // 8 MB
};
```

### P2P Configuration

```rust
use vantis_streaming::P2PConfig;

let config = P2PConfig {
    enabled: true,
    listen_address: "/ip4/0.0.0.0/tcp/0".to_string(),
    bootstrap_peers: vec![],
    max_peers: 50,
    enable_nat_traversal: true,
    enable_mdns: true,
    enable_dht: true,
    enable_gossipsub: true,
    connection_timeout_secs: 30,
    keep_alive_interval_secs: 60,
};
```

## Performance Optimization

### Tips for Best Performance

1. **Enable Caching**: Reduce bandwidth usage with intelligent caching
2. **Adjust Buffer Size**: Balance memory usage and smooth playback
3. **Use Adaptive Streaming**: Automatically adjust quality to network conditions
4. **Monitor Bandwidth**: Track network conditions for optimal quality
5. **Enable HTTP/2**: Take advantage of multiplexing and header compression
6. **Configure Timeouts**: Set appropriate timeouts for your network
7. **Use P2P for Popular Content**: Reduce server load with peer-to-peer distribution

### Memory Management

```rust
// Monitor cache usage
let cache_stats = cache.stats();
println!("Cache utilization: {:.2}%", cache_stats.utilization * 100.0);

// Clear cache if needed
cache.clear()?;

// Monitor P2P connections
let p2p_stats = streamer.stats();
println!("Connected peers: {}", p2p_stats.connected_peers);
```

## Error Handling

Vantis Streaming provides comprehensive error handling:

```rust
use vantis_streaming::StreamingError;

match engine.adaptive_streamer() {
    Ok(streamer) => {
        // Use streamer
    }
    Err(StreamingError::NetworkError(msg)) => {
        eprintln!("Network error: {}", msg);
    }
    Err(StreamingError::ProtocolError(msg)) => {
        eprintln!("Protocol error: {}", msg);
    }
    Err(StreamingError::Timeout) => {
        eprintln!("Operation timed out");
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Integration with Vantis Player

### Example Integration

```rust
use vantis_streaming::{StreamingEngine, StreamSource, StreamProtocol};
use vantis_core::Player;

struct StreamingPlayer {
    player: Player,
    streaming_engine: StreamingEngine,
}

impl StreamingPlayer {
    fn new() -> Result<Self, StreamingError> {
        let player = Player::new()?;
        let streaming_engine = StreamingEngine::new()?;
        
        Ok(Self {
            player,
            streaming_engine,
        })
    }
    
    fn play_stream(&mut self, url: &str) -> Result<(), StreamingError> {
        // Detect protocol
        let protocol = StreamProtocol::from_url(url);
        
        // Create stream source
        let source = StreamSource {
            url: url.to_string(),
            protocol,
            is_live: url.contains(".m3u8"),
            auth: None,
            headers: HashMap::new(),
            proxy: None,
        };
        
        // Start streaming
        // ... streaming logic ...
        
        Ok(())
    }
}
```

## Troubleshooting

### Common Issues

#### Buffer Underruns

**Problem**: Frequent rebuffering during playback

**Solution**:
- Increase buffer size
- Lower quality level
- Check network connection
- Enable caching

#### High Latency

**Problem**: Delay between stream and playback

**Solution**:
- Reduce buffer size
- Use lower latency protocols (WebRTC)
- Check network latency
- Disable adaptive streaming if not needed

#### Cache Issues

**Problem**: Cache not working as expected

**Solution**:
- Check cache directory permissions
- Verify cache size limits
- Clear cache and restart
- Check eviction policy

#### P2P Connection Issues

**Problem**: Cannot connect to peers

**Solution**:
- Check firewall settings
- Enable NAT traversal
- Verify bootstrap peers
- Check mDNS discovery

## API Reference

### Core Types

- `StreamingEngine`: Main streaming engine
- `StreamingConfig`: Streaming configuration
- `StreamStats`: Streaming statistics
- `StreamingError`: Streaming error types
- `StreamingResult<T>`: Result type for streaming operations

### Adaptive Streaming

- `AdaptiveStreamer`: Adaptive streaming engine
- `AdaptiveConfig`: Adaptive configuration
- `QualityLevel`: Quality level enumeration
- `QualitySelector`: Quality selector
- `QualityMetrics`: Quality metrics

### Bandwidth Monitoring

- `BandwidthMonitor`: Bandwidth monitor
- `BandwidthMonitorConfig`: Monitor configuration
- `BandwidthStats`: Bandwidth statistics

### Caching

- `StreamCache`: Stream cache
- `CacheConfig`: Cache configuration
- `CacheEntry`: Cache entry
- `CacheStats`: Cache statistics
- `EvictionPolicy`: Eviction policy enumeration

### Recording

- `StreamRecorder`: Stream recorder
- `RecorderConfig`: Recorder configuration
- `RecordingFormat`: Recording format enumeration
- `RecordingMetadata`: Recording metadata
- `RecordingStats`: Recording statistics
- `ChapterMarker`: Chapter marker

### P2P Streaming

- `P2PStreamer`: P2P streamer
- `P2PConfig`: P2P configuration
- `PeerInfo`: Peer information
- `ContentManifest`: Content manifest
- `ContentPiece`: Content piece
- `P2PStats`: P2P statistics

### Protocols

- `StreamProtocol`: Protocol enumeration
- `StreamSource`: Stream source
- `StreamInfo`: Stream information
- `QualityLevel`: Quality level for adaptive streaming
- `ProtocolHandler`: Protocol handler trait
- `HttpHandler`: HTTP protocol handler

### Utilities

- `parse_url`: Parse URL into components
- `format_bytes`: Format bytes to human-readable string
- `format_bitrate`: Format bitrate to human-readable string
- `format_duration`: Format duration to human-readable string
- `calculate_bitrate`: Calculate bitrate from bytes and duration
- `calculate_duration`: Calculate duration from bytes and bitrate
- `moving_average`: Calculate moving average
- `calculate_percentile`: Calculate percentile

## Best Practices

1. **Always Handle Errors**: Properly handle all streaming errors
2. **Monitor Statistics**: Track bandwidth, buffer health, and quality
3. **Use Appropriate Quality**: Start with lower quality for unstable connections
4. **Enable Caching**: Reduce bandwidth usage with caching
5. **Configure Timeouts**: Set appropriate timeouts for your network
6. **Test Thoroughly**: Test with various network conditions
7. **Log Events**: Enable logging for debugging
8. **Clean Up Resources**: Properly close streams and connections

## Future Enhancements

Planned features for future releases:

- WebRTC implementation
- RTSP/RTMP protocol handlers
- Advanced P2P features (swarm management, piece selection)
- Stream encryption
- DRM support
- Adaptive streaming improvements (ABR algorithms)
- Advanced caching strategies (predictive caching)
- Cloud integration (CDN support)
- Mobile optimization

## License

Vantis Streaming is part of the Vantis Media Player project and is licensed under MIT.

## Support

For issues, questions, or contributions:
- GitHub: https://github.com/vantis/vantis-player
- Documentation: https://docs.vantis.ai
- Community: https://community.vantis.ai