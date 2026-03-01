//! Examples Gallery
//! 
//! Provides a gallery of code examples organized by category,
//  with search, filtering, and interactive features.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, error};

/// Examples gallery
pub struct ExamplesGallery {
    /// Examples directory
    examples_dir: PathBuf,
    
    /// Loaded examples
    examples: HashMap<String, CodeExample>,
}

/// Code example
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeExample {
    /// Example ID
    pub id: String,
    
    /// Example title
    pub title: String,
    
    /// Example description
    pub description: String,
    
    /// Example category
    pub category: ExampleCategory,
    
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    
    /// Programming language
    pub language: String,
    
    /// Code
    pub code: String,
    
    /// Explanation
    pub explanation: String,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Related examples
    pub related_examples: Vec<String>,
    
    /// Author
    pub author: String,
    
    /// Last updated
    pub last_updated: String,
}

/// Example category
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExampleCategory {
    /// Getting started
    GettingStarted,
    
    /// Core concepts
    CoreConcepts,
    
    /// Video processing
    Video,
    
    /// Audio processing
    Audio,
    
    /// Subtitles
    Subtitles,
    
    /// Plugins
    Plugins,
    
    /// Advanced topics
    Advanced,
    
    /// Integration
    Integration,
}

/// Difficulty level
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DifficultyLevel {
    /// Beginner
    Beginner,
    
    /// Intermediate
    Intermediate,
    
    /// Advanced
    Advanced,
    
    /// Expert
    Expert,
}

impl ExamplesGallery {
    /// Create a new examples gallery
    pub fn new(examples_dir: PathBuf) -> Result<Self> {
        info!("📚 Initializing Examples Gallery");
        
        // Create examples directory if it doesn't exist
        std::fs::create_dir_all(&examples_dir)?;
        
        // Load examples
        let examples = Self::load_examples(&examples_dir)?;
        
        info!("✅ Examples gallery initialized");
        info!("   - Examples directory: {}", examples_dir.display());
        info!("   - Loaded {} examples", examples.len());
        
        Ok(Self {
            examples_dir,
            examples,
        })
    }
    
    /// Load examples from directory
    fn load_examples(dir: &Path) -> Result<HashMap<String, CodeExample>> {
        let mut examples = HashMap::new();
        
        if !dir.exists() {
            return Ok(examples);
        }
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                let content = std::fs::read_to_string(&path)?;
                let example: CodeExample = serde_json::from_str(&content)?;
                examples.insert(example.id.clone(), example);
            }
        }
        
        Ok(examples)
    }
    
    /// Generate examples gallery
    pub async fn generate(&self) -> Result<()> {
        info!("📚 Generating examples gallery");
        
        // Generate HTML for each example
        for (id, example) in &self.examples {
            self.generate_example_html(example)?;
            debug!("✅ Generated example: {}", id);
        }
        
        // Generate index page
        self.generate_index()?;
        
        info!("✅ Examples gallery generated successfully");
        
        Ok(())
    }
    
    /// Generate example HTML
    fn generate_example_html(&self, example: &CodeExample) -> Result<()> {
        let output_path = self.examples_dir.join(format!("{}.html", example.id));
        
        // Generate HTML content
        let html = self.render_example(example)?;
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Render example to HTML
    fn render_example(&self, example: &CodeExample) -> Result<String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>");
        html.push_str(&example.title);
        html.push_str("</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/examples.css&quot;>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/highlight.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;example-container&quot;>\n");
        html.push_str("    <a href=&quot;index.html&quot; class=&quot;back-link&quot;>← Back to Gallery</a>\n");
        html.push_str("    <h1>");
        html.push_str(&example.title);
        html.push_str("</h1>\n");
        html.push_str("    <p class=&quot;description&quot;>");
        html.push_str(&example.description);
        html.push_str("</p>\n");
        
        // Example metadata
        html.push_str("    <div class=&quot;example-meta&quot;>\n");
        html.push_str("      <span class=&quot;category&quot;>");
        html.push_str(&format!("{:?}", example.category));
        html.push_str("</span>\n");
        html.push_str("      <span class=&quot;difficulty&quot;>");
        html.push_str(&format!("{:?}", example.difficulty));
        html.push_str("</span>\n");
        html.push_str("      <span class=&quot;language&quot;>");
        html.push_str(&example.language);
        html.push_str("</span>\n");
        html.push_str("    </div>\n");
        
        // Tags
        if !example.tags.is_empty() {
            html.push_str("    <div class=&quot;tags&quot;>\n");
            for tag in &example.tags {
                html.push_str("      <span class=&quot;tag&quot;>");
                html.push_str(tag);
                html.push_str("</span>\n");
            }
            html.push_str("    </div>\n");
        }
        
        // Code section
        html.push_str("    <div class=&quot;code-section&quot;>\n");
        html.push_str("      <h2>Code</h2>\n");
        html.push_str("      <pre><code class=&quot;language-");
        html.push_str(&example.language);
        html.push_str("&quot;>");
        html.push_str(&html_escape(&example.code));
        html.push_str("</code></pre>\n");
        html.push_str("      <button class=&quot;copy-button&quot; data-code-id=&quot;");
        html.push_str(&example.id);
        html.push_str("&quot;>Copy Code</button>\n");
        html.push_str("    </div>\n");
        
        // Explanation section
        html.push_str("    <div class=&quot;explanation-section&quot;>\n");
        html.push_str("      <h2>Explanation</h2>\n");
        html.push_str("      <div class=&quot;explanation&quot;>\n");
        html.push_str(&example.explanation);
        html.push_str("      </div>\n");
        html.push_str("    </div>\n");
        
        // Related examples
        if !example.related_examples.is_empty() {
            html.push_str("    <div class=&quot;related-examples&quot;>\n");
            html.push_str("      <h2>Related Examples</h2>\n");
            for related_id in &example.related_examples {
                if let Some(related_example) = self.examples.get(related_id) {
                    html.push_str("      <div class=&quot;related-example&quot;>\n");
                    html.push_str("        <a href=&quot;");
                    html.push_str(related_id);
                    html.push_str(".html&quot;>");
                    html.push_str(&related_example.title);
                    html.push_str("</a>\n");
                    html.push_str("      </div>\n");
                }
            }
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/highlight.js&quot;></script>\n");
        html.push_str("  <script src=&quot;/static/js/examples.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        Ok(html)
    }
    
    /// Generate index page
    fn generate_index(&self) -> Result<()> {
        let output_path = self.examples_dir.join("index.html");
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>Examples Gallery - Vantis Media Player</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/examples.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;examples-gallery&quot;>\n");
        html.push_str("    <h1>Examples Gallery</h1>\n");
        
        // Search and filter
        html.push_str("    <div class=&quot;search-filter&quot;>\n");
        html.push_str("      <input type=&quot;text&quot; id=&quot;search&quot; placeholder=&quot;Search examples...&quot;>\n");
        html.push_str("      <select id=&quot;category-filter&quot;>\n");
        html.push_str("        <option value=&quot;&quot;>All Categories</option>\n");
        html.push_str("        <option value=&quot;GettingStarted&quot;>Getting Started</option>\n");
        html.push_str("        <option value=&quot;CoreConcepts&quot;>Core Concepts</option>\n");
        html.push_str("        <option value=&quot;Video&quot;>Video</option>\n");
        html.push_str("        <option value=&quot;Audio&quot;>Audio</option>\n");
        html.push_str("        <option value=&quot;Subtitles&quot;>Subtitles</option>\n");
        html.push_str("        <option value=&quot;Plugins&quot;>Plugins</option>\n");
        html.push_str("        <option value=&quot;Advanced&quot;>Advanced</option>\n");
        html.push_str("        <option value=&quot;Integration&quot;>Integration</option>\n");
        html.push_str("      </select>\n");
        html.push_str("      <select id=&quot;difficulty-filter&quot;>\n");
        html.push_str("        <option value=&quot;&quot;>All Difficulties</option>\n");
        html.push_str("        <option value=&quot;Beginner&quot;>Beginner</option>\n");
        html.push_str("        <option value=&quot;Intermediate&quot;>Intermediate</option>\n");
        html.push_str("        <option value=&quot;Advanced&quot;>Advanced</option>\n");
        html.push_str("        <option value=&quot;Expert&quot;>Expert</option>\n");
        html.push_str("      </select>\n");
        html.push_str("    </div>\n");
        
        // Group by category
        let mut by_category: HashMap<ExampleCategory, Vec<&CodeExample>> = HashMap::new();
        for example in self.examples.values() {
            by_category.entry(example.category.clone()).or_default().push(example);
        }
        
        // Render categories
        for (category, examples) in by_category {
            html.push_str("    <div class=&quot;category&quot; data-category=&quot;");
            html.push_str(&format!("{:?}", category));
            html.push_str("&quot;>\n");
            html.push_str("      <h2>");
            html.push_str(&format!("{:?}", category));
            html.push_str("</h2>\n");
            
            for example in examples {
                html.push_str("      <div class=&quot;example-card&quot; data-difficulty=&quot;");
                html.push_str(&format!("{:?}", example.difficulty));
                html.push_str("&quot;>\n");
                html.push_str("        <a href=&quot;");
                html.push_str(&example.id);
                html.push_str(".html&quot;>\n");
                html.push_str("          <h3>");
                html.push_str(&example.title);
                html.push_str("</h3>\n");
                html.push_str("          <p>");
                html.push_str(&example.description);
                html.push_str("</p>\n");
                html.push_str("          <div class=&quot;meta&quot;>\n");
                html.push_str("            <span class=&quot;difficulty&quot;>");
                html.push_str(&format!("{:?}", example.difficulty));
                html.push_str("</span>\n");
                html.push_str("            <span class=&quot;language&quot;>");
                html.push_str(&example.language);
                html.push_str("</span>\n");
                html.push_str("          </div>\n");
                html.push_str("        </a>\n");
                html.push_str("      </div>\n");
            }
            
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/examples.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Get example by ID
    pub fn get_example(&self, id: &str) -> Option<&CodeExample> {
        self.examples.get(id)
    }
    
    /// Get all examples
    pub fn get_all_examples(&self) -> Vec<&CodeExample> {
        self.examples.values().collect()
    }
    
    /// Get examples by category
    pub fn get_examples_by_category(&self, category: ExampleCategory) -> Vec<&CodeExample> {
        self.examples
            .values()
            .filter(|e| e.category == category)
            .collect()
    }
    
    /// Get examples by difficulty
    pub fn get_examples_by_difficulty(&self, difficulty: DifficultyLevel) -> Vec<&CodeExample> {
        self.examples
            .values()
            .filter(|e| e.difficulty == difficulty)
            .collect()
    }
    
    /// Search examples
    pub fn search_examples(&self, query: &str) -> Vec<&CodeExample> {
        let query_lower = query.to_lowercase();
        self.examples
            .values()
            .filter(|e| {
                e.title.to_lowercase().contains(&query_lower)
                    || e.description.to_lowercase().contains(&query_lower)
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_examples_gallery_creation() {
        let dir = PathBuf::from("./examples");
        let gallery = ExamplesGallery::new(dir);
        assert!(gallery.is_ok());
    }
}