//! UI Inspector Module
//! 
//! Provides element inspection for development builds.

use iced::{Color, Point, Rectangle, Size};
use std::collections::HashMap;

/// UI Element Inspector
#[derive(Debug, Clone)]
pub struct Inspector {
    /// Is inspector active
    pub active: bool,
    /// Selected element
    pub selected: Option<InspectElement>,
    /// Hover element
    pub hovered: Option<InspectElement>,
    /// Element tree
    pub tree: Vec<InspectElement>,
    /// Show layout bounds
    pub show_bounds: bool,
    /// Show layout guides
    pub show_guides: bool,
    /// Highlight color
    pub highlight_color: Color,
    /// Hover color
    pub hover_color: Color,
}

impl Default for Inspector {
    fn default() -> Self {
        Self {
            active: false,
            selected: None,
            hovered: None,
            tree: Vec::new(),
            show_bounds: true,
            show_guides: false,
            highlight_color: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
            hover_color: Color::from_rgba(1.0, 0.0, 0.5, 0.2),
        }
    }
}

impl Inspector {
    /// Create a new inspector
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Toggle inspector
    pub fn toggle(&mut self) {
        self.active = !self.active;
    }
    
    /// Select element at point
    pub fn select_at(&mut self, point: Point, elements: &[InspectElement]) {
        for element in elements {
            if element.bounds.contains(point) {
                self.selected = Some(element.clone());
                return;
            }
        }
        self.selected = None;
    }
    
    /// Hover element at point
    pub fn hover_at(&mut self, point: Point, elements: &[InspectElement]) {
        for element in elements {
            if element.bounds.contains(point) {
                self.hovered = Some(element.clone());
                return;
            }
        }
        self.hovered = None;
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
        self.hovered = None;
    }
    
    /// Get selected element info
    pub fn selected_info(&self) -> Option<String> {
        self.selected.as_ref().map(|e| {
            format!(
                "Element: {}\n\
                 Type: {}\n\
                 Position: ({:.1}, {:.1})\n\
                 Size: {:.1} x {:.1}\n\
                 Visible: {}",
                e.id,
                e.element_type,
                e.bounds.x, e.bounds.y,
                e.bounds.width, e.bounds.height,
                e.visible
            )
        })
    }
}

/// Inspectable element
#[derive(Debug, Clone)]
pub struct InspectElement {
    /// Element ID
    pub id: String,
    /// Element type
    pub element_type: ElementType,
    /// Bounds
    pub bounds: Rectangle,
    /// Is visible
    pub visible: bool,
    /// Is enabled
    pub enabled: bool,
    /// Z-index
    pub z_index: u32,
    /// Opacity
    pub opacity: f32,
    /// Custom properties
    pub properties: HashMap<String, String>,
    /// Children
    pub children: Vec<InspectElement>,
}

impl InspectElement {
    /// Create a new inspect element
    pub fn new(id: impl Into<String>, element_type: ElementType) -> Self {
        Self {
            id: id.into(),
            element_type,
            bounds: Rectangle::new(Point::ORIGIN, Size::ZERO),
            visible: true,
            enabled: true,
            z_index: 0,
            opacity: 1.0,
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }
    
    /// Set bounds
    pub fn with_bounds(mut self, bounds: Rectangle) -> Self {
        self.bounds = bounds;
        self
    }
    
    /// Set visibility
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
    
    /// Add property
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
    
    /// Add child
    pub fn with_child(mut self, child: InspectElement) -> Self {
        self.children.push(child);
        self
    }
}

/// Element types for inspection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementType {
    Container,
    Text,
    Button,
    Image,
    Video,
    Slider,
    TextInput,
    Checkbox,
    Dropdown,
    List,
    Grid,
    Stack,
    Canvas,
    Custom(&'static str),
}

impl ElementType {
    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            ElementType::Container => "Container",
            ElementType::Text => "Text",
            ElementType::Button => "Button",
            ElementType::Image => "Image",
            ElementType::Video => "Video",
            ElementType::Slider => "Slider",
            ElementType::TextInput => "TextInput",
            ElementType::Checkbox => "Checkbox",
            ElementType::Dropdown => "Dropdown",
            ElementType::List => "List",
            ElementType::Grid => "Grid",
            ElementType::Stack => "Stack",
            ElementType::Canvas => "Canvas",
            ElementType::Custom(name) => name,
        }
    }
}

/// Layout guide for showing spacing
#[derive(Debug, Clone, Copy)]
pub struct LayoutGuide {
    pub start: f32,
    pub end: f32,
    pub axis: Axis,
    pub label: &'static str,
}

/// Axis for layout guides
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_inspector_default() {
        let inspector = Inspector::new();
        assert!(!inspector.active);
        assert!(inspector.selected.is_none());
    }
    
    #[test]
    fn test_inspector_toggle() {
        let mut inspector = Inspector::new();
        inspector.toggle();
        assert!(inspector.active);
    }
    
    #[test]
    fn test_element_selection() {
        let mut inspector = Inspector::new();
        let element = InspectElement::new("test", ElementType::Button)
            .with_bounds(Rectangle::new(Point::new(0.0, 0.0), Size::new(100.0, 50.0)));
        
        inspector.select_at(Point::new(50.0, 25.0), &[element]);
        assert!(inspector.selected.is_some());
    }
}