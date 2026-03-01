//! Advanced Documentation System Example
//! 
//! This example demonstrates all features of the advanced documentation system:
//! - Interactive tutorials
//! - Video tutorials
//! - API playground
//! - Examples gallery
//! - Troubleshooting wizard

use std::path::PathBuf;
use tokio::time::sleep;
use std::time::Duration;
use vantisplayer::advanced_docs::{
    AdvancedDocumentationSystem, AdvancedDocsConfig,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("📚 Advanced Documentation System Example\n");
    
    // Create documentation and output directories
    let docs_dir = PathBuf::from("./docs");
    let output_dir = PathBuf::from("./output");
    
    // Create system with custom configuration
    let config = AdvancedDocsConfig {
        enable_tutorials: true,
        enable_video_tutorials: true,
        enable_api_playground: true,
        enable_examples_gallery: true,
        enable_troubleshooting: true,
        server_port: 8080,
        auto_reload: true,
        generate_static: true,
    };
    
    let docs_system = AdvancedDocumentationSystem::with_config(
        docs_dir.clone(),
        output_dir.clone(),
        config,
    )?;
    
    // Example 1: Interactive Tutorials
    println!("=== Example 1: Interactive Tutorials ===\n");
    example_tutorials(&docs_system).await?;
    
    // Example 2: Video Tutorials
    println!("\n=== Example 2: Video Tutorials ===\n");
    example_video_tutorials(&docs_system).await?;
    
    // Example 3: API Playground
    println!("\n=== Example 3: API Playground ===\n");
    example_api_playground(&docs_system).await?;
    
    // Example 4: Examples Gallery
    println!("\n=== Example 4: Examples Gallery ===\n");
    example_examples_gallery(&docs_system).await?;
    
    // Example 5: Troubleshooting Wizard
    println!("\n=== Example 5: Troubleshooting Wizard ===\n");
    example_troubleshooting_wizard(&docs_system).await?;
    
    // Example 6: Generate All Documentation
    println!("\n=== Example 6: Generate All Documentation ===\n");
    example_generate_all(&docs_system).await?;
    
    // Example 7: Start Documentation Server
    println!("\n=== Example 7: Start Documentation Server ===\n");
    example_start_server(&docs_system).await?;
    
    println!("\n✅ All examples completed successfully!");
    
    Ok(())
}

/// Example 1: Interactive Tutorials
async fn example_tutorials(docs_system: &AdvancedDocumentationSystem) -> anyhow::Result<()> {
    println!("📖 Accessing interactive tutorials...");
    
    let tutorials = docs_system.tutorials();
    
    // Get all tutorials
    let all_tutorials = tutorials.get_all_tutorials();
    println!("✅ Found {} tutorials", all_tutorials.len());
    
    // Get tutorials by category
    let getting_started = tutorials.get_tutorials_by_category(
        vantisplayer::advanced_docs::tutorials::TutorialCategory::GettingStarted
    );
    println!("✅ Getting Started tutorials: {}", getting_started.len());
    
    // Get tutorials by difficulty
    let beginner = tutorials.get_tutorials_by_difficulty(
        vantisplayer::advanced_docs::tutorials::DifficultyLevel::Beginner
    );
    println!("✅ Beginner tutorials: {}", beginner.len());
    
    Ok(())
}

/// Example 2: Video Tutorials
async fn example_video_tutorials(docs_system: &AdvancedDocumentationSystem) -> anyhow::Result<()> {
    println!("🎬 Accessing video tutorials...");
    
    let video_tutorials = docs_system.video_tutorials();
    
    // Get all videos
    let all_videos = video_tutorials.get_all_videos();
    println!("✅ Found {} video tutorials", all_videos.len());
    
    // Get videos by category
    let video_videos = video_tutorials.get_videos_by_category(
        vantisplayer::advanced_docs::video_tutorials::VideoCategory::Video
    );
    println!("✅ Video category tutorials: {}", video_videos.len());
    
    Ok(())
}

/// Example 3: API Playground
async fn example_api_playground(docs_system: &AdvancedDocumentationSystem) -> anyhow::Result<()> {
    println!("🎮 Accessing API playground...");
    
    let api_playground = docs_system.api_playground();
    
    // Get all endpoints
    let all_endpoints = api_playground.get_all_endpoints();
    println!("✅ Found {} API endpoints", all_endpoints.len());
    
    // Get specific endpoint
    if let Some(endpoint) = api_playground.get_endpoint("play") {
        println!("✅ Found endpoint: {} {}", endpoint.method, endpoint.path);
    }
    
    Ok(())
}

/// Example 4: Examples Gallery
async fn example_examples_gallery(docs_system: &AdvancedDocumentationSystem) -> anyhow::Result<()> {
    println!("📚 Accessing examples gallery...");
    
    let examples_gallery = docs_system.examples_gallery();
    
    // Get all examples
    let all_examples = examples_gallery.get_all_examples();
    println!("✅ Found {} code examples", all_examples.len());
    
    // Get examples by category
    let video_examples = examples_gallery.get_examples_by_category(
        vantisplayer::advanced_docs::examples_gallery::ExampleCategory::Video
    );
    println!("✅ Video examples: {}", video_examples.len());
    
    // Search examples
    let results = examples_gallery.search_examples("playback");
    println!("✅ Search results: {}", results.len());
    
    Ok(())
}

/// Example 5: Troubleshooting Wizard
async fn example_troubleshooting_wizard(docs_system: &AdvancedDocumentationSystem) -> anyhow::Result<()> {
    println!("🔧 Accessing troubleshooting wizard...");
    
    let troubleshooting = docs_system.troubleshooting();
    
    // Search by symptom
    let results = troubleshooting.search_by_symptom("no audio");
    println!("✅ Found {} troubleshooting guides", results.len());
    
    // Get specific guide
    if let Some(guide) = troubleshooting.get_guide("audio-issues") {
        println!("✅ Found guide: {}", guide.title);
    }
    
    // Get specific solution
    if let Some(solution) = troubleshooting.get_solution("check-audio-device") {
        println!("✅ Found solution: {}", solution.title);
    }
    
    Ok(())
}

/// Example 6: Generate All Documentation
async fn example_generate_all(docs_system: &AdvancedDocumentationSystem) -> anyhow::Result<()> {
    println!("📄 Generating all documentation...");
    
    let start = std::time::Instant::now();
    docs_system.generate_all().await?;
    let duration = start.elapsed();
    
    println!("✅ Documentation generated in {:?}", duration);
    println!("   - Check output/ directory for generated files");
    
    Ok(())
}

/// Example 7: Start Documentation Server
async fn example_start_server(docs_system: &AdvancedDocumentationSystem) -> anyhow::Result<()> {
    println!("🚀 Starting documentation server...");
    
    // Start server (this would normally run indefinitely)
    println!("✅ Documentation server would start on http://localhost:8080");
    println!("   - Interactive tutorials: http://localhost:8080/tutorials/");
    println!("   - Video tutorials: http://localhost:8080/video_tutorials/");
    println!("   - API playground: http://localhost:8080/api_playground/");
    println!("   - Examples gallery: http://localhost:8080/examples/");
    println!("   - Troubleshooting: http://localhost:8080/troubleshooting/");
    
    // Note: In a real application, you would call:
    // docs_system.start_server().await?;
    // And then wait for Ctrl+C to stop
    
    Ok(())
}