//! Subtitle Style Customization Example
//! 
//! This example demonstrates the subtitle style customization features
//! in the Vantis Media Player.

use subtitles::{
    StyleManager,
    SubtitleStyle,
    FontStyle,
    Alignment,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Subtitle Style Customization Example\n");
    
    // Create style manager
    let manager = StyleManager::new();
    
    // Example 1: Get default style
    println!("1️⃣  Getting default subtitle style:");
    let default_style = manager.get_current_style().await;
    println!("   Font: {}", default_style.font_family);
    println!("   Size: {}px", default_style.font_size);
    println!("   Color: {}", default_style.text_color);
    println!("   Position: {}%", default_style.vertical_position);
    println!();
    
    // Example 2: List available presets
    println!("2️⃣  Available style presets:");
    let presets = manager.get_presets().await;
    for preset in &presets {
        println!("   - {} ({})", preset.name, preset.description);
        println!("     Category: {}", preset.category);
    }
    println!("   Total: {} presets", presets.len());
    println!();
    
    // Example 3: Apply a preset
    println!("3️⃣  Applying 'Modern' preset:");
    manager.apply_preset("modern").await?;
    
    let modern_style = manager.get_current_style().await;
    println!("   Font: {}", modern_style.font_family);
    println!("   Size: {}px", modern_style.font_size);
    println!("   Color: {}", modern_style.text_color);
    println!("   Background: {}", modern_style.background_color);
    println!();
    
    // Example 4: Create custom style
    println!("4️⃣  Creating custom style:");
    let custom_style = SubtitleStyle {
        font_family: "Comic Sans MS".to_string(),
        font_size: 32,
        font_weight: 700,
        font_style: FontStyle::Italic,
        text_color: "#00FF00".to_string(),
        background_color: "#333333".to_string(),
        background_opacity: 0.9,
        vertical_position: 90,
        horizontal_alignment: Alignment::Right,
        ..Default::default()
    };
    
    manager.set_current_style(custom_style.clone()).await?;
    let current_style = manager.get_current_style().await;
    
    println!("   Font: {} {}px", current_style.font_family, current_style.font_size);
    println!("   Weight: {}", current_style.font_weight);
    println!("   Style: {}", current_style.font_style.as_str());
    println!("   Color: {}", current_style.text_color);
    println!("   Alignment: {}", current_style.horizontal_alignment.as_str());
    println!();
    
    // Example 5: Save custom style
    println!("5️⃣  Saving custom style as 'My Style':");
    manager.save_custom_style("My Style".to_string(), custom_style).await?;
    println!("   ✅ Custom style saved");
    
    let custom_styles = manager.get_custom_styles().await;
    println!("   Total custom styles: {}", custom_styles.len());
    println!();
    
    // Example 6: Load custom style
    println!("6️⃣  Loading custom style:");
    let loaded = manager.load_custom_style("My Style").await?;
    println!("   Loaded style with font: {}", loaded.font_family);
    println!();
    
    // Example 7: Export style to JSON
    println!("7️⃣  Exporting style to JSON:");
    let json = manager.export_style(&loaded).await?;
    println!("   JSON length: {} characters", json.len());
    println!("   Preview: {}", &json[..100]);
    println!();
    
    // Example 8: Import style from JSON
    println!("8️⃣  Importing style from JSON:");
    let imported = manager.import_style(&json).await?;
    println!("   Imported style: {} {}px", imported.font_family, imported.font_size);
    println!();
    
    // Example 9: Get editor state
    println!("9️⃣  Getting editor state:");
    let editor_state = manager.get_editor_state().await;
    println!("   Has unsaved changes: {}", editor_state.has_unsaved_changes);
    println!("   Selected preset: {:?}", editor_state.selected_preset);
    println!("   Available fonts: {} fonts", editor_state.config.available_fonts.len());
    println!();
    
    // Example 10: Test style validation
    println!("🔟  Testing style validation:");
    
    // Valid style
    let valid_style = SubtitleStyle::default();
    println!("   Valid style: {}", valid_style.validate().is_ok());
    
    // Invalid style (font size too large)
    let mut invalid_style = SubtitleStyle::default();
    invalid_style.font_size = 100;
    println!("   Invalid style (size=100): {}", invalid_style.validate().is_ok());
    
    // Invalid style (font weight out of range)
    invalid_style.font_size = 24;
    invalid_style.font_weight = 50;
    println!("   Invalid style (weight=50): {}", invalid_style.validate().is_ok());
    println!();
    
    // Example 11: Generate CSS
    println!("1️⃣1️⃣  Generating CSS from style:");
    let css = modern_style.to_css();
    println!("   CSS snippet: {}...", &css[..150]);
    println!();
    
    // Example 12: Apply different presets
    println!("1️⃣2️⃣  Testing different presets:");
    for preset_name in ["classic", "minimal", "bold", "cinematic"] {
        if let Ok(()) = manager.apply_preset(preset_name).await {
            let style = manager.get_current_style().await;
            println!("   {}: {} {}px", preset_name, style.font_family, style.font_size);
        }
    }
    println!();
    
    // Example 13: Create style with values
    println!("1️⃣3️⃣  Creating style with specific values:");
    let custom = SubtitleStyle::with_values(
        "Lato".to_string(),
        36,
        "#FF00FF".to_string(),
    );
    println!("   Created style: {} {}px {}", custom.font_family, custom.font_size, custom.text_color);
    println!();
    
    // Example 14: Delete custom style
    println!("1️⃣4️⃣  Deleting custom style:");
    manager.delete_custom_style("My Style").await?;
    println!("   ✅ Custom style deleted");
    
    let custom_styles = manager.get_custom_styles().await;
    println!("   Remaining custom styles: {}", custom_styles.len());
    println!();
    
    // Example 15: Reset to default
    println!("1️⃣5️⃣  Resetting to default style:");
    manager.reset_to_default().await?;
    let default = manager.get_current_style().await;
    println!("   Reset to: {} {}px", default.font_family, default.font_size);
    println!();
    
    // Example 16: Test all font styles
    println!("1️⃣6️⃣  Testing all font styles:");
    for font_style in FontStyle::all() {
        let mut style = SubtitleStyle::default();
        style.font_style = font_style;
        println!("   {}: {}", font_style.as_str(), style.to_css().split(';').next().unwrap_or(""));
    }
    println!();
    
    // Example 17: Test all alignments
    println!("1️⃣7️⃣  Testing all alignments:");
    for alignment in [Alignment::Left, Alignment::Center, Alignment::Right] {
        let mut style = SubtitleStyle::default();
        style.horizontal_alignment = alignment;
        println!("   {}: {}", alignment.as_str(), style.horizontal_alignment.as_str());
    }
    println!();
    
    // Example 18: Advanced customization
    println!("1️⃣8️⃣  Advanced style customization:");
    let advanced = SubtitleStyle {
        font_family: "Noto Sans".to_string(),
        font_size: 28,
        font_weight: 500,
        font_style: FontStyle::Normal,
        text_color: "#F0F8FF".to_string(),  // AliceBlue
        background_color: "#2F4F4F".to_string(),  // DarkSlateGray
        background_opacity: 0.85,
        outline_color: "#000000".to_string(),
        outline_width: 2,
        shadow_color: "#000000".to_string(),
        shadow_blur: 4,
        shadow_offset_x: 0,
        shadow_offset_y: 2,
        vertical_position: 88,
        horizontal_alignment: Alignment::Center,
        line_spacing: 1.4,
        character_spacing: 1,
        border_radius: 10,
    };
    
    manager.set_current_style(advanced).await?;
    let styled = manager.get_current_style().await;
    
    println!("   Font: {}", styled.font_family);
    println!("   Size: {}px", styled.font_size);
    println!("   Line spacing: {}", styled.line_spacing);
    println!("   Char spacing: {}px", styled.character_spacing);
    println!("   Border radius: {}px", styled.border_radius);
    println!();
    
    println!("✅ Subtitle style customization example completed successfully!");
    
    Ok(())
}