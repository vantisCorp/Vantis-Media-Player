// Example: Network streaming capabilities in Vantis Media Player
//
// This example demonstrates how to stream media from various sources,
// including HTTP, RTSP, and custom protocols.

use vantis_core::{Player, StreamSource, StreamingConfig};
use vantis_core::network::{NetworkClient, StreamProtocol};

#[tokio::main]
async fn main() {
    println!("Vantis Media Player - Network Streaming Example");
    println!("=================================================\n");
    
    // Create player
    let mut player = Player::new();
    
    // Example 1: Stream from HTTP/HTTPS
    println!("Example 1: HTTP/HTTPS streaming");
    println!("--------------------------------");
    
    let http_url = "https://example.com/video.mp4";
    println!("Streaming from: {}", http_url);
    
    match player.load_from_url(http_url).await {
        Ok(_) => {
            println!("✓ Stream loaded successfully");
            
            player.play();
            println!("✓ Playback started");
            
            // Monitor stream
            monitor_stream(&player).await;
            
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load stream: {}", e);
        }
    }
    
    println!();
    
    // Example 2: Stream from RTSP
    println!("Example 2: RTSP streaming");
    println!("-------------------------");
    
    let rtsp_url = "rtsp://camera.example.com/stream";
    println!("Streaming from: {}", rtsp_url);
    
    match player.load_from_url(rtsp_url).await {
        Ok(_) => {
            println!("✓ RTSP stream loaded");
            
            player.play();
            println!("✓ Playback started");
            
            // Monitor stream
            monitor_stream(&player).await;
            
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load RTSP stream: {}", e);
        }
    }
    
    println!();
    
    // Example 3: Stream with custom headers
    println!("Example 3: Streaming with authentication");
    println!("-----------------------------------------");
    
    let mut config = StreamingConfig::default();
    config.headers.insert(
        "Authorization".to_string(),
        "Bearer token123".to_string()
    );
    config.headers.insert(
        "User-Agent".to_string(),
        "VantisPlayer/1.0".to_string()
    );
    
    let url = "https://secure.example.com/protected_video.mp4";
    println!("Streaming with authentication from: {}", url);
    
    match player.load_from_url_with_config(url, config).await {
        Ok(_) => {
            println!("✓ Stream loaded with authentication");
            
            player.play();
            monitor_stream(&player).await;
            
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load stream: {}", e);
        }
    }
    
    println!();
    
    // Example 4: Adaptive bitrate streaming
    println!("Example 4: Adaptive bitrate streaming");
    println!("------------------------------------");
    
    let dash_url = "https://example.com/video.mpd";
    println!("DASH manifest: {}", dash_url);
    
    let mut config = StreamingConfig::default();
    config.adaptive_bitrate = true;
    config.target_bitrate = Some(5_000_000); // 5 Mbps
    config.max_bitrate = Some(10_000_000);   // 10 Mbps
    
    match player.load_from_url_with_config(dash_url, config).await {
        Ok(_) => {
            println!("✓ DASH stream loaded");
            
            player.play();
            
            // Monitor adaptive bitrate
            for _ in 0..10 {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                if let Some(stats) = player.streaming_stats() {
                    println!("  Current bitrate: {} Mbps", 
                        stats.current_bitrate / 1_000_000);
                    println!("  Buffer health: {:.1}%", 
                        stats.buffer_health * 100.0);
                }
            }
            
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load DASH stream: {}", e);
        }
    }
    
    println!();
    
    // Example 5: Live streaming
    println!("Example 5: Live streaming");
    println!("------------------------");
    
    let live_url = "https://example.com/live/stream.m3u8";
    println!("Live stream: {}", live_url);
    
    let mut config = StreamingConfig::default();
    config.live = true;
    config.buffer_size_ms = 30000; // 30 seconds buffer for live
    
    match player.load_from_url_with_config(live_url, config).await {
        Ok(_) => {
            println!("✓ Live stream loaded");
            
            player.play();
            
            // Monitor live stream
            for i in 0..15 {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                
                let position = player.current_position();
                let buffer = player.buffer_health();
                
                println!("  T+{}s - Position: {:.1}s - Buffer: {:.1}%", 
                    i * 2, position, buffer * 100.0);
            }
            
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load live stream: {}", e);
        }
    }
    
    println!();
    
    // Example 6: Stream quality selection
    println!("Example 6: Stream quality selection");
    println!("------------------------------------");
    
    let url = "https://example.com/video.mpd";
    
    match player.load_from_url(url).await {
        Ok(_) => {
            println!("✓ Stream loaded");
            
            // List available qualities
            if let Some(qualities) = player.available_qualities() {
                println!("Available qualities:");
                for (i, quality) in qualities.iter().enumerate() {
                    println!("  {}. {} - {} Mbps", 
                        i + 1, quality.name, quality.bitrate / 1_000_000);
                }
                
                // Select specific quality
                if let Some(target) = qualities.get(1) {
                    player.select_quality(target.id);
                    println!("✓ Selected: {}", target.name);
                }
            }
            
            player.play();
            monitor_stream(&player).await;
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load stream: {}", e);
        }
    }
    
    println!();
    
    // Example 7: Custom stream source
    println!("Example 7: Custom stream source");
    println!("--------------------------------");
    
    // Create custom stream source
    let source = StreamSource::Custom {
        name: "My Custom Stream".to_string(),
        protocol: StreamProtocol::Custom("my_protocol".to_string()),
        url: "myprotocol://stream.example.com".to_string(),
    };
    
    println!("Custom source: {:?}", source);
    
    println!();
    
    // Example 8: Stream recording
    println!("Example 8: Stream recording");
    println!("-------------------------");
    
    let url = "https://example.com/video.mp4";
    
    match player.load_from_url(url).await {
        Ok(_) => {
            println!("✓ Stream loaded");
            
            // Start recording
            match player.start_recording("examples/recorded_stream.mp4") {
                Ok(_) => {
                    println!("✓ Recording started");
                    
                    player.play();
                    
                    // Record for 10 seconds
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    
                    // Stop recording
                    player.stop_recording();
                    println!("✓ Recording stopped");
                    println!("✓ Saved to: examples/recorded_stream.mp4");
                }
                Err(e) => {
                    println!("✗ Failed to start recording: {}", e);
                }
            }
            
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load stream: {}", e);
        }
    }
    
    println!();
    
    // Example 9: Stream error handling
    println!("Example 9: Stream error handling");
    println!("---------------------------------");
    
    let invalid_url = "https://invalid.example.com/video.mp4";
    println!("Testing invalid URL: {}", invalid_url);
    
    match player.load_from_url(invalid_url).await {
        Ok(_) => {
            println!("✗ Should have failed!");
        }
        Err(e) => {
            println!("✓ Error handled correctly: {}", e);
            
            // Check error type
            match e {
                vantis_core::error::PlayerError::NetworkError(_) => {
                    println!("  Error type: Network error");
                }
                vantis_core::error::PlayerError::StreamError(_) => {
                    println!("  Error type: Stream error");
                }
                _ => {
                    println!("  Error type: Other");
                }
            }
        }
    }
    
    println!();
    
    // Example 10: Network client configuration
    println!("Example 10: Network client configuration");
    println!("----------------------------------------");
    
    let client = NetworkClient::new();
    
    // Configure client
    client.set_timeout(30000); // 30 seconds
    client.set_max_retries(3);
    client.set_user_agent("VantisPlayer/1.0");
    
    println!("Network client configured:");
    println!("  Timeout: 30 seconds");
    println!("  Max retries: 3");
    println!("  User-Agent: VantisPlayer/1.0");
    
    println!();
    
    // Example 11: Stream with proxies
    println!("Example 11: Proxy configuration");
    println!("----------------------------------");
    
    let mut config = StreamingConfig::default();
    config.proxy_url = Some("http://proxy.example.com:8080".to_string());
    config.proxy_username = Some("user".to_string());
    config.proxy_password = Some("pass".to_string());
    
    let url = "https://example.com/video.mp4";
    
    match player.load_from_url_with_config(url, config).await {
        Ok(_) => {
            println!("✓ Stream loaded via proxy");
            
            player.play();
            monitor_stream(&player).await;
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load stream: {}", e);
        }
    }
    
    println!();
    
    // Example 12: Stream caching
    println!("Example 12: Stream caching");
    println!("-------------------------");
    
    let mut config = StreamingConfig::default();
    config.cache_enabled = true;
    config.cache_size_mb = 512;
    config.cache_dir = Some("examples/stream_cache".to_string());
    
    let url = "https://example.com/video.mp4";
    
    match player.load_from_url_with_config(url, config).await {
        Ok(_) => {
            println!("✓ Stream loaded with caching");
            println!("  Cache size: 512 MB");
            println!("  Cache dir: examples/stream_cache");
            
            player.play();
            monitor_stream(&player).await;
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load stream: {}", e);
        }
    }
    
    println!();
    
    // Example 13: Multi-stream (picture-in-picture)
    println!("Example 13: Picture-in-picture");
    println!("---------------------------------");
    
    let url1 = "https://example.com/video1.mp4";
    let url2 = "https://example.com/video2.mp4";
    
    // Create secondary player for PiP
    let mut player2 = Player::new();
    
    match player.load_from_url(url1).await {
        Ok(_) => {
            println!("✓ Main stream loaded");
            
            player.play();
            
            // Load second stream
            if let Ok(_) = player2.load_from_url(url2).await {
                println!("✓ PiP stream loaded");
                
                player2.play();
                
                // Monitor both streams
                for _ in 0..5 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    
                    println!("  Main: {:.1}s | PiP: {:.1}s", 
                        player.current_position(),
                        player2.current_position());
                }
                
                player2.stop();
            }
            
            player.stop();
        }
        Err(e) => {
            println!("✗ Failed to load stream: {}", e);
        }
    }
    
    println!();
    println!("Network streaming examples completed!");
}

/// Monitor streaming progress
async fn monitor_stream(player: &Player) {
    println!("\nMonitoring stream...");
    
    for i in 0..10 {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        let position = player.current_position();
        let duration = player.duration();
        let buffer = player.buffer_health();
        
        let progress = if duration > 0.0 {
            (position / duration) * 100.0
        } else {
            0.0
        };
        
        println!("  T+{}s - Position: {:.1}s / {:.1}s ({:.1}%) - Buffer: {:.1}%", 
            i, position, duration, progress, buffer * 100.0);
    }
    
    println!();
}