//! TTML (Timed Text Markup Language) Parser
//!
//! Parses TTML and IMSC (IMSC1/IMSC1.1) subtitle formats with styling support.

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use tracing::{debug, warn};

use crate::parser::SubtitleEntry;

/// TTML namespace
const TTML_NS: &str = "http://www.w3.org/ns/ttml";

/// TTML region
#[derive(Debug, Clone)]
pub struct TtmlRegion {
    pub id: String,
    pub origin: String,
    pub extent: String,
    pub display_align: String,
}

/// TTML style
#[derive(Debug, Clone)]
pub struct TtmlStyle {
    pub id: String,
    pub properties: HashMap<String, String>,
}

/// TTML subtitle entry with styling
#[derive(Debug, Clone)]
pub struct TtmlSubtitleEntry {
    pub base: SubtitleEntry,
    pub style: Option<String>,
    pub region: Option<String>,
    pub styling: HashMap<String, String>,
}

/// TTML parser
pub struct TtmlParser;

impl TtmlParser {
    /// Parse TTML content
    pub fn parse(content: &str) -> Result<Vec<TtmlSubtitleEntry>> {
        debug!("📄 Parsing TTML content");
        
        // Parse XML structure
        let document = roxmltree::Document::parse(content)
            .map_err(|e| anyhow!("Failed to parse TTML XML: {}", e))?;
        
        let root = document.root_element();
        
        // Extract styles
        let styles = Self::extract_styles(&document)?;
        
        // Extract regions
        let regions = Self::extract_regions(&document)?;
        
        // Extract subtitles
        let entries = Self::extract_subtitles(&document, &styles, &regions)?;
        
        debug!("✅ Parsed {} TTML entries with styling", entries.len());
        
        Ok(entries)
    }
    
    /// Parse IMSC (IMSC1/IMSC1.1) content
    pub fn parse_imsc(content: &str) -> Result<Vec<TtmlSubtitleEntry>> {
        debug!("📄 Parsing IMSC content");
        
        // IMSC is a profile of TTML, so we use the same parser
        // but with additional validation for IMSC features
        let entries = Self::parse(content)?;
        
        // Validate IMSC-specific constraints
        Self::validate_imsc(&entries)?;
        
        debug!("✅ Parsed {} IMSC entries", entries.len());
        
        Ok(entries)
    }
    
    /// Extract styles from TTML document
    fn extract_styles(document: &roxmltree::Document) -> Result<HashMap<String, TtmlStyle>> {
        let mut styles = HashMap::new();
        
        for node in document.descendants() {
            if node.tag_name().name() == "style" {
                if let Some(id) = node.attribute("id") {
                    let mut properties = HashMap::new();
                    
                    // Extract style properties
                    for (name, value) in node.attributes() {
                        if name != "id" && name != "xml:id" {
                            properties.insert(name.to_string(), value.to_string());
                        }
                    }
                    
                    styles.insert(id.to_string(), TtmlStyle {
                        id: id.to_string(),
                        properties,
                    });
                }
            }
        }
        
        Ok(styles)
    }
    
    /// Extract regions from TTML document
    fn extract_regions(document: &roxmltree::Document) -> Result<HashMap<String, TtmlRegion>> {
        let mut regions = HashMap::new();
        
        for node in document.descendants() {
            if node.tag_name().name() == "region" {
                if let Some(id) = node.attribute("id") {
                    let region = TtmlRegion {
                        id: id.to_string(),
                        origin: node.attribute("origin").unwrap_or("auto").to_string(),
                        extent: node.attribute("extent").unwrap_or("auto").to_string(),
                        display_align: node.attribute("displayAlign")
                            .or_else(|| node.attribute("display-align"))
                            .unwrap_or("auto").to_string(),
                    };
                    
                    regions.insert(id.to_string(), region);
                }
            }
        }
        
        Ok(regions)
    }
    
    /// Extract subtitles from TTML document
    fn extract_subtitles(
        document: &roxmltree::Document,
        styles: &HashMap<String, TtmlStyle>,
        _regions: &HashMap<String, TtmlRegion>,
    ) -> Result<Vec<TtmlSubtitleEntry>> {
        let mut entries = Vec::new();
        
        for node in document.descendants() {
            if node.tag_name().name() == "p" {
                // Parse timing
                let (begin, end) = Self::parse_timing(&node)?;
                
                // Extract text content
                let text = Self::extract_text(&node);
                
                // Extract style
                let style = node.attribute("style")
                    .or_else(|| node.attribute("xml:style"))
                    .map(|s| s.to_string());
                
                // Extract region
                let region = node.attribute("region")
                    .or_else(|| node.attribute("xml:region"))
                    .map(|s| s.to_string());
                
                // Collect inline styling
                let mut styling = HashMap::new();
                
                // Apply style properties
                if let Some(style_id) = &style {
                    if let Some(style_def) = styles.get(style_id) {
                        styling.extend(style_def.properties.clone());
                    }
                }
                
                // Extract inline properties
                for (name, value) in node.attributes() {
                    if name != "style" && name != "xml:style" && 
                       name != "region" && name != "xml:region" &&
                       name != "begin" && name != "end" {
                        styling.insert(name.to_string(), value.to_string());
                    }
                }
                
                // Create base subtitle entry
                let base = SubtitleEntry {
                    start_ms: begin,
                    end_ms: end,
                    text,
                };
                
                entries.push(TtmlSubtitleEntry {
                    base,
                    style,
                    region,
                    styling,
                });
            }
        }
        
        // Sort entries by start time
        entries.sort_by(|a, b| a.base.start_ms.cmp(&b.base.start_ms));
        
        Ok(entries)
    }
    
    /// Parse timing from TTML element
    fn parse_timing(node: &roxmltree::Node) -> Result<(u64, u64)> {
        let begin = node.attribute("begin")
            .ok_or_else(|| anyhow!("Missing 'begin' attribute"))?;
        
        let end = node.attribute("end")
            .ok_or_else(|| anyhow!("Missing 'end' attribute"))?;
        
        let start_ms = Self::parse_ttml_timestamp(begin)?;
        let end_ms = Self::parse_ttml_timestamp(end)?;
        
        Ok((start_ms, end_ms))
    }
    
    /// Parse TTML timestamp (e.g., "00:00:01.000" -> 1000ms)
    fn parse_ttml_timestamp(ts: &str) -> Result<u64> {
        let ts = ts.trim();
        
        // Handle fractional seconds
        if ts.contains('.') {
            let parts: Vec<&str> = ts.split('.').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid timestamp format"));
            }
            
            let time_part = parts[0];
            let fraction = parts[1];
            
            let mut time_parts: Vec<&str> = time_part.split(':').collect();
            
            // Add hours if missing
            if time_parts.len() == 2 {
                time_parts.insert(0, "0");
            }
            
            if time_parts.len() != 3 {
                return Err(anyhow!("Invalid timestamp format"));
            }
            
            let hours: u64 = time_parts[0].parse()?;
            let minutes: u64 = time_parts[1].parse()?;
            let seconds: u64 = time_parts[2].parse()?;
            let milliseconds: u64 = format!("{}.{:0<3}", fraction, "").parse::<f64>()? as u64;
            
            let total_ms = hours * 3600000 + minutes * 60000 + seconds * 1000 + milliseconds;
            Ok(total_ms)
        } else {
            // Handle frame-based timing (e.g., "10f" for 10 frames)
            if ts.ends_with('f') {
                let frames: u64 = ts.trim_end_matches('f').parse()?;
                // Assume 30fps for now
                Ok(frames * 1000 / 30)
            } else {
                // Handle time-based timing without fraction
                let mut time_parts: Vec<&str> = ts.split(':').collect();
                
                if time_parts.len() == 2 {
                    time_parts.insert(0, "0");
                }
                
                if time_parts.len() != 3 {
                    return Err(anyhow!("Invalid timestamp format"));
                }
                
                let hours: u64 = time_parts[0].parse()?;
                let minutes: u64 = time_parts[1].parse()?;
                let seconds: u64 = time_parts[2].parse()?;
                
                let total_ms = hours * 3600000 + minutes * 60000 + seconds * 1000;
                Ok(total_ms)
            }
        }
    }
    
    /// Extract text content from TTML element
    fn extract_text(node: &roxmltree::Node) -> String {
        let mut text = String::new();
        
        for child in node.children() {
            if child.is_text() {
                text.push_str(child.text().unwrap_or(""));
            } else if child.tag_name().name() == "br" {
                text.push('\n');
            } else if child.tag_name().name() == "span" {
                // Recursively extract text from spans
                text.push_str(&Self::extract_text(&child));
            }
        }
        
        text.trim().to_string()
    }
    
    /// Validate IMSC-specific constraints
    fn validate_imsc(entries: &[TtmlSubtitleEntry]) -> Result<()> {
        // IMSC has specific constraints on styling and features
        // This is a basic validation
        
        for entry in entries {
            // Check for IMSC-unsupported features
            for (prop, value) in &entry.styling {
                if prop == "writingMode" && !value.starts_with("lrtb") && !value.starts_with("rltb") {
                    warn!("IMSC may not support writingMode: {}", value);
                }
            }
        }
        
        Ok(())
    }
    
    /// Convert TTML styling to CSS
    pub fn styling_to_css(styling: &HashMap<String, String>) -> String {
        let mut css = String::new();
        
        let mut props: Vec<(&str, &str)> = styling.iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        
        props.sort();
        
        for (prop, value) in props {
            css.push_str(&format!("  {}: {};\n", prop, value));
        }
        
        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_ttml_timestamp() {
        let ts = "00:00:01.000";
        let result = TtmlParser::parse_ttml_timestamp(ts).unwrap();
        assert_eq!(result, 1000);
        
        let ts = "00:01:30.500";
        let result = TtmlParser::parse_ttml_timestamp(ts).unwrap();
        assert_eq!(result, 90500);
    }
    
    #[test]
    fn test_styling_to_css() {
        let mut styling = HashMap::new();
        styling.insert("color".to_string(), "#ffffff".to_string());
        styling.insert("fontSize".to_string(), "24px".to_string());
        
        let css = TtmlParser::styling_to_css(&styling);
        assert!(css.contains("color: #ffffff;"));
        assert!(css.contains("fontSize: 24px;"));
    }
}