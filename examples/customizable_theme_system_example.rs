//! Customizable Theme System Example
//!
//! This example demonstrates the customizable theme system features of Vantis Media Player:
//! - Theme specification format
//! - Theme presets
//! - Theme editor UI
//! - Theme import/export
//! - Theme validation

use std::path::PathBuf;
use vantisui::theme_system::{
    BorderRadius, ButtonStyles, CardStyles, ColorPalette, ComponentStyles, InputStyles,
    NavigationStyles, Shadows, Spacing, ThemeManager, ThemeSpecification, ThemeType,
    Typography,
};

fn main() -> anyhow::Result<()> {
    println!("=== Vantis Media Player - Customizable Theme System Example ===\n");

    // Example 1: Basic theme manager setup
    println!("Example 1: Basic Theme Manager Setup");
    basic_theme_manager_setup()?;

    // Example 2: Theme presets
    println!("\nExample 2: Theme Presets");
    theme_presets()?;

    // Example 3: Create custom theme
    println!("\nExample 3: Create Custom Theme");
    create_custom_theme()?;

    // Example 4: Theme validation
    println!("\nExample 4: Theme Validation");
    theme_validation()?;

    // Example 5: Theme import/export
    println!("\nExample 5: Theme Import/Export");
    theme_import_export()?;

    // Example 6: Theme customization
    println!("\nExample 6: Theme Customization");
    theme_customization()?;

    // Example 7: Complete workflow
    println!("\nExample 7: Complete Workflow");
    complete_workflow()?;

    println!("\n=== All Examples Completed Successfully ===");
    Ok(())
}

/// Example 1: Basic theme manager setup
fn basic_theme_manager_setup() -> anyhow::Result<()> {
    let manager = ThemeManager::new();

    println!("  Theme manager created");
    println!("    Current theme: {}", manager.current_theme().name);
    println!("    Available themes: {}", manager.available_themes().len());
    println!("    Presets: {}", manager.presets().len());

    Ok(())
}

/// Example 2: Theme presets
fn theme_presets() -> anyhow::Result<()> {
    let manager = ThemeManager::new();

    println!("  Available theme presets:");
    for preset in manager.presets() {
        println!("    - {} ({})", preset.name, preset.theme_type.as_str());
        println!("      Author: {}", preset.author);
        println!("      Description: {}", preset.description);
    }

    Ok(())
}

/// Example 3: Create custom theme
fn create_custom_theme() -> anyhow::Result<()> {
    let mut manager = ThemeManager::new();

    let custom_theme = ThemeSpecification {
        name: "My Custom Theme".to_string(),
        version: "1.0.0".to_string(),
        author: "User".to_string(),
        description: "A custom theme created by the user".to_string(),
        theme_type: ThemeType::Custom,
        colors: ColorPalette {
            primary: "#ff6b6b".to_string(),
            secondary: "#4ecdc4".to_string(),
            accent: "#ffe66d".to_string(),
            background: "#2d3436".to_string(),
            surface: "#636e72".to_string(),
            text: "#dfe6e9".to_string(),
            text_secondary: "#b2bec3".to_string(),
            border: "#74b9ff".to_string(),
            success: "#00b894".to_string(),
            warning: "#fdcb6e".to_string(),
            error: "#d63031".to_string(),
            info: "#0984e3".to_string(),
        },
        typography: Typography::default(),
        spacing: Spacing::default(),
        border_radius: BorderRadius::default(),
        shadows: Shadows::default(),
        components: ComponentStyles::default(),
        custom_css: Some("/* Custom CSS */".to_string()),
    };

    manager.add_theme(custom_theme);
    println!("  Custom theme 'My Custom Theme' added");
    println!("    Total themes: {}", manager.available_themes().len());

    Ok(())
}

/// Example 4: Theme validation
fn theme_validation() -> anyhow::Result<()> {
    let manager = ThemeManager::new();

    // Valid theme
    let valid_theme = ThemeSpecification::default();
    let result = manager.validate_theme(&valid_theme);
    println!("  Valid theme validation: {}", result.is_ok());

    // Invalid theme (empty name)
    let mut invalid_theme = ThemeSpecification::default();
    invalid_theme.name = String::new();
    let result = manager.validate_theme(&invalid_theme);
    println!("  Invalid theme (empty name) validation: {}", result.is_err());

    // Invalid theme (invalid color)
    let mut invalid_theme = ThemeSpecification::default();
    invalid_theme.colors.primary = "invalid".to_string();
    let result = manager.validate_theme(&invalid_theme);
    println!("  Invalid theme (invalid color) validation: {}", result.is_err());

    Ok(())
}

/// Example 5: Theme import/export
fn theme_import_export() -> anyhow::Result<()> {
    let mut manager = ThemeManager::new();

    // Export theme
    let export_path = PathBuf::from("/tmp/my_theme.json");
    manager.export_theme("Dark", &export_path)?;
    println!("  Theme exported to: {:?}", export_path);

    // Import theme
    let import_path = PathBuf::from("/tmp/my_theme.json");
    manager.import_theme(&import_path)?;
    println!("  Theme imported from: {:?}", import_path);

    Ok(())
}

/// Example 6: Theme customization
fn theme_customization() -> anyhow::Result<()> {
    let mut manager = ThemeManager::new();

    // Get current theme
    let current_theme = manager.current_theme().clone();
    println!("  Current theme: {}", current_theme.name);
    println!("    Primary color: {}", current_theme.colors.primary);
    println!("    Background color: {}", current_theme.colors.background);

    // Customize theme
    let mut customized_theme = current_theme;
    customized_theme.name = "Customized Dark".to_string();
    customized_theme.colors.primary = "#ff6b6b".to_string();
    customized_theme.colors.accent = "#4ecdc4".to_string();

    manager.add_theme(customized_theme);
    println!("\n  Customized theme added");
    println!("    New primary color: #ff6b6b");
    println!("    New accent color: #4ecdc4");

    Ok(())
}

/// Example 7: Complete workflow
fn complete_workflow() -> anyhow::Result<()> {
    let mut manager = ThemeManager::new();

    println!("  Step 1: Create custom theme");
    let custom_theme = ThemeSpecification {
        name: "Complete Workflow Theme".to_string(),
        version: "1.0.0".to_string(),
        author: "User".to_string(),
        description: "Theme created through complete workflow".to_string(),
        theme_type: ThemeType::Custom,
        colors: ColorPalette {
            primary: "#9b59b6".to_string(),
            secondary: "#3498db".to_string(),
            accent: "#e74c3c".to_string(),
            background: "#1a1a2e".to_string(),
            surface: "#16213e".to_string(),
            text: "#e94560".to_string(),
            text_secondary: "#0f3460".to_string(),
            border: "#533483".to_string(),
            success: "#2ecc71".to_string(),
            warning: "#f39c12".to_string(),
            error: "#e74c3c".to_string(),
            info: "#3498db".to_string(),
        },
        typography: Typography {
            font_family: "Roboto, sans-serif".to_string(),
            font_size_base: 16,
            font_size_small: 14,
            font_size_large: 20,
            font_weight_normal: 400,
            font_weight_medium: 500,
            font_weight_bold: 700,
            line_height: 1.6,
            letter_spacing: 0.5,
        },
        spacing: Spacing {
            unit: 4,
            small: 8,
            medium: 16,
            large: 24,
            extra_large: 32,
        },
        border_radius: BorderRadius {
            small: 4,
            medium: 8,
            large: 12,
            extra_large: 16,
            full: 9999,
        },
        shadows: Shadows {
            small: "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string(),
            medium: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
            large: "0 10px 15px -3px rgb(0 0 0 / 0.1)".to_string(),
            extra_large: "0 20px 25px -5px rgb(0 0 0 / 0.1)".to_string(),
        },
        components: ComponentStyles {
            button: ButtonStyles {
                primary_background: "#9b59b6".to_string(),
                primary_text: "#ffffff".to_string(),
                secondary_background: "#533483".to_string(),
                secondary_text: "#ffffff".to_string(),
                border_radius: 8,
                padding: "12px 24px".to_string(),
            },
            input: InputStyles {
                background: "#16213e".to_string(),
                border_color: "#533483".to_string(),
                border_color_focus: "#9b59b6".to_string(),
                text_color: "#e94560".to_string(),
                placeholder_color: "#0f3460".to_string(),
                border_radius: 8,
                padding: "12px 16px".to_string(),
            },
            card: CardStyles {
                background: "#16213e".to_string(),
                border_color: "#533483".to_string(),
                border_radius: 12,
                padding: "24px".to_string(),
                shadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)".to_string(),
            },
            navigation: NavigationStyles {
                background: "#1a1a2e".to_string(),
                text_color: "#0f3460".to_string(),
                active_text_color: "#9b59b6".to_string(),
                border_color: "#533483".to_string(),
                padding: "16px 24px".to_string(),
            },
        },
        custom_css: Some("/* Custom CSS for complete workflow */".to_string()),
    };

    println!("  Step 2: Validate theme");
    manager.validate_theme(&custom_theme)?;
    println!("    Theme validated successfully");

    println!("  Step 3: Add theme to manager");
    manager.add_theme(custom_theme);
    println!("    Theme added");

    println!("  Step 4: Set as current theme");
    manager.set_theme_by_name("Complete Workflow Theme")?;
    println!("    Theme set as current");

    println!("  Step 5: Export theme");
    let export_path = PathBuf::from("/tmp/complete_workflow_theme.json");
    manager.export_theme("Complete Workflow Theme", &export_path)?;
    println!("    Theme exported to: {:?}", export_path);

    println!("  Step 6: Remove theme");
    manager.remove_theme("Complete Workflow Theme")?;
    println!("    Theme removed");

    println!("  Step 7: Import theme back");
    manager.import_theme(&export_path)?;
    println!("    Theme imported back");

    println!("  Complete workflow finished successfully!");

    Ok(())
}