//! Troubleshooting Wizard
//! 
//! Provides an interactive troubleshooting wizard to help users
//! diagnose and resolve common issues.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, error};

/// Troubleshooting wizard
pub struct TroubleshootingWizard {
    /// Troubleshooting directory
    troubleshooting_dir: PathBuf,
    
    /// Troubleshooting guides
    guides: HashMap<String, TroubleshootingGuide>,
    
    /// Solutions database
    solutions: HashMap<String, Solution>,
}

/// Troubleshooting guide
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TroubleshootingGuide {
    /// Guide ID
    pub id: String,
    
    /// Guide title
    pub title: String,
    
    /// Guide description
    pub description: String,
    
    /// Guide category
    pub category: TroubleshootingCategory,
    
    /// Symptoms
    pub symptoms: Vec<String>,
    
    /// Questions for diagnosis
    pub questions: Vec<DiagnosticQuestion>,
    
    /// Possible solutions
    pub solutions: Vec<String>,
}

/// Troubleshooting category
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TroubleshootingCategory {
    /// Installation issues
    Installation,
    
    /// Playback issues
    Playback,
    
    /// Audio issues
    Audio,
    
    /// Video issues
    Video,
    
    /// Subtitle issues
    Subtitles,
    
    /// Plugin issues
    Plugins,
    
    /// Performance issues
    Performance,
    
    /// Network issues
    Network,
}

/// Diagnostic question
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiagnosticQuestion {
    /// Question ID
    pub id: String,
    
    /// Question text
    pub question: String,
    
    /// Question type
    pub question_type: QuestionType,
    
    /// Options (for multiple choice)
    pub options: Vec<QuestionOption>,
    
    /// Next question mapping
    pub next_questions: HashMap<String, String>,
}

/// Question type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    /// Yes/No question
    YesNo,
    
    /// Multiple choice
    MultipleChoice,
    
    /// Text input
    TextInput,
}

/// Question option
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionOption {
    /// Option value
    pub value: String,
    
    /// Option label
    pub label: String,
}

/// Solution
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Solution {
    /// Solution ID
    pub id: String,
    
    /// Solution title
    pub title: String,
    
    /// Solution description
    pub description: String,
    
    /// Steps to resolve
    pub steps: Vec<SolutionStep>,
    
    /// Severity
    pub severity: Severity,
    
    /// Estimated time to resolve
    pub estimated_time: String,
    
    /// Related guides
    pub related_guides: Vec<String>,
}

/// Solution step
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolutionStep {
    /// Step number
    pub number: u32,
    
    /// Step title
    pub title: String,
    
    /// Step description
    pub description: String,
    
    /// Code example (optional)
    pub code_example: Option<String>,
    
    /// Screenshot (optional)
    pub screenshot: Option<String>,
}

/// Severity level
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Low severity
    Low,
    
    /// Medium severity
    Medium,
    
    /// High severity
    High,
    
    /// Critical severity
    Critical,
}

impl TroubleshootingWizard {
    /// Create a new troubleshooting wizard
    pub fn new(troubleshooting_dir: PathBuf) -> Result<Self> {
        info!("🔧 Initializing Troubleshooting Wizard");
        
        // Create troubleshooting directory if it doesn't exist
        std::fs::create_dir_all(&troubleshooting_dir)?;
        
        // Load troubleshooting guides
        let guides = Self::load_guides(&troubleshooting_dir)?;
        
        // Load solutions
        let solutions = Self::load_solutions(&troubleshooting_dir)?;
        
        info!("✅ Troubleshooting wizard initialized");
        info!("   - Troubleshooting directory: {}", troubleshooting_dir.display());
        info!("   - Loaded {} guides", guides.len());
        info!("   - Loaded {} solutions", solutions.len());
        
        Ok(Self {
            troubleshooting_dir,
            guides,
            solutions,
        })
    }
    
    /// Load troubleshooting guides from directory
    fn load_guides(dir: &Path) -> Result<HashMap<String, TroubleshootingGuide>> {
        let mut guides = HashMap::new();
        
        if !dir.exists() {
            return Ok(guides);
        }
        
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                let content = std::fs::read_to_string(&path)?;
                let guide: TroubleshootingGuide = serde_json::from_str(&content)?;
                guides.insert(guide.id.clone(), guide);
            }
        }
        
        Ok(guides)
    }
    
    /// Load solutions from directory
    fn load_solutions(dir: &Path) -> Result<HashMap<String, Solution>> {
        let mut solutions = HashMap::new();
        
        let solutions_file = dir.join("solutions.json");
        if solutions_file.exists() {
            let content = std::fs::read_to_string(&solutions_file)?;
            let solution_list: Vec<Solution> = serde_json::from_str(&content)?;
            for solution in solution_list {
                solutions.insert(solution.id.clone(), solution);
            }
        }
        
        Ok(solutions)
    }
    
    /// Generate troubleshooting wizard
    pub async fn generate(&self) -> Result<()> {
        info!("🔧 Generating troubleshooting wizard");
        
        // Generate main wizard page
        self.generate_wizard_page()?;
        
        // Generate guide pages
        for (id, guide) in &self.guides {
            self.generate_guide_page(guide)?;
            debug!("✅ Generated guide page: {}", id);
        }
        
        // Generate solution pages
        for (id, solution) in &self.solutions {
            self.generate_solution_page(solution)?;
            debug!("✅ Generated solution page: {}", id);
        }
        
        info!("✅ Troubleshooting wizard generated successfully");
        
        Ok(())
    }
    
    /// Generate main wizard page
    fn generate_wizard_page(&self) -> Result<()> {
        let output_path = self.troubleshooting_dir.join("index.html");
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>Troubleshooting Wizard - Vantis Media Player</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/troubleshooting.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;troubleshooting-wizard&quot;>\n");
        html.push_str("    <h1>Troubleshooting Wizard</h1>\n");
        html.push_str("    <p>Interactive troubleshooting to help you resolve issues</p>\n");
        
        // Symptom search
        html.push_str("    <div class=&quot;symptom-search&quot;>\n");
        html.push_str("      <h2>Describe your problem</h2>\n");
        html.push_str("      <input type=&quot;text&quot; id=&quot;symptom-input&quot; placeholder=&quot;Describe the issue you're experiencing...&quot;>\n");
        html.push_str("      <button id=&quot;search-symptoms&quot;>Search</button>\n");
        html.push_str("    </div>\n");
        
        // Category selection
        html.push_str("    <div class=&quot;category-selection&quot;>\n");
        html.push_str("      <h2>Or select a category</h2>\n");
        
        for category in &[
            TroubleshootingCategory::Installation,
            TroubleshootingCategory::Playback,
            TroubleshootingCategory::Audio,
            TroubleshootingCategory::Video,
            TroubleshootingCategory::Subtitles,
            TroubleshootingCategory::Plugins,
            TroubleshootingCategory::Performance,
            TroubleshootingCategory::Network,
        ] {
            html.push_str("      <a href=&quot;category-");
            html.push_str(&format!("{:?}", category));
            html.push_str(".html&quot; class=&quot;category-card&quot;>\n");
            html.push_str("        <h3>");
            html.push_str(&format!("{:?}", category));
            html.push_str("</h3>\n");
            html.push_str("      </a>\n");
        }
        
        html.push_str("    </div>\n");
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/troubleshooting.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Generate guide page
    fn generate_guide_page(&self, guide: &TroubleshootingGuide) -> Result<()> {
        let output_path = self.troubleshooting_dir.join(format!("{}.html", guide.id));
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>");
        html.push_str(&guide.title);
        html.push_str(" - Troubleshooting</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/troubleshooting.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;guide-page&quot;>\n");
        html.push_str("    <a href=&quot;index.html&quot; class=&quot;back-link&quot;>← Back to Wizard</a>\n");
        html.push_str("    <h1>");
        html.push_str(&guide.title);
        html.push_str("</h1>\n");
        html.push_str("    <p class=&quot;description&quot;>");
        html.push_str(&guide.description);
        html.push_str("</p>\n");
        
        // Symptoms
        html.push_str("    <div class=&quot;symptoms-section&quot;>\n");
        html.push_str("      <h2>Common Symptoms</h2>\n");
        html.push_str("      <ul>\n");
        for symptom in &guide.symptoms {
            html.push_str("        <li>");
            html.push_str(symptom);
            html.push_str("</li>\n");
        }
        html.push_str("      </ul>\n");
        html.push_str("    </div>\n");
        
        // Diagnostic questions
        if !guide.questions.is_empty() {
            html.push_str("    <div class=&quot;diagnostic-section&quot;>\n");
            html.push_str("      <h2>Diagnostic Questions</h2>\n");
            
            for question in &guide.questions {
                html.push_str("      <div class=&quot;question&quot; data-question-id=&quot;");
                html.push_str(&question.id);
                html.push_str("&quot;>\n");
                html.push_str("        <p class=&quot;question-text&quot;>");
                html.push_str(&question.question);
                html.push_str("</p>\n");
                
                match question.question_type {
                    QuestionType::YesNo => {
                        html.push_str("        <div class=&quot;options&quot;>\n");
                        html.push_str("          <button class=&quot;option&quot; data-value=&quot;yes&quot;>Yes</button>\n");
                        html.push_str("          <button class=&quot;option&quot; data-value=&quot;no&quot;>No</button>\n");
                        html.push_str("        </div>\n");
                    }
                    QuestionType::MultipleChoice => {
                        html.push_str("        <div class=&quot;options&quot;>\n");
                        for option in &question.options {
                            html.push_str("          <button class=&quot;option&quot; data-value=&quot;");
                            html.push_str(&option.value);
                            html.push_str("&quot;>");
                            html.push_str(&option.label);
                            html.push_str("</button>\n");
                        }
                        html.push_str("        </div>\n");
                    }
                    QuestionType::TextInput => {
                        html.push_str("        <input type=&quot;text&quot; class=&quot;text-input&quot; placeholder=&quot;Enter your answer&quot;>\n");
                    }
                }
                
                html.push_str("      </div>\n");
            }
            
            html.push_str("    </div>\n");
        }
        
        // Possible solutions
        if !guide.solutions.is_empty() {
            html.push_str("    <div class=&quot;solutions-section&quot;>\n");
            html.push_str("      <h2>Possible Solutions</h2>\n");
            
            for solution_id in &guide.solutions {
                if let Some(solution) = self.solutions.get(solution_id) {
                    html.push_str("      <div class=&quot;solution-card&quot;>\n");
                    html.push_str("        <a href=&quot;solution-");
                    html.push_str(solution_id);
                    html.push_str(".html&quot;>\n");
                    html.push_str("          <h3>");
                    html.push_str(&solution.title);
                    html.push_str("</h3>\n");
                    html.push_str("          <p>");
                    html.push_str(&solution.description);
                    html.push_str("</p>\n");
                    html.push_str("          <span class=&quot;severity ");
                    html.push_str(&format!("{:?}", solution.severity).to_lowercase());
                    html.push_str("&quot;>");
                    html.push_str(&format!("{:?}", solution.severity));
                    html.push_str("</span>\n");
                    html.push_str("        </a>\n");
                    html.push_str("      </div>\n");
                }
            }
            
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("  <script src=&quot;/static/js/troubleshooting.js&quot;></script>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Generate solution page
    fn generate_solution_page(&self, solution: &Solution) -> Result<()> {
        let output_path = self.troubleshooting_dir.join(format!("solution-{}.html", solution.id));
        
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n");
        html.push_str("<head>\n");
        html.push_str("  <title>");
        html.push_str(&solution.title);
        html.push_str(" - Solution</title>\n");
        html.push_str("  <link rel=&quot;stylesheet&quot; href=&quot;/static/css/troubleshooting.css&quot;>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("  <div class=&quot;solution-page&quot;>\n");
        html.push_str("    <a href=&quot;index.html&quot; class=&quot;back-link&quot;>← Back to Wizard</a>\n");
        html.push_str("    <h1>");
        html.push_str(&solution.title);
        html.push_str("</h1>\n");
        html.push_str("    <p class=&quot;description&quot;>");
        html.push_str(&solution.description);
        html.push_str("</p>\n");
        
        // Solution metadata
        html.push_str("    <div class=&quot;solution-meta&quot;>\n");
        html.push_str("      <span class=&quot;severity ");
        html.push_str(&format!("{:?}", solution.severity).to_lowercase());
        html.push_str("&quot;>");
        html.push_str(&format!("{:?}", solution.severity));
        html.push_str("</span>\n");
        html.push_str("      <span class=&quot;estimated-time&quot;>");
        html.push_str(&solution.estimated_time);
        html.push_str("</span>\n");
        html.push_str("    </div>\n");
        
        // Steps
        html.push_str("    <div class=&quot;steps-section&quot;>\n");
        html.push_str("      <h2>Steps to Resolve</h2>\n");
        
        for step in &solution.steps {
            html.push_str("      <div class=&quot;step&quot;>\n");
            html.push_str("        <h3>Step ");
            html.push_str(&step.number.to_string());
            html.push_str(": ");
            html.push_str(&step.title);
            html.push_str("</h3>\n");
            html.push_str("        <p>");
            html.push_str(&step.description);
            html.push_str("</p>\n");
            
            if let Some(code) = &step.code_example {
                html.push_str("        <pre><code>");
                html.push_str(&html_escape(code));
                html.push_str("</code></pre>\n");
            }
            
            if let Some(screenshot) = &step.screenshot {
                html.push_str("        <img src=&quot;");
                html.push_str(screenshot);
                html.push_str("&quot; alt=&quot;Screenshot&quot;>\n");
            }
            
            html.push_str("      </div>\n");
        }
        
        html.push_str("    </div>\n");
        
        // Related guides
        if !solution.related_guides.is_empty() {
            html.push_str("    <div class=&quot;related-guides&quot;>\n");
            html.push_str("      <h2>Related Guides</h2>\n");
            
            for guide_id in &solution.related_guides {
                if let Some(guide) = self.guides.get(guide_id) {
                    html.push_str("      <a href=&quot;");
                    html.push_str(guide_id);
                    html.push_str(".html&quot; class=&quot;related-guide&quot;>\n");
                    html.push_str("        ");
                    html.push_str(&guide.title);
                    html.push_str("\n");
                    html.push_str("      </a>\n");
                }
            }
            
            html.push_str("    </div>\n");
        }
        
        html.push_str("  </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        
        std::fs::write(&output_path, html)?;
        
        Ok(())
    }
    
    /// Get guide by ID
    pub fn get_guide(&self, id: &str) -> Option<&TroubleshootingGuide> {
        self.guides.get(id)
    }
    
    /// Get solution by ID
    pub fn get_solution(&self, id: &str) -> Option<&Solution> {
        self.solutions.get(id)
    }
    
    /// Search guides by symptom
    pub fn search_by_symptom(&self, symptom: &str) -> Vec<&TroubleshootingGuide> {
        let symptom_lower = symptom.to_lowercase();
        self.guides
            .values()
            .filter(|g| {
                g.symptoms
                    .iter()
                    .any(|s| s.to_lowercase().contains(&symptom_lower))
                    || g.description.to_lowercase().contains(&symptom_lower)
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
    fn test_troubleshooting_wizard_creation() {
        let dir = PathBuf::from("./troubleshooting");
        let wizard = TroubleshootingWizard::new(dir);
        assert!(wizard.is_ok());
    }
}