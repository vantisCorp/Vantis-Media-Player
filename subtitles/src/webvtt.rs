//! Enhanced WebVTT Parser
//!
//! Parses WebVTT subtitle format with advanced styling support including
//! voice spans, classes, CSS positioning, and regions.

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use tracing::{debug};

use crate::parser::SubtitleEntry;

/// WebVTT cue with styling
#[derive(Debug, Clone)]
pub struct WebVttCue {
    pub base: SubtitleEntry,
    pub identifier: Option<String>,
    pub settings: Vec<WebVttSetting>,
    pub text: String,
    pub classes: Vec<String>,
    pub regions: Vec<String>,
}

/// WebVTT setting
#[derive(Debug, Clone, PartialEq)]
pub enum WebVttSetting {
    Vertical(VerticalDirection),
    Line { value: String, snap_to_lines: bool },
    Position { value: String, line_align: Option<LineAlign> },
    Size(u32),
    Align(TextAlign),
}

/// Vertical direction
#[derive(Debug, Clone, PartialEq)]
pub enum VerticalDirection {
    Rl,
    Lr,
}

/// Line alignment
#[derive(Debug, Clone, PartialEq)]
pub enum LineAlign {
    Start,
    Center,
    End,
}

/// Text alignment
#[derive(Debug, Clone, PartialEq)]
pub enum TextAlign {
    Start,
    Center,
    End,
    Left,
    Right,
}

/// WebVTT region
#[derive(Debug, Clone)]
pub struct WebVttRegion {
    pub id: String,
    pub width: Option<String>,
    pub lines: Option<u32>,
    pub region_anchor: Option<String>,
    pub viewport_anchor: Option<String>,
    pub scroll: Option<ScrollBehavior>,
}

/// Scroll behavior
#[derive(Debug, Clone, PartialEq)]
pub enum ScrollBehavior {
    Up,
}

/// WebVTT style block
#[derive(Debug, Clone)]
pub struct WebVttStyle {
    pub selector: String,
    pub rules: HashMap<String, String>,
}

/// Enhanced WebVTT parser
pub struct WebVttParser;

impl WebVttParser {
    /// Parse WebVTT content with styling support
    pub fn parse(content: &str) -> Result<Vec<WebVttCue>> {
        debug!("📄 Parsing WebVTT content");
        
        let content = content.strip_prefix("WEBVTT")
            .ok_or_else(|| anyhow!("Invalid WebVTT: Missing WEBVTT header"))?;
        
        let content = content.trim();
        
        // Parse regions
        let (regions, content) = Self::parse_regions(content)?;
        
        // Parse styles
        let (styles, content) = Self::parse_styles(content)?;
        
        // Parse cues
        let cues = Self::parse_cues(content)?;
        
        debug!("✅ Parsed {} WebVTT cues with {} regions and {} styles", 
               cues.len(), regions.len(), styles.len());
        
        Ok(cues)
    }
    
    /// Parse region definitions
    fn parse_regions(content: &str) -> Result<(Vec<WebVttRegion>, String)> {
        let mut regions = Vec::new();
        let mut remaining_content = content;
        
        for line in content.lines() {
            if line.starts_with("Region:") {
                if let Some(region) = Self::parse_region(line) {
                    regions.push(region);
                }
            } else if !line.trim().is_empty() && !line.starts_with("STYLE") {
                break;
            }
            
            remaining_content = remaining_content.lines()
                .skip(1)
                .collect::<Vec<_>>()
                .join("\n");
        }
        
        Ok((regions, remaining_content))
    }
    
    /// Parse a single region definition
    fn parse_region(line: &str) -> Option<WebVttRegion> {
        let line = line.trim_start_matches("Region:");
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        
        if parts.is_empty() {
            return None;
        }
        
        let id = parts[0].to_string();
        
        let mut width = None;
        let mut lines = None;
        let mut region_anchor = None;
        let mut viewport_anchor = None;
        let mut scroll = None;
        
        for part in &parts[1..] {
            if let Some(key_value) = part.split_once('=') {
                let (key, value) = key_value;
                match key {
                    "width" => width = Some(value.to_string()),
                    "lines" => lines = value.parse().ok(),
                    "regionanchor" => region_anchor = Some(value.to_string()),
                    "viewportanchor" => viewport_anchor = Some(value.to_string()),
                    "scroll" => scroll = if value == "up" {
                        Some(ScrollBehavior::Up)
                    } else {
                        None
                    },
                    _ => {}
                }
            }
        }
        
        Some(WebVttRegion {
            id,
            width,
            lines,
            region_anchor,
            viewport_anchor,
            scroll,
        })
    }
    
    /// Parse style definitions
    fn parse_styles(content: &str) -> Result<(Vec<WebVttStyle>, String)> {
        let mut styles = Vec::new();
        let mut remaining_content = content;
        
        let lines: Vec<&str> = content.lines().collect();
        let mut in_style_block = false;
        let mut current_style: Option<String> = None;
        let mut style_content = String::new();
        
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "STYLE" {
                in_style_block = true;
            } else if in_style_block {
                if line.trim().is_empty() {
                    if let Some(selector) = current_style.take() {
                        let rules = Self::parse_css_rules(&style_content)?;
                        styles.push(WebVttStyle { selector, rules });
                        style_content.clear();
                    }
                } else if line.contains('{') && line.contains('}') {
                    // Single line style
                    if let Some((selector, rules_str)) = line.split_once('{') {
                        let selector = selector.trim().to_string();
                        let rules_str = rules_str.trim().trim_end_matches('}');
                        let rules = Self::parse_css_rules(rules_str)?;
                        styles.push(WebVttStyle { selector, rules });
                    }
                } else if line.contains('{') {
                    // Multi-line style start
                    current_style = Some(line.split('{').next().unwrap().trim().to_string());
                    style_content.clear();
                } else if line.contains('}') {
                    // Multi-line style end
                    if let Some(selector) = current_style.take() {
                        let rules = Self::parse_css_rules(&style_content)?;
                        styles.push(WebVttStyle { selector, rules });
                        style_content.clear();
                    }
                } else {
                    style_content.push_str(line);
                    style_content.push('\n');
                }
                
                if !in_style_block || styles.is_empty() {
                    remaining_content = lines[i..].join("\n");
                }
            }
        }
        
        Ok((styles, remaining_content))
    }
    
    /// Parse CSS rules
    fn parse_css_rules(content: &str) -> Result<HashMap<String, String>> {
        let mut rules = HashMap::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("/*") {
                continue;
            }
            
            if let Some((property, value)) = line.split_once(':') {
                let property = property.trim().to_string();
                let value = value.trim().trim_end_matches(';').to_string();
                rules.insert(property, value);
            }
        }
        
        Ok(rules)
    }
    
    /// Parse cue definitions
    fn parse_cues(content: &str) -> Result<Vec<WebVttCue>> {
        let mut cues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("NOTE") || line.starts_with("REGION") {
                i += 1;
                continue;
            }
            
            // Parse cue identifier (optional)
            let identifier = if !line.contains("-->") {
                let id = line.to_string();
                i += 1;
                if i < lines.len() {
                    Some(id)
                } else {
                    break;
                }
            } else {
                None
            };
            
            // Parse cue timing and settings
            if i >= lines.len() {
                break;
            }
            
            let timing_line = lines[i].trim();
            let (start_ms, end_ms, settings) = Self::parse_cue_timing(timing_line)?;
            i += 1;
            
            // Parse cue text
            let mut text_lines = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() {
                text_lines.push(lines[i].to_string());
                i += 1;
            }
            
            let text = text_lines.join("\n");
            
            // Extract classes and regions from text
            let (classes, regions) = Self::extract_classes_and_regions(&text);
            
            // Create base subtitle entry
            let base = SubtitleEntry::new(start_ms, end_ms, text.clone());
            
            cues.push(WebVttCue {
                base,
                identifier,
                settings,
                text,
                classes,
                regions,
            });
        }
        
        Ok(cues)
    }
    
    /// Parse cue timing line
    fn parse_cue_timing(line: &str) -> Result<(u64, u64, Vec<WebVttSetting>)> {
        let parts: Vec<&str> = line.split("-->").collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid cue timing line"));
        }
        
        let start_str = parts[0].trim();
        let (end_str, settings_str) = if let Some(pos) = parts[1].find(' ') {
            parts[1].split_at(pos)
        } else {
            (parts[1].trim(), "")
        };
        
        let start_ms = Self::parse_webvtt_timestamp(start_str)?;
        let end_ms = Self::parse_webvtt_timestamp(end_str.trim())?;
        
        let settings = if !settings_str.is_empty() {
            Self::parse_settings(settings_str.trim())?
        } else {
            Vec::new()
        };
        
        Ok((start_ms, end_ms, settings))
    }
    
    /// Parse WebVTT timestamp
    fn parse_webvtt_timestamp(ts: &str) -> Result<u64> {
        let ts = ts.trim();
        let parts: Vec<&str> = ts.split(':').collect();
        
        if parts.len() == 3 {
            let hours: u64 = parts[0].parse()?;
            let minutes: u64 = parts[1].parse()?;
            let seconds_str = parts[2];
            
            let (seconds, milliseconds) = if seconds_str.contains('.') {
                let sec_parts: Vec<&str> = seconds_str.split('.').collect();
                let seconds: u64 = sec_parts[0].parse()?;
                let milliseconds_str = format!("{:0<3}", sec_parts.get(1).unwrap_or(&""));
                let milliseconds: u64 = milliseconds_str[..3.min(milliseconds_str.len())].parse().unwrap_or(0);
                (seconds, milliseconds)
            } else {
                let seconds: u64 = seconds_str.parse()?;
                (seconds, 0)
            };
            
            let total_ms = hours * 3600000 + minutes * 60000 + seconds * 1000 + milliseconds;
            Ok(total_ms)
        } else if parts.len() == 2 {
            let minutes: u64 = parts[0].parse()?;
            let seconds_str = parts[1];
            
            let (seconds, milliseconds) = if seconds_str.contains('.') {
                let sec_parts: Vec<&str> = seconds_str.split('.').collect();
                let seconds: u64 = sec_parts[0].parse()?;
                let milliseconds_str = format!("{:0<3}", sec_parts.get(1).unwrap_or(&""));
                let milliseconds: u64 = milliseconds_str[..3.min(milliseconds_str.len())].parse().unwrap_or(0);
                (seconds, milliseconds)
            } else {
                let seconds: u64 = seconds_str.parse()?;
                (seconds, 0)
            };
            
            let total_ms = minutes * 60000 + seconds * 1000 + milliseconds;
            Ok(total_ms)
        } else {
            Err(anyhow!("Invalid WebVTT timestamp format"))
        }
    }
    
    /// Parse cue settings
    fn parse_settings(settings_str: &str) -> Result<Vec<WebVttSetting>> {
        let mut settings = Vec::new();
        
        for setting in settings_str.split_whitespace() {
            if let Some((key, value)) = setting.split_once(':') {
                match key {
                    "vertical" => {
                        let direction = match value {
                            "rl" => VerticalDirection::Rl,
                            "lr" => VerticalDirection::Lr,
                            _ => continue,
                        };
                        settings.push(WebVttSetting::Vertical(direction));
                    }
                    "line" => {
                        let snap_to_lines = !value.ends_with('%');
                        settings.push(WebVttSetting::Line {
                            value: value.to_string(),
                            snap_to_lines,
                        });
                    }
                    "position" => {
                        let line_align = None; // Could parse this
                        settings.push(WebVttSetting::Position {
                            value: value.to_string(),
                            line_align,
                        });
                    }
                    "size" => {
                        if let Ok(size) = value.parse::<u32>() {
                            settings.push(WebVttSetting::Size(size));
                        }
                    }
                    "align" => {
                        let align = match value {
                            "start" => TextAlign::Start,
                            "center" | "middle" => TextAlign::Center,
                            "end" => TextAlign::End,
                            "left" => TextAlign::Left,
                            "right" => TextAlign::Right,
                            _ => continue,
                        };
                        settings.push(WebVttSetting::Align(align));
                    }
                    _ => {}
                }
            }
        }
        
        Ok(settings)
    }
    
    /// Extract classes and regions from text
    fn extract_classes_and_regions(text: &str) -> (Vec<String>, Vec<String>) {
        let mut classes = Vec::new();
        let mut regions = Vec::new();
        
        // Simple extraction - in a real implementation, this would parse HTML
        if text.contains("<") {
            // This is a placeholder - would need proper HTML parsing
            // Extract class="..." attributes
        }
        
        (classes, regions)
    }
    
    /// Convert WebVTT styling to CSS
    pub fn styling_to_css(settings: &[WebVttSetting]) -> String {
        let mut css = String::new();
        
        for setting in settings {
            match setting {
                WebVttSetting::Vertical(dir) => {
                    css.push_str(&format!("  writing-mode: {:?};\n", dir));
                }
                WebVttSetting::Line { value, .. } => {
                    css.push_str(&format!("  top: {};\n", value));
                }
                WebVttSetting::Position { value, .. } => {
                    css.push_str(&format!("  left: {};\n", value));
                }
                WebVttSetting::Size(size) => {
                    css.push_str(&format!("  width: {}%;\n", size));
                }
                WebVttSetting::Align(align) => {
                    css.push_str(&format!("  text-align: {:?};\n", align));
                }
            }
        }
        
        css
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_webvtt_timestamp() {
        let ts = "00:00:01.000";
        let result = WebVttParser::parse_webvtt_timestamp(ts).unwrap();
        assert_eq!(result, 1000);
        
        let ts = "00:01:30.500";
        let result = WebVttParser::parse_webvtt_timestamp(ts).unwrap();
        assert_eq!(result, 90500);
        
        let ts = "01:30.500";
        let result = WebVttParser::parse_webvtt_timestamp(ts).unwrap();
        assert_eq!(result, 90500);
    }
    
    #[test]
    fn test_parse_cue_timing() {
        let line = "00:00:01.000 --> 00:00:04.000 align:center size:50%";
        let (start, end, settings) = WebVttParser::parse_cue_timing(line).unwrap();
        assert_eq!(start, 1000);
        assert_eq!(end, 4000);
        assert!(!settings.is_empty());
    }
}