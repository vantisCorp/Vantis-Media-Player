//! Plugin Development Tools Example
//!
//! Demonstrates the plugin development tools including CLI, testing,
//! debugging, documentation generation, and templates.

use anyhow::Result;
use vanis_devtools::{PluginDevelopmentTools, DevToolsConfig};

fn main() -> Result<()> {
    println!("🛠️ Plugin Development Tools Example\n");
    
    // Create development tools with default configuration
    let config = DevToolsConfig::default();
    let devtools = PluginDevelopmentTools::new(config);
    
    println!("✅ Development tools initialized");
    println!("   Templates directory: {}", devtools.config.templates_dir);
    println!("   Default template: {}", devtools.config.default_template);
    
    // Example 1: Create a new plugin from template
    println!("\n📦 Example 1: Creating a New Plugin from Template");
    let plugin_name = "my_awesome_plugin";
    println!("   Creating plugin: {}", plugin_name);
    
    // Note: This would actually create files, so we'll just demonstrate
    println!("   Template options:");
    for template in devtools.get_templates() {
        println!("     - {}", template);
    }
    
    println!("\n   Example command:");
    println!("   $ vanis-plugin create {} --template basic", plugin_name);
    
    // Example 2: Build a plugin
    println!("\n🔨 Example 2: Building a Plugin");
    println!("   Building plugin: plugins/my_plugin");
    println!("   Example command:");
    println!("   $ vanis-plugin build plugins/my_plugin");
    
    // Example 3: Run tests
    println!("\n🧪 Example 3: Running Tests");
    println!("   Running tests for: plugins/my_plugin");
    println!("   Example command:");
    println!("   $ vanis-plugin test plugins/my_plugin");
    
    // Example 4: Generate documentation
    println!("\n📚 Example 4: Generating Documentation");
    println!("   Generating documentation for: plugins/my_plugin");
    println!("   Example command:");
    println!("   $ vanis-plugin doc plugins/my_plugin");
    
    // Example 5: Debug a plugin
    println!("\n🐛 Example 5: Debugging a Plugin");
    println!("   Starting debug session for: plugins/my_plugin");
    println!("   Example command:");
    println!("   $ vanis-plugin debug plugins/my_plugin -- --args");
    
    // Example 6: Analyze performance
    println!("\n⚡ Example 6: Performance Analysis");
    println!("   Analyzing performance for: plugins/my_plugin");
    println!("   Example command:");
    println!("   $ vanis-plugin bench plugins/my_plugin");
    
    // Example 7: Generate README
    println!("\n📝 Example 7: Generating README");
    println!("   Generating README from code comments");
    println!("   Example command:");
    println!("   $ vanis-plugin readme plugins/my_plugin");
    
    // Example 8: Get test coverage
    println!("\n📊 Example 8: Test Coverage");
    println!("   Calculating coverage for: plugins/my_plugin");
    println!("   Example command:");
    println!("   $ vanis-plugin coverage plugins/my_plugin");
    
    // Example 9: Complete workflow
    println!("\n🔄 Example 9: Complete Plugin Development Workflow");
    println!("   1. Create plugin from template");
    println!("      $ vanis-plugin create my_plugin --template advanced");
    println!("\n   2. Navigate to plugin directory");
    println!("      $ cd plugins/my_plugin");
    println!("\n   3. Implement your plugin logic");
    println!("      $ vim src/lib.rs");
    println!("\n   4. Build the plugin");
    println!("      $ vanis-plugin build .");
    println!("\n   5. Run tests");
    println!("      $ vanis-plugin test .");
    println!("\n   6. Check coverage");
    println!("      $ vanis-plugin coverage .");
    println!("\n   7. Generate documentation");
    println!("      $ vanis-plugin doc .");
    println!("\n   8. Generate README");
    println!("      $ vanis-plugin readme .");
    println!("\n   9. Debug if needed");
    println!("      $ vanis-plugin debug .");
    println!("\n  10. Analyze performance");
    println!("      $ vanis-plugin bench .");
    
    // Example 10: Available commands summary
    println!("\n📋 Example 10: Available Commands Summary");
    println!("   vanis-plugin <command> [options]");
    println!("\n   Commands:");
    println!("     create <name>           Create a new plugin from template");
    println!("     build <path>            Build a plugin");
    println!("     test <path>             Run plugin tests");
    println!("     doc <path>              Generate documentation");
    println!("     readme <path>           Generate README from comments");
    println!("     debug <path>            Start debug session");
    println!("     bench <path>            Analyze performance");
    println!("     coverage <path>         Calculate test coverage");
    println!("\n   Options:");
    println!("     --template <name>       Template to use (basic, advanced, minimal)");
    println!("     --output <dir>          Output directory for generated files");
    println!("     --verbose               Enable verbose output");
    
    println!("\n✅ All plugin development tools examples completed successfully!");
    println!("\nPlugin development tools provide:");
    println!("   • Plugin creation from templates");
    println!("   • Building and testing plugins");
    println!("   • Debugging support");
    println!("   • Documentation generation");
    println!("   • Performance analysis");
    println!("   • Test coverage calculation");
    println!("   • README generation");
    println!("   • Multiple template options");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_devtools_creation() {
        let devtools = PluginDevelopmentTools::default_config();
        assert_eq!(devtools.config.default_template, "basic");
    }
    
    #[test]
    fn test_available_templates() {
        let devtools = PluginDevelopmentTools::default_config();
        let templates = devtools.get_templates();
        assert!(templates.len() >= 3);
        assert!(templates.contains(&"basic".to_string()));
        assert!(templates.contains(&"advanced".to_string()));
        assert!(templates.contains(&"minimal".to_string()));
    }
}