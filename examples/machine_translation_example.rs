//! Machine Translation Example
//! 
//! This example demonstrates the machine translation features for subtitles
//! in the Vantis Media Player.

use subtitles::{
    VantisBabel, SubtitleTrack, SubtitleEntry,
    translation::{MachineTranslator, TranslationConfig, TranslationService, TranslationResult}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Machine Translation Example\n");
    
    // Create subtitle engine with translation support
    let mut babel = VantisBabel::new()?;
    
    // Create a sample subtitle track
    let mut track = SubtitleTrack::new("test.srt", "en");
    track.entries = vec![
        SubtitleEntry::new(0, 3000, "Hello, world!"),
        SubtitleEntry::new(3500, 6000, "Welcome to the Vantis Media Player"),
        SubtitleEntry::new(6500, 9000, "This is a demonstration of subtitle translation"),
    ];
    
    println!("📝 Original subtitle track (English):");
    for entry in &track.entries {
        println!("  [{}-{}ms] {}", entry.start, entry.end, entry.text);
    }
    println!();
    
    // Example 1: Translate entire subtitle track
    println!("1️⃣  Translating entire subtitle track to Spanish:");
    babel.translate_subtitle("test", "es").await?;
    
    println!("📝 Translated subtitle track (Spanish):");
    for entry in &track.entries {
        println!("  [{}-{}ms] {}", entry.start, entry.end, entry.text);
    }
    println!();
    
    // Example 2: Check supported languages
    println!("2️⃣  Supported languages:");
    let translator = babel.translator();
    let languages = translator.supported_languages();
    
    for (i, lang) in languages.iter().enumerate().take(10) {
        println!("  {}: {} ({})", lang.code, lang.name, lang.native_name);
    }
    println!("  ... and {} more", languages.len() - 10);
    println!();
    
    // Example 3: Test different translation services
    println!("3️⃣  Testing different translation services:");
    
    let text = "The quick brown fox jumps over the lazy dog";
    let target_lang = "de";
    
    println!("   Original: {}", text);
    
    for service in [
        TranslationService::GoogleTranslate,
        TranslationService::DeepL,
        TranslationService::LibreTranslate,
    ] {
        let mut config = TranslationConfig::default();
        config.service = service;
        let translator = MachineTranslator::new(config);
        
        let result = translator.translate(text, "en", target_lang).await?;
        println!("   {}: {}", service.as_str(), result.translated_text);
        println!("      Quality: {:.2}, Confidence: {:.2}", result.quality_score, result.confidence);
    }
    println!();
    
    // Example 4: Batch translation
    println!("4️⃣  Batch translation:");
    let translator = babel.translator();
    
    let texts = vec![
        "Welcome".to_string(),
        "Good morning".to_string(),
        "How are you?".to_string(),
        "Thank you".to_string(),
        "Goodbye".to_string(),
    ];
    
    let result = translator.batch_translate(&texts, "en", "fr").await?;
    println!("   Translated {}/{} texts successfully", result.successful, result.total_items);
    println!("   Average quality: {:.2}", result.average_quality);
    println!();
    
    for (i, r) in result.results.iter().enumerate() {
        println!("   {}: &quot;{}&quot; → &quot;{}&quot;", i + 1, r.original_text, r.translated_text);
    }
    println!();
    
    // Example 5: Translation caching
    println!("5️⃣  Translation caching:");
    let translator = babel.translator();
    
    // First translation (not cached)
    let result1 = translator.translate("Hello", "en", "es").await?;
    println!("   First translation - Cached: {}", result1.cached);
    
    // Second translation (should be cached)
    let result2 = translator.translate("Hello", "en", "es").await?;
    println!("   Second translation - Cached: {}", result2.cached);
    
    // Cache size
    let cache_size = translator.cache_size().await;
    println!("   Cache size: {}", cache_size);
    println!();
    
    // Example 6: Custom translation configuration
    println!("6️⃣  Custom translation configuration:");
    let mut config = TranslationConfig::default();
    config.service = TranslationService::DeepL;
    config.enable_cache = true;
    config.cache_size_limit = 500;
    config.batch_size = 100;
    
    let translator = MachineTranslator::new(config);
    
    println!("   Service: {}", translator.config().service.as_str());
    println!("   Cache enabled: {}", translator.config().enable_cache);
    println!("   Cache size limit: {}", translator.config().cache_size_limit);
    println!("   Batch size: {}", translator.config().batch_size);
    println!();
    
    // Example 7: Clear cache
    println!("7️⃣  Cache management:");
    let translator = babel.translator_mut();
    
    // Add some translations
    translator.translate("Test 1", "en", "es").await?;
    translator.translate("Test 2", "en", "fr").await?;
    translator.translate("Test 3", "en", "de").await?;
    
    let cache_size = translator.cache_size().await;
    println!("   Cache size before clearing: {}", cache_size);
    
    translator.clear_cache().await;
    
    let cache_size = translator.cache_size().await;
    println!("   Cache size after clearing: {}", cache_size);
    println!();
    
    println!("✅ Machine translation example completed successfully!");
    
    Ok(())
}