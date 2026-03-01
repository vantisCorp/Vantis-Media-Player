// Example: Media library management in Vantis Media Player
//
// This example demonstrates how to work with the media library system,
// including scanning, organizing, and managing collections.

use vantis_core::{Player, MediaLibrary, MediaType, MediaItem};
use vantis_integrations::tmdb::TMDBClient;

#[tokio::main]
async fn main() {
    println!("Vantis Media Player - Media Library Example");
    println!("==============================================\n");
    
    // Initialize media library
    let library = MediaLibrary::new();
    
    // Example 1: Scan directories for media
    println!("Example 1: Scan directories");
    println!("----------------------------");
    
    let media_dirs = vec![
        "/media/Movies",
        "/media/TV Shows",
        "/media/Documentaries",
    ];
    
    for dir in &media_dirs {
        println!("Scanning: {}", dir);
        match library.scan_directory(dir, true).await {
            Ok(count) => {
                println!("  ✓ Found {} media items", count);
            }
            Err(e) => {
                println!("  ✗ Scan failed: {}", e);
            }
        }
    }
    
    println!();
    
    // Example 2: Filter by media type
    println!("Example 2: Filter by media type");
    println!("-------------------------------");
    
    let movies = library.filter_by_type(MediaType::Movie);
    let tv_shows = library.filter_by_type(MediaType::TVShow);
    
    println!("Movies: {}", movies.len());
    println!("TV Shows: {}", tv_shows.len());
    
    // Show recent movies
    println!("\nRecent Movies:");
    for (i, movie) in movies.iter().take(5).enumerate() {
        println!("  {}. {} ({})", i + 1, movie.title, movie.year);
    }
    
    println!();
    
    // Example 3: Search the library
    println!("Example 3: Search library");
    println!("-------------------------");
    
    let search_term = "Inception";
    println!("Searching for: '{}'", search_term);
    
    let results = library.search(search_term);
    println!("Found {} results:", results.len());
    
    for (i, item) in results.iter().take(10).enumerate() {
        println!("  {}. {} ({}) - {}", 
            i + 1, 
            item.title, 
            item.year,
            item.path
        );
    }
    
    println!();
    
    // Example 4: Create playlists
    println!("Example 4: Create playlists");
    println!("----------------------------");
    
    // Create a favorites playlist
    let favorites = library.create_playlist("Favorites");
    
    // Add items to playlist
    let favorite_items = vec![
        "Inception 2010",
        "The Dark Knight 2008",
        "Interstellar 2014",
    ];
    
    for item_name in &favorite_items {
        if let Some(item) = library.find_by_title(item_name) {
            library.add_to_playlist(&favorites, item.id);
            println!("  ✓ Added: {}", item.title);
        }
    }
    
    // Show playlist
    let playlist_items = library.get_playlist_items(&favorites);
    println!("\nPlaylist '{}':", favorites);
    for (i, item) in playlist_items.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, item.title, item.duration);
    }
    
    println!();
    
    // Example 5: Enrich metadata with TMDB
    println!("Example 5: Enrich metadata with TMDB");
    println!("--------------------------------------");
    
    let tmdb_client = TMDBClient::new(
        std::env::var("TMDB_API_KEY").unwrap_or_default()
    );
    
    // Get a movie item
    if let Some(movie) = library.find_by_title("Inception") {
        println!("Enriching: {}", movie.title);
        
        match tmdb_client.search_movie(&movie.title, movie.year).await {
            Ok(Some(metadata)) => {
                library.update_metadata(movie.id, metadata);
                println!("  ✓ Updated metadata");
                println!("    Plot: {}", metadata.overview.unwrap_or_default());
                println!("    Rating: {}/10", metadata.vote_average.unwrap_or(0.0));
                println!("    Genres: {:?}", metadata.genres);
            }
            Ok(None) => {
                println!("  ✗ No metadata found");
            }
            Err(e) => {
                println!("  ✗ TMDB error: {}", e);
            }
        }
    }
    
    println!();
    
    // Example 6: Get recommendations
    println!("Example 6: Get recommendations");
    println!("------------------------------");
    
    if let Some(movie) = library.find_by_title("Inception") {
        println!("Recommendations for: {}", movie.title);
        
        let recommendations = library.get_recommendations(movie.id, 5);
        for (i, item) in recommendations.iter().enumerate() {
            println!("  {}. {} ({}) - Similarity: {:.1}%", 
                i + 1, 
                item.title, 
                item.year,
                item.similarity.unwrap_or(0.0) * 100.0
            );
        }
    }
    
    println!();
    
    // Example 7: Watch history
    println!("Example 7: Watch history");
    println!("------------------------");
    
    let mut player = Player::new();
    
    // Play a video
    if let Some(movie) = library.find_by_title("Inception") {
        player.load(&movie.path).unwrap();
        player.play();
        
        // Simulate watching (seek to 30 minutes)
        player.seek(1800.0);
        
        // Record watch progress
        library.record_watch_progress(
            movie.id, 
            player.current_position(),
            player.duration()
        );
        
        println!("Recorded progress for: {}", movie.title);
        println!("  Position: {:.1}s / {:.1}s", 
            player.current_position(),
            player.duration()
        );
        
        player.stop();
    }
    
    // Get recently watched
    let recent = library.get_recently_watched(5);
    println!("\nRecently Watched:");
    for (i, (item, progress)) in recent.iter().enumerate() {
        let percentage = if item.duration > 0.0 {
            (progress / item.duration) * 100.0
        } else {
            0.0
        };
        println!("  {}. {} - {:.1}% complete", 
            i + 1, 
            item.title,
            percentage
        );
    }
    
    println!();
    
    // Example 8: Resume playback
    println!("Example 8: Resume playback");
    println!("--------------------------");
    
    if let Some(movie) = library.find_by_title("Inception") {
        let progress = library.get_watch_progress(movie.id);
        
        if let Some(position) = progress {
            println!("Resuming: {} at {:.1}s", movie.title, position);
            
            player.load(&movie.path).unwrap();
            player.seek(position);
            player.play();
            
            println!("✓ Playback resumed");
            
            player.stop();
        } else {
            println!("No watch progress found");
        }
    }
    
    println!();
    
    // Example 9: Filter by genre
    println!("Example 9: Filter by genre");
    println!("--------------------------");
    
    let sci_fi_movies = library.filter_by_genre("Science Fiction");
    println!("Science Fiction movies: {}", sci_fi_movies.len());
    
    for (i, movie) in sci_fi_movies.iter().take(5).enumerate() {
        println!("  {}. {} ({})", i + 1, movie.title, movie.year);
    }
    
    println!();
    
    // Example 10: Sort and order
    println!("Example 10: Sort and order");
    println!("--------------------------");
    
    // Sort by year (newest first)
    let sorted = library.sort_by(|a, b| b.year.cmp(&a.year));
    
    println!("Movies by year (newest):");
    for (i, movie) in sorted.iter().take(5).enumerate() {
        println!("  {}. {} ({}) - Rating: {:.1}", 
            i + 1, 
            movie.title, 
            movie.year,
            movie.rating.unwrap_or(0.0)
        );
    }
    
    println!();
    
    // Example 11: Export library
    println!("Example 11: Export library");
    println!("--------------------------");
    
    match library.export_json("library_export.json") {
        Ok(_) => {
            println!("✓ Library exported to: library_export.json");
        }
        Err(e) => {
            println!("✗ Export failed: {}", e);
        }
    }
    
    // Example 12: Import library
    println!("Example 12: Import library");
    println!("--------------------------");
    
    let mut new_library = MediaLibrary::new();
    match new_library.import_json("library_export.json") {
        Ok(count) => {
            println!("✓ Library imported: {} items", count);
        }
        Err(e) => {
            println!("✗ Import failed: {}", e);
        }
    }
    
    println!();
    
    // Example 13: Statistics
    println!("Example 13: Library statistics");
    println!("-------------------------------");
    
    let stats = library.get_statistics();
    println!("Total items: {}", stats.total_items);
    println!("Movies: {}", stats.movies);
    println!("TV Shows: {}", stats.tv_shows);
    println!("Total duration: {:.1} hours", stats.total_duration / 3600.0);
    println!("Total size: {:.2} GB", stats.total_size / (1024.0 * 1024.0 * 1024.0));
    println!("Genres: {:?}", stats.top_genres);
    println!("Years: {} - {}", stats.earliest_year, stats.latest_year);
    
    println!();
    
    // Example 14: Delete items
    println!("Example 14: Delete items");
    println!("------------------------");
    
    if let Some(item) = library.find_by_title("Old Movie") {
        println!("Deleting: {}", item.title);
        
        match library.delete_item(item.id) {
            Ok(_) => {
                println!("✓ Item deleted");
            }
            Err(e) => {
                println!("✗ Delete failed: {}", e);
            }
        }
    }
    
    println!();
    println!("Media library examples completed!");
}