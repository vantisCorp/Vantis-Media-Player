//! Streaming Features Example for Vantis Media Player
//! 
//! This example demonstrates how to use the streaming features in Vantis Media Player:
//! - Adaptive streaming with quality selection
//! - Bandwidth monitoring
//! - Stream caching
//! - Stream recording
//! - P2P streaming
//! - Protocol handling

use vantis_streaming::{
    StreamingEngine, StreamingConfig,
    AdaptiveStreamer, AdaptiveConfig, QualityLevel,
    BandwidthMonitor, BandwidthMonitorConfig,
    StreamCache, CacheConfig, EvictionPolicy,
    StreamRecorder, RecorderConfig, RecordingFormat, RecordingMetadata,
    P2PStreamer, P2PConfig, ContentManifest,
    StreamSource, StreamProtocol, Authentication,
    format_bytes, format_bitrate, format_duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Vantis Streaming Features Example");
    println!("==================================\n");
    
    // Example 1: Adaptive Streaming
    println!("1. Adaptive Streaming Example");
    adaptive_streaming_example()?;
    println!();
    
    // Example 2: Bandwidth Monitoring
    println!("2. Bandwidth Monitoring Example");
    bandwidth_monitoring_example()?;
    println!();
    
    // Example 3: Stream Caching
    println!("3. Stream Caching Example");
    stream_caching_example()?;
    println!();
    
    // Example 4: Stream Recording
    println!("4. Stream Recording Example");
    stream_recording_example()?;
    println!();
    
    // Example 5: P2P Streaming
    println!("5. P2P Streaming Example");
    p2p_streaming_example()?;
    println!();
    
    // Example 6: Protocol Handling
    println!("6. Protocol Handling Example");
    protocol_handling_example()?;
    println!();
    
    println!("All examples completed successfully!");
    Ok(())
}

/// Adaptive streaming example
fn adaptive_streaming_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating adaptive streamer...");
    let mut streamer = AdaptiveStreamer::new()?;
    
    // Simulate network conditions
    println!("   Simulating network conditions...");
    let network_conditions = vec![
        (10_000_000, 0.9),  // 10 Mbps, good buffer
        (5_000_000, 0.8),   // 5 Mbps, good buffer
        (2_000_000, 0.6),   // 2 Mbps, medium buffer
        (500_000, 0.3),     // 500 kbps, low buffer
        (8_000_000, 0.95),  // 8 Mbps, excellent buffer
    ];
    
    for (bandwidth, buffer_health) in network_conditions {
        let quality = streamer.select_quality(bandwidth, buffer_health)?;
        
        println!("   Bandwidth: {}, Buffer: {:.2} → Quality: {:?}",
            format_bitrate(bandwidth),
            buffer_health,
            quality
        );
    }
    
    // Get quality statistics
    let stats = streamer.get_quality_stats();
    println!("\n   Quality Statistics:");
    println!("   - Current Quality: {:?}", stats.current_quality);
    println!("   - Quality Changes: {}", stats.quality_changes);
    println!("   - Average Bandwidth: {}", format_bitrate(stats.average_bandwidth));
    println!("   - Buffer Health: {:.2}", stats.buffer_health);
    
    Ok(())
}

/// Bandwidth monitoring example
fn bandwidth_monitoring_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating bandwidth monitor...");
    let mut monitor = BandwidthMonitor::new()?;
    
    // Simulate data reception
    println!("   Simulating data reception...");
    let data_rates = vec![
        1_000_000,  // 1 MB
        2_000_000,  // 2 MB
        1_500_000,  // 1.5 MB
        3_000_000,  // 3 MB
        2_500_000,  // 2.5 MB
    ];
    
    for (i, bytes) in data_rates.iter().enumerate() {
        monitor.record_bytes(*bytes);
        
        // Simulate time passing
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        let stats = monitor.update()?;
        
        println!("   Sample {}: {} → {}",
            i + 1,
            format_bytes(*bytes),
            format_bitrate(stats.current_bandwidth)
        );
    }
    
    // Get final statistics
    let stats = monitor.stats();
    println!("\n   Bandwidth Statistics:");
    println!("   - Current: {}", format_bitrate(stats.current_bandwidth));
    println!("   - Average: {}", format_bitrate(stats.average_bandwidth));
    println!("   - Peak: {}", format_bitrate(stats.peak_bandwidth));
    println!("   - Minimum: {}", format_bitrate(stats.min_bandwidth));
    println!("   - Std Deviation: {:.2}", stats.std_deviation);
    println!("   - Stability Score: {:.2}", monitor.stability_score());
    
    // Predict future bandwidth
    let predicted = monitor.predict_bandwidth();
    println!("   - Predicted: {}", format_bitrate(predicted));
    
    Ok(())
}

/// Stream caching example
fn stream_caching_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating stream cache...");
    let config = CacheConfig {
        max_cache_size: 1024, // Small cache for demo
        eviction_policy: EvictionPolicy::LRU,
        ..Default::default()
    };
    
    let mut cache = StreamCache::new(config)?;
    
    // Add entries to cache
    println!("   Adding entries to cache...");
    let entries = vec![
        ("key1", vec![1, 2, 3, 4, 5]),
        ("key2", vec![10, 20, 30, 40, 50]),
        ("key3", vec![100, 200, 300, 400, 500]),
    ];
    
    for (key, data) in &entries {
        cache.put(key.to_string(), data.clone())?;
        println!("   ✓ Cached: {} ({} bytes)", key, data.len());
    }
    
    // Retrieve entries
    println!("\n   Retrieving entries from cache...");
    for (key, _) in &entries {
        match cache.get(key)? {
            Some(data) => println!("   ✓ Cache hit: {} → {:?}", key, data),
            None => println!("   ✗ Cache miss: {}", key),
        }
    }
    
    // Get cache statistics
    let stats = cache.stats();
    println!("\n   Cache Statistics:");
    println!("   - Entries: {}", stats.entry_count);
    println!("   - Total Size: {}", format_bytes(stats.total_size));
    println!("   - Max Size: {}", format_bytes(stats.max_size));
    println!("   - Hit Rate: {:.2}%", stats.hit_rate * 100.0);
    println!("   - Utilization: {:.2}%", stats.utilization * 100.0);
    
    Ok(())
}

/// Stream recording example
fn stream_recording_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating stream recorder...");
    let config = RecorderConfig {
        output_dir: "recordings".to_string(),
        format: RecordingFormat::MP4,
        enable_auto_split: false,
        enable_metadata: true,
        enable_chapters: true,
        ..Default::default()
    };
    
    let mut recorder = StreamRecorder::new(config)?;
    
    // Start recording
    println!("   Starting recording...");
    let metadata = RecordingMetadata {
        title: "Test Stream Recording".to_string(),
        description: Some("Example recording".to_string()),
        stream_url: "https://example.com/stream.m3u8".to_string(),
        video_codec: Some("H.264".to_string()),
        audio_codec: Some("AAC".to_string()),
        resolution: Some((1920, 1080)),
        frame_rate: Some(30.0),
        bitrate: Some(5_000_000),
        ..Default::default()
    };
    
    let file_path = recorder.start(metadata)?;
    println!("   ✓ Recording started: {:?}", file_path);
    
    // Write some data
    println!("   Writing data...");
    let sample_data = vec![0u8; 1024 * 1024]; // 1 MB sample
    recorder.write(&sample_data)?;
    println!("   ✓ Written: {}", format_bytes(sample_data.len() as u64));
    
    // Add chapter marker
    println!("   Adding chapter marker...");
    recorder.add_chapter("Introduction".to_string(), 0.0)?;
    println!("   ✓ Chapter added");
    
    // Get recording statistics
    let stats = recorder.stats();
    println!("\n   Recording Statistics:");
    println!("   - Is Recording: {}", stats.is_recording);
    println!("   - Bytes Written: {}", format_bytes(stats.bytes_written));
    println!("   - Duration: {:?}", stats.duration_secs);
    println!("   - Chapters: {}", stats.chapter_count);
    
    // Stop recording
    println!("\n   Stopping recording...");
    recorder.stop()?;
    println!("   ✓ Recording stopped");
    
    Ok(())
}

/// P2P streaming example
fn p2p_streaming_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Creating P2P streamer...");
    let mut streamer = P2PStreamer::new()?;
    
    // Start P2P networking
    println!("   Starting P2P networking...");
    streamer.start()?;
    println!("   ✓ P2P networking started");
    
    // Connect to peers
    println!("\n   Connecting to peers...");
    let peers = vec![
        ("peer1", "/ip4/127.0.0.1/tcp/1234"),
        ("peer2", "/ip4/127.0.0.1/tcp/1235"),
        ("peer3", "/ip4/127.0.0.1/tcp/1236"),
    ];
    
    for (peer_id, address) in peers {
        streamer.connect(peer_id.to_string(), address.to_string())?;
        println!("   ✓ Connected to {} at {}", peer_id, address);
    }
    
    // List connected peers
    println!("\n   Connected Peers:");
    for peer in streamer.peers() {
        println!("   - {} ({} addresses)", peer.peer_id, peer.addresses.len());
    }
    
    // Publish content
    println!("\n   Publishing content...");
    let manifest = ContentManifest {
        content_id: "content_123".to_string(),
        name: "Example Video".to_string(),
        total_size: 1024 * 1024 * 1024, // 1 GB
        piece_size: 256 * 1024, // 256 KB
        piece_count: 4096,
        root_hash: "abc123def456".to_string(),
        piece_hashes: vec!["hash1".to_string(), "hash2".to_string()],
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("title".to_string(), "Example".to_string());
            meta
        },
    };
    
    streamer.publish_content(manifest)?;
    println!("   ✓ Content published");
    
    // Subscribe to content
    println!("\n   Subscribing to content...");
    streamer.subscribe_content("content_123".to_string())?;
    println!("   ✓ Subscribed to content_123");
    
    // Request pieces
    println!("\n   Requesting pieces...");
    for i in 0..3 {
        match streamer.request_piece("content_123", i) {
            Ok(piece) => println!("   ✓ Piece {} received ({} bytes)", piece.index, piece.size),
            Err(e) => println!("   ✗ Failed to request piece {}: {}", i, e),
        }
    }
    
    // Check download progress
    let progress = streamer.download_progress("content_123");
    println!("\n   Download Progress: {:.2}%", progress * 100.0);
    
    // Get P2P statistics
    let stats = streamer.stats();
    println!("\n   P2P Statistics:");
    println!("   - Connected Peers: {}", stats.connected_peers);
    println!("   - Published Content: {}", stats.published_content);
    println!("   - Subscribed Content: {}", stats.subscribed_content);
    println!("   - Pieces Downloaded: {}", stats.pieces_downloaded);
    println!("   - Pieces Uploaded: {}", stats.pieces_uploaded);
    
    Ok(())
}

/// Protocol handling example
fn protocol_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing protocol detection...");
    
    let test_urls = vec![
        "https://example.com/video.mp4",
        "https://example.com/stream.m3u8",
        "https://example.com/stream.mpd",
        "rtsp://example.com/stream",
        "rtmp://example.com/stream",
        "webrtc://example.com/stream",
        "p2p://example.com/content",
    ];
    
    for url in test_urls {
        let protocol = StreamProtocol::from_url(url);
        println!("   {} → {:?} ({})",
            url,
            protocol,
            protocol.name()
        );
    }
    
    // Test protocol properties
    println!("\n   Protocol Properties:");
    let protocols = vec![
        StreamProtocol::HTTP,
        StreamProtocol::HLS,
        StreamProtocol::DASH,
        StreamProtocol::RTSP,
    ];
    
    for protocol in protocols {
        println!("   {:?}:",
            protocol
        );
        println!("     - Supports Adaptive: {}", protocol.supports_adaptive());
        println!("     - Is Real-time: {}", protocol.is_realtime());
    }
    
    // Create stream source
    println!("\n   Creating stream source...");
    let source = StreamSource {
        url: "https://example.com/stream.m3u8".to_string(),
        protocol: StreamProtocol::HLS,
        is_live: true,
        auth: Some(Authentication::Bearer {
            token: "your_token_here".to_string()
        }),
        headers: {
            let mut headers = std::collections::HashMap::new();
            headers.insert("User-Agent".to_string(), "VantisPlayer/1.0".to_string());
            headers.insert("Accept".to_string(), "*/*".to_string());
            headers
        },
        proxy: None,
    };
    
    println!("   ✓ Stream source created:");
    println!("     - URL: {}", source.url);
    println!("     - Protocol: {:?}", source.protocol);
    println!("     - Is Live: {}", source.is_live);
    println!("     - Auth: {:?}", source.auth);
    println!("     - Headers: {} custom headers", source.headers.len());
    
    Ok(())
}