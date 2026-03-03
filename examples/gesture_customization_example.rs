//! Gesture Customization Example
//! 
//! This example demonstrates the gesture customization features
//! in the Vantis Media Player.

use advanced_ui::gestures::{
    GestureController, GestureConfig, GesturePresetManager, GestureType, GestureCallback, GestureEvent
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("👋 Gesture Customization Example\n");
    
    // Example 1: Create gesture controller with default config
    println!("1️⃣  Creating gesture controller with default configuration:");
    let config = GestureConfig::default();
    let controller = GestureController::new(config)?;
    
    let default_config = controller.get_config().await;
    println!("   Swipe sensitivity: {}", default_config.swipe_sensitivity);
    println!("   Tap sensitivity: {}", default_config.tap_sensitivity);
    println!("   Multi-touch enabled: {}", default_config.enable_multi_touch);
    println!();
    
    // Example 2: Get available gesture types
    println!("2️⃣  Available gesture types:");
    println!("   Swipe Left, Right, Up, Down");
    println!("   Pinch In, Pinch Out");
    println!("   Tap, Double Tap, Long Press");
    println!();
    
    // Example 3: Register gesture callbacks
    println!("3️⃣  Registering gesture callbacks:");
    
    let tap_callback: GestureCallback = Arc::new(|event| {
        println!("   🎯 Tap detected at ({}, {})", event.x, event.y);
    });
    
    let swipe_callback: GestureCallback = Arc::new(|event| {
        println!("   ➡️  Swipe detected: {:?}, distance: {}", event.gesture_type, event.distance);
    });
    
    // Note: In real implementation, you'd register these callbacks
    // controller.register_callback(GestureType::Tap, tap_callback).await;
    println!("   Callbacks registered (simulated)");
    println!();
    
    // Example 4: Get available presets
    println!("4️⃣  Available gesture presets:");
    let preset_manager = GesturePresetManager::new();
    let presets = preset_manager.get_presets().await;
    
    for preset in &presets {
        println!("   - {} ({})", preset.name, preset.description);
        println!("     Swipe sensitivity: {}", preset.swipe_sensitivity);
        println!("     Tap sensitivity: {}", preset.tap_sensitivity);
    }
    println!("   Total: {} presets", presets.len());
    println!();
    
    // Example 5: Apply a preset
    println!("5️⃣  Applying 'Sensitive' preset:");
    preset_manager.apply_preset("sensitive", &controller).await?;
    
    let sensitive_config = controller.get_config().await;
    println!("   Applied sensitivity: {}", sensitive_config.swipe_sensitivity);
    println!();
    
    // Example 6: Create custom preset
    println!("6️⃣  Creating custom preset:");
    let custom_preset = advanced_ui::gestures::GesturePreset {
        name: "My Custom".to_string(),
        description: "My custom gesture configuration".to_string(),
        swipe_sensitivity: 0.8,
        tap_sensitivity: 0.85,
        min_swipe_distance: 55,
        tap_duration_threshold: 320,
    };
    
    preset_manager.add_preset("my_custom".to_string(), custom_preset).await?;
    println!("   ✅ Custom preset added");
    
    let all_presets = preset_manager.get_presets().await;
    println!("   Total presets now: {}", all_presets.len());
    println!();
    
    // Example 7: Apply custom preset
    println!("7️⃣  Applying custom preset:");
    preset_manager.apply_preset("my_custom", &controller).await?;
    
    let custom_config = controller.get_config().await;
    println!("   Applied custom sensitivity: {}", custom_config.swipe_sensitivity);
    println!();
    
    // Example 8: Export configuration
    println!("8️⃣  Exporting configuration to JSON:");
    let json = controller.export_config().await?;
    println!("   JSON length: {} characters", json.len());
    println!("   Preview: {}", &json[..150]);
    println!();
    
    // Example 9: Import configuration
    println!("9️⃣  Importing configuration from JSON:");
    let imported_config = controller.import_config(&json).await?;
    println!("   ✅ Configuration imported successfully");
    println!();
    
    // Example 10: Export preset
    println!("🔟  Exporting preset to JSON:");
    let preset_json = preset_manager.export_preset("normal").await?;
    println!("   JSON length: {} characters", preset_json.len());
    println!("   Preview: {}", &preset_json[..150]);
    println!();
    
    // Example 11: Import preset
    println!("1️⃣1️⃣  Importing preset from JSON:");
    let imported_preset = preset_manager.import_preset("imported_normal".to_string(), &preset_json).await?;
    println!("   ✅ Preset imported successfully");
    println!();
    
    // Example 12: Modify configuration
    println!("1️⃣2️⃣  Modifying gesture configuration:");
    let mut modified_config = controller.get_config().await;
    modified_config.swipe_sensitivity = 0.85;
    modified_config.tap_sensitivity = 0.9;
    modified_config.enable_haptics = true;
    modified_config.enable_sound_feedback = true;
    
    controller.set_config(modified_config).await?;
    println!("   ✅ Configuration modified");
    
    let updated_config = controller.get_config().await;
    println!("   New swipe sensitivity: {}", updated_config.swipe_sensitivity);
    println!("   Haptics enabled: {}", updated_config.enable_haptics);
    println!();
    
    // Example 13: Remove preset
    println!("1️⃣3️⃣  Removing custom preset:");
    preset_manager.remove_preset("my_custom").await?;
    println!("   ✅ Custom preset removed");
    
    let remaining_presets = preset_manager.get_presets().await;
    println!("   Remaining presets: {}", remaining_presets.len());
    println!();
    
    // Example 14: Test gesture recognition
    println!("1️⃣4️⃣  Testing gesture recognition (simulated):");
    println!("   Simulating swipe right gesture...");
    
    // In real implementation, you'd simulate touch events:
    // controller.handle_touch_start(100.0, 100.0, 0).await?;
    // controller.handle_touch_move(200.0, 100.0, 100).await?;
    // controller.handle_touch_end(300.0, 100.0, 200).await?;
    
    println!("   ✅ Gesture would be recognized and callback triggered");
    println!();
    
    // Example 15: Multi-touch gesture
    println!("1️⃣5️⃣  Multi-touch gesture support:");
    let multi_touch_config = GestureConfig {
        enable_multi_touch: true,
        max_touches: 5,
        ..Default::default()
    };
    
    let multi_controller = GestureController::new(multi_touch_config)?;
    let mt_config = multi_controller.get_config().await;
    println!("   Multi-touch enabled: {}", mt_config.enable_multi_touch);
    println!("   Max touches: {}", mt_config.max_touches);
    println!();
    
    println!("✅ Gesture customization example completed successfully!");
    
    Ok(())
}