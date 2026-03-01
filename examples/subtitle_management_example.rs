// Example: Subtitle management in Vantis Media Player
//
// This example demonstrates how to work with the Vantis Babel subtitle system,
// including downloading, searching, and synchronizing subtitles.

use vantis_core::Player;
use vantis_subtitles::{SubtitleManager, SubtitleSource, SubtitleFormat};

#[tokio::main]
async fn main() {
    println!("Vantis Media Player - Subtitle Management Example");
    println!("===================================================\n");
    
    // Create a player instance
    let mut player = Player::new();
    
    // Create subtitle manager
    let subtitle_manager = SubtitleManager::new();
    
    // Example 1: Load video and auto-download subtitles
    println!("Example 1: Auto-download subtitles");
    println!("----------------------------------");
    
    let video_path = "examples/sample_video.mp4";
    player.load(video_path)
        .expect("Failed to load video");
    
    // Enable auto-download
    subtitle_manager.set_auto_download(true);
    
    // Download subtitles (automatically determines best source)
    match subtitle_manager.download_best_subtitles(video_path, "pl").await {
        Ok(subtitle_path) => {
            println!("✓ Downloaded subtitles: {}", subtitle_path);
            
            // Load subtitles into player
            player.load_subtitles(&subtitle_path);
            println!("✓ Subtitles loaded");
        }
        Err(e) => {
            println!("✗ Failed to download subtitles: {}", e);
        }
    }
    
    println!();
    
    // Example 2: Search for subtitles
    println!("Example 2: Search for subtitles");
    println!("---------------------------------");
    
    let search_term = "Inception 2010";
    println!("Searching for: '{}'", search_term);
    
    match subtitle_manager.search_subtitles(search_term, "pl").await {
        Ok(results) => {
            println!("Found {} results:", results.len());
            for (i, result) in results.iter().take(5).enumerate() {
                println!("  {}. {} - {} ({})", 
                    i + 1, 
                    result.title, 
                    result.language,
                    result.source
                );
            }
        }
        Err(e) => {
            println!("✗ Search failed: {}", e);
        }
    }
    
    println!();
    
    // Example 3: Download from specific source
    println!("Example 3: Download from specific source");
    println!("-----------------------------------------");
    
    match subtitle_manager.download_from_source(
        video_path, 
        SubtitleSource::NapiProjekt, 
        "pl"
    ).await {
        Ok(subtitle_path) => {
            println!("✓ Downloaded from NapiProjekt: {}", subtitle_path);
        }
        Err(e) => {
            println!("✗ Download failed: {}", e);
        }
    }
    
    println!();
    
    // Example 4: Fix encoding issues
    println!("Example 4: Fix subtitle encoding");
    println!("---------------------------------");
    
    let subtitle_file = "examples/test_subtitles.srt";
    
    // Detect and fix encoding automatically
    match subtitle_manager.fix_encoding(subtitle_file) {
        Ok(fixed_content) => {
            println!("✓ Encoding detected and fixed");
            
            // Save fixed subtitles
            let fixed_path = format!("{}_fixed.srt", 
                subtitle_file.trim_end_matches(".srt"));
            std::fs::write(&fixed_path, fixed_content)
                .expect("Failed to save fixed subtitles");
            println!("✓ Saved to: {}", fixed_path);
        }
        Err(e) => {
            println!("✗ Encoding fix failed: {}", e);
        }
    }
    
    println!();
    
    // Example 5: AI subtitle synchronization
    println!("Example 5: AI subtitle synchronization");
    println!("--------------------------------------");
    
    let unsynced_subtitle = "examples/unsynced_subtitles.srt";
    
    println!("Syncing subtitles to video...");
    
    match subtitle_manager.ai_sync_subtitles(unsynced_subtitle, video_path).await {
        Ok(synced_content) => {
            println!("✓ Subtitles synchronized using AI");
            
            // Save synced subtitles
            let synced_path = format!("{}_synced.srt", 
                unsynced_subtitle.trim_end_matches(".srt"));
            std::fs::write(&synced_path, synced_content)
                .expect("Failed to save synced subtitles");
            println!("✓ Saved to: {}", synced_path);
        }
        Err(e) => {
            println!("✗ Sync failed: {}", e);
        }
    }
    
    println!();
    
    // Example 6: Parse different subtitle formats
    println!("Example 6: Parse different formats");
    println!("-----------------------------------");
    
    let formats = vec![
        ("examples/test.srt", SubtitleFormat::SRT),
        ("examples/test.ass", SubtitleFormat::ASS),
        ("examples/test.vtt", SubtitleFormat::VTT),
    ];
    
    for (path, format) in formats {
        match subtitle_manager.parse_subtitle_file(path, format) {
            Ok(subtitles) => {
                println!("✓ Parsed {}: {} entries", 
                    path, subtitles.len());
            }
            Err(e) => {
                println!("✗ Failed to parse {}: {}", path, e);
            }
        }
    }
    
    println!();
    
    // Example 7: Convert between formats
    println!("Example 7: Convert subtitle format");
    println!("-----------------------------------");
    
    let input_file = "examples/input.srt";
    let output_file = "examples/output.ass";
    
    match subtitle_manager.convert_subtitle(input_file, output_file).await {
        Ok(_) => {
            println!("✓ Converted SRT to ASS");
            println!("  Input:  {}", input_file);
            println!("  Output: {}", output_file);
        }
        Err(e) => {
            println!("✗ Conversion failed: {}", e);
        }
    }
    
    println!();
    
    // Example 8: Configure subtitle settings
    println!("Example 8: Configure subtitle settings");
    println!("---------------------------------------");
    
    subtitle_manager.set_default_language("pl");
    subtitle_manager.set_font_size(28);
    subtitle_manager.set_font_family("Arial");
    subtitle_manager.set_font_color("#FFFFFF");
    subtitle_manager.set_background_color("#000000");
    subtitle_manager.set_border_width(2);
    
    println!("Subtitle configuration:");
    println!("  Default language: pl");
    println!("  Font size: 28");
    println!("  Font family: Arial");
    println!("  Font color: #FFFFFF");
    println!("  Background color: #000000");
    println!("  Border width: 2");
    
    println!();
    
    // Example 9: Subtitle statistics
    println!("Example 9: Subtitle statistics");
    println!("-------------------------------");
    
    let subtitle_path = "examples/test_subtitles.srt";
    
    match subtitle_manager.get_subtitle_info(subtitle_path) {
        Ok(info) => {
            println!("Subtitle file information:");
            println!("  Path: {}", subtitle_path);
            println!("  Format: {:?}", info.format);
            println!("  Entries: {}", info.entry_count);
            println!("  Duration: {:.1}s", info.duration);
            println!("  Languages: {:?}", info.languages);
            println!("  Encoding: {:?}", info.encoding);
        }
        Err(e) => {
            println!("✗ Failed to get subtitle info: {}", e);
        }
    }
    
    println!();
    
    // Example 10: Multiple subtitle tracks
    println!("Example 10: Multiple subtitle tracks");
    println!("------------------------------------");
    
    // Load multiple subtitle tracks
    let tracks = vec![
        "examples/polish.srt",
        "examples/english.srt",
        "examples/spanish.srt",
    ];
    
    for (index, track) in tracks.iter().enumerate() {
        match player.load_subtitle_track(track) {
            Ok(_) => {
                println!("✓ Loaded track {}: {}", index + 1, track);
            }
            Err(e) => {
                println!("✗ Failed to load track {}: {}", index + 1, e);
            }
        }
    }
    
    // List available tracks
    let available_tracks = player.list_subtitle_tracks();
    println!("\nAvailable subtitle tracks:");
    for (index, track) in available_tracks.iter().enumerate() {
        println!("  {}. {} ({}), Active: {}", 
            index + 1, 
            track.language, 
            track.format,
            track.is_active
        );
    }
    
    // Switch to specific track
    if let Some(track) = available_tracks.get(1) {
        player.select_subtitle_track(track.id);
        println!("\n✓ Selected track: {} ({})", track.language, track.format);
    }
    
    println!();
    println!("Subtitle management examples completed!");
}