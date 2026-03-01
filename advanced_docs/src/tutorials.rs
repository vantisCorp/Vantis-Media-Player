//! Interactive Tutorials
//! 
//! Provides interactive tutorials with step-by-step guidance,
//! code examples, and hands-on exercises.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, error};

/// Tutorial manager
pub struct TutorialManager {
    /// Tutorials directory
    tutorials_dir: PathBuf,
    
    /// Loaded tutorials
    tutorials: HashMap<String, Tutorial>,
    
    /// User progress
    progress: HashMap<String, TutorialProgress>,
}

/// Tutorial
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tutorial {
    /// Tutorial ID
    pub id: String,
    
    /// Tutorial title
    pub title: String,
    
    /// Tutorial description
    pub description: String,
    
    /// Tutorial category
    pub category: TutorialCategory,
    
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    
    /// Estimated duration in minutes
    pub duration: u32,
    
    /// Tutorial steps
    pub steps: Vec<TutorialStep>,
    
    /// Prerequisites
    pub prerequisites: Vec<String>,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Author
    pub author: String,
    
    /// Last updated
    pub last_updated: String,
}

/// Tutorial category
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TutorialCategory {
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
    
    /// Troubleshooting
    Troubleshooting,
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

/// Tutorial step
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TutorialStep {
    /// Step number
    pub number: u32,
    
    /// Step title
    pub title: String,
    
    /// Step content
    pub content: String,
    
    /// Code example (optional)
    pub code_example: Option<CodeExample>,
    
    /// Exercise (optional)
    pub exercise: Option<Exercise>,
    
    /// Expected output (optional)
    pub expected_output: Option<String>,
}

/// Code example
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeExample {
    /// Programming language
    pub language: String,
    
    /// Code
    pub code: String,
    
    /// Explanation
    pub explanation: String,
}

/// Exercise
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exercise {
    /// Exercise description
    pub description: String,
    
    /// Exercise type
    pub exercise_type: ExerciseType,
    
    /// Initial code (optional)
    pub initial_code: Option<String>,
    
    /// Solution (optional)
    pub solution: Option<String>,
    
    /// Hints
    pub hints: Vec<String>,
}

/// Exercise type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExerciseType {
    /// Fill in the blank
    FillInTheBlank,
    
    /// Multiple choice
    MultipleChoice,
    
    /// Code completion
    CodeCompletion,
    
    /// Debugging
    Debugging,
    
    /// Implementation
    Implementation,
}

/// Tutorial progress
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TutorialProgress {
    /// Tutorial ID
    pub tutorial_id: String,
    
    /// Current step
    pub current_step: u32,
    
    /// Completed steps
    pub completed_steps: Vec<u32>,
    
    /// Started at
    pub started_at: String,
    
    /// Last accessed at
    pub last_accessed_at: String,
    
    /// Completed at (optional)
    pub completed_at: Option<String>,
    
    /// Time spent in seconds
    pub time_spent: u64,
}

impl TutorialManager {
    /// Create a new tutorial manager
    pub fn new(tutorials_dir: PathBuf) -> Result<Self> {
        info!("📖 Initializing Tutorial Manager");
        
        // Create tutorials directory if it doesn't exist
        std::fs::create_dir_all(&tutorials_dir)?;
        
        // Load tutorials
        let tutorials = Self::load_tutorials(&tutorials_dir)?;
        
        info!("✅ Tutorial manager initialized");
        info!("   - Tutorials directory: {}", tutorials_dir.display());
        info!("   - Loaded {} tutorials", tutorials.len());
        
        Ok(Self {
            tutorials_dir,
            tutorials,
            progress: HashMap::new(),
        })
    }
    
    /// Load tutorials from directory
    fn load_tutorials(dir: &Path) -> Result<HashMap<String, Tutorial>> {
        let mut tutorials = HashMap::new();
        
        if !dir.exists() {
            return Ok(tutorials);
        }
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                let content = std::fs::read_to_string(&path)?;
                let tutorial: Tutorial = serde_json::from_str(&content)?;
                tutorials.insert(tutorial.id.clone(), tutorial);
            }
        }
        
        Ok(tutorials)
    }
    
    /// Generate tutorials
    pub async fn generate(&self) -> Result<()> {
        info!("📖 Generating tutorials");
        
        // Generate HTML for each tutorial
        for (id, tutorial) in &self.tutorials {
            self.generate_tutorial_html(tutorial)?;
            debug!("✅ Generated tutorial: {}", id);
        }
        
        // Generate index page
        self.generate_index()?;
        
        info!("✅ Tutorials generated successfully");
        
        Ok(())
    }
    
    /// Generate tutorial HTML
    fn generate_tutorial_html(&self, tutorial: &Tutorial) -> Result<()> {
        let output_path = self.tutorials_dir.join(format!("{}.html", tutorial.id));
        
        // Generate HTML content
        let html = self.render_tutorial(tutorial)?;
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Render tutorial to HTML
    fn render_tutorial(&self, tutorial: &Tutorial) -> Result<String> {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>");
        html.push_str(&tutorial.title);
        html.push_str("</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/tutorials.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;tutorial-container&quot;>\n");
        html.push_str("    <h1>");
        html.push_str(&tutorial.title);
        html.push_str("</h1>\n");
        html.push_str("    <p class=&quot;description&quot;>");
        html.push_str(&tutorial.description);
        html.push_str("</p>\n");
        
        // Render steps
        for step in &tutorial.steps {
            html.push_str("    <div class=&quot;step&quot;>\n");
            html.push_str("      <h2>Step ");
            html.push_str(&step.number.to_string());
            html.push_str(": ");
            html.push_str(&step.title);
            html.push_str("</h2>\n");
            html.push_str("      <div class=&quot;content&quot;>\n");
            html.push_str(&step.content);
            html.push_str("      </div>\n");
            
            // Render code example if present
            if let Some(code_example) = &step.code_example {
                html.push_str("      <div class=&quot;code-example&quot;>\n");
                html.push_str("        <pre><code class=&quot;language-");
                html.push_str(&code_example.language);
                html.push_str("&quot;>");
                html.push_str(&html_escape(&code_example.code));
                html.push_str("</code></pre>\n");
                html.push_str("        <p class=&quot;explanation&quot;>");
                html.push_str(&code_example.explanation);
                html.push_str("</p>\n");
                html.push_str("      </div>\n");
            }
            
            // Render exercise if present
            if let Some(exercise) = &step.exercise {
                html.push_str("      <div class=&quot;exercise&quot;>\n");
                html.push_str("        <h3>Exercise</h3>\n");
                html.push_str("        <p>");
                html.push_str(&exercise.description);
                html.push_str("</p>\n");
                
                if !exercise.hints.is_empty() {
                    html.push_str("        <div class=&quot;hints&quot;>\n");
                    html.push_str("          <h4>Hints:</h4>\n");
                    for hint in &exercise.hints {
                        html.push_str("          <p>");
                        html.push_str(hint);
                        html.push_str("</p>\n");
                    }
                    html.push_str("        </div>\n");
                }
                
                html.push_str("      </div>\n");
            }
            
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/tutorials.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        Ok(html)
    }
    
    /// Generate index page
    fn generate_index(&self) -> Result<()> {
        let output_path = self.tutorials_dir.join("index.html");
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>Tutorials - Vantis Media Player</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/tutorials.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;tutorials-index&quot;>\n");
        html.push_str("    <h1>Tutorials</h1>\n");
        
        // Group by category
        let mut by_category: HashMap<TutorialCategory, Vec<&Tutorial>> = HashMap::new();
        for tutorial in self.tutorials.values() {
            by_category.entry(tutorial.category.clone()).or_default().push(tutorial);
        }
        
        // Render categories
        for (category, tutorials) in by_category {
            html.push_str("    <div class=&quot;category&quot;>\n");
            html.push_str("      <h2>");
            html.push_str(&format!("{:?}", category));
            html.push_str("</h2>\n");
            
            for tutorial in tutorials {
                html.push_str("      <div class=&quot;tutorial-card&quot;>\n");
                html.push_str("        <h3><a href=&quot;");
                html.push_str(&tutorial.id);
                html.push_str(".html&quot;>");
                html.push_str(&tutorial.title);
                html.push_str("</a></h3>\n");
                html.push_str("        <p>");
                html.push_str(&tutorial.description);
                html.push_str("</p>\n");
                html.push_str("        <div class=&quot;meta&quot;>\n");
                html.push_str("          <span class=&quot;difficulty&quot;>");
                html.push_str(&format!("{:?}", tutorial.difficulty));
                html.push_str("</span>\n");
                html.push_str("          <span class=&quot;duration&quot;>");
                html.push_str(&format!("{} min", tutorial.duration));
                html.push_str("</span>\n");
                html.push_str("        </div>\n");
                html.push_str("      </div>\n");
            }
            
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Get tutorial by ID
    pub fn get_tutorial(&self, id: &str) -> Option<&Tutorial> {
        self.tutorials.get(id)
    }
    
    /// Get all tutorials
    pub fn get_all_tutorials(&self) -> Vec<&Tutorial> {
        self.tutorials.values().collect()
    }
    
    /// Get tutorials by category
    pub fn get_tutorials_by_category(&self, category: TutorialCategory) -> Vec<&Tutorial> {
        self.tutorials
            .values()
            .filter(|t| t.category == category)
            .collect()
    }
    
    /// Get tutorials by difficulty
    pub fn get_tutorials_by_difficulty(&self, difficulty: DifficultyLevel) -> Vec<&Tutorial> {
        self.tutorials
            .values()
            .filter(|t| t.difficulty == difficulty)
            .collect()
    }
    
    /// Update progress
    pub fn update_progress(&mut self, tutorial_id: &str, step: u32) -> Result<()> {
        let progress = self.progress.entry(tutorial_id.to_string()).or_insert_with(|| {
            TutorialProgress {
                tutorial_id: tutorial_id.to_string(),
                current_step: 0,
                completed_steps: Vec::new(),
                started_at: chrono::Utc::now().to_rfc3339(),
                last_accessed_at: chrono::Utc::now().to_rfc3339(),
                completed_at: None,
                time_spent: 0,
            }
        });
        
        progress.current_step = step;
        progress.last_accessed_at = chrono::Utc::now().to_rfc3339();
        
        if !progress.completed_steps.contains(&step) {
            progress.completed_steps.push(step);
        }
        
        // Check if tutorial is complete
        if let Some(tutorial) = self.tutorials.get(tutorial_id) {
            if progress.completed_steps.len() == tutorial.steps.len() {
                progress.completed_at = Some(chrono::Utc::now().to_rfc3339());
            }
        }
        
        Ok(())
    }
    
    /// Get progress
    pub fn get_progress(&self, tutorial_id: &str) -> Option<&TutorialProgress> {
        self.progress.get(tutorial_id)
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
    fn test_tutorial_manager_creation() {
        let dir = PathBuf::from("./tutorials");
        let manager = TutorialManager::new(dir);
        assert!(manager.is_ok());
    }
}