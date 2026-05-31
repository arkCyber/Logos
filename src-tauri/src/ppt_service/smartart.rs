//! SmartArt Element Module
//! 
//! Aerospace-grade SmartArt implementation for PPT slides with:
//! - Input validation
//! - Bounds checking
//! - Comprehensive error handling
//! - Multiple SmartArt types and layouts
//! - Node and connection management
//! - Performance monitoring

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SmartArt type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmartArtType {
    /// Process (流程图)
    Process,
    /// Cycle (循环图)
    Cycle,
    /// Hierarchy (层次结构图)
    Hierarchy,
    /// Relationship (关系图)
    Relationship,
    /// Matrix (矩阵图)
    Matrix,
    /// Pyramid (棱锥图)
    Pyramid,
    /// List (列表)
    List,
    /// Chart (图表)
    Chart,
    /// Custom (自定义)
    Custom(String),
}

/// SmartArt layout
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmartArtLayout {
    /// Basic process
    BasicProcess,
    /// Chevron process
    ChevronProcess,
    /// Arrow process
    ArrowProcess,
    /// Circle process
    CircleProcess,
    /// Basic cycle
    BasicCycle,
    /// Radial cycle
    RadialCycle,
    /// Horizontal hierarchy
    HorizontalHierarchy,
    /// Vertical hierarchy
    VerticalHierarchy,
    /// Organization chart
    OrganizationChart,
    /// Balance
    Balance,
    /// Converging
    Converging,
    /// Diverging
    Diverging,
    /// Basic matrix
    BasicMatrix,
    /// Titled matrix
    TitledMatrix,
    /// Basic pyramid
    BasicPyramid,
    /// Inverted pyramid
    InvertedPyramid,
    /// Segmented pyramid
    SegmentedPyramid,
    /// Custom (自定义)
    Custom(String),
}

/// SmartArt color style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmartArtColorStyle {
    /// Colorful
    Colorful,
    /// Simple
    Simple,
    /// Gradient
    Gradient,
    /// Monochrome
    Monochrome,
    /// Custom (自定义)
    Custom(String),
}

/// SmartArt node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartArtNode {
    /// Node ID
    pub id: String,
    /// Node text
    pub text: String,
    /// Parent node ID (for hierarchy)
    pub parent_id: Option<String>,
    /// Child node IDs
    pub child_ids: Vec<String>,
    /// Position (X, Y coordinates in points)
    pub position: (f64, f64),
    /// Size (width, height in points)
    pub size: (f64, f64),
    /// Background color (RGB)
    pub background_color: Option<(u8, u8, u8)>,
    /// Text color (RGB)
    pub text_color: (u8, u8, u8),
    /// Font name
    pub font_name: String,
    /// Font size (points)
    pub font_size: f64,
    /// Shape type
    pub shape_type: SmartArtShapeType,
    /// Level (for hierarchy)
    pub level: u32,
}

/// SmartArt shape type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SmartArtShapeType {
    /// Rectangle
    Rectangle,
    /// Rounded rectangle
    RoundedRectangle,
    /// Circle
    Circle,
    /// Oval
    Oval,
    /// Diamond
    Diamond,
    /// Hexagon
    Hexagon,
    /// Octagon
    Octagon,
    /// Triangle
    Triangle,
    /// Arrow
    Arrow,
    /// Chevron
    Chevron,
    /// Custom (自定义)
    Custom(String),
}

/// SmartArt connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartArtConnection {
    /// Connection ID
    pub id: String,
    /// Source node ID
    pub source_id: String,
    /// Target node ID
    pub target_id: String,
    /// Connection type
    pub connection_type: ConnectionType,
    /// Line color (RGB)
    pub line_color: (u8, u8, u8),
    /// Line width (points)
    pub line_width: f64,
    /// Arrow style
    pub arrow_style: ArrowStyle,
}

/// Connection type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    /// Straight line
    Straight,
    /// Curved line
    Curved,
    /// Elbow
    Elbow,
    /// Custom (自定义)
    Custom(String),
}

/// Arrow style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArrowStyle {
    /// No arrow
    None,
    /// Arrow at end
    End,
    /// Arrow at start
    Start,
    /// Arrows at both ends
    Both,
}

/// SmartArt element for PPT slides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartArtElement {
    /// SmartArt element ID
    pub id: String,
    /// SmartArt type
    pub smartart_type: SmartArtType,
    /// SmartArt layout
    pub layout: SmartArtLayout,
    /// Color style
    pub color_style: SmartArtColorStyle,
    /// Position (X, Y coordinates in points)
    pub position: (f64, f64),
    /// Size (width, height in points)
    pub size: (f64, f64),
    /// Nodes
    pub nodes: HashMap<String, SmartArtNode>,
    /// Connections
    pub connections: Vec<SmartArtConnection>,
    /// Primary color (RGB)
    pub primary_color: (u8, u8, u8),
    /// Secondary color (RGB)
    pub secondary_color: (u8, u8, u8),
    /// Accent color (RGB)
    pub accent_color: (u8, u8, u8),
}

impl SmartArtElement {
    /// Maximum number of nodes to prevent memory exhaustion
    const MAX_NODES: usize = 50;

    /// Maximum number of connections
    const MAX_CONNECTIONS: usize = 100;

    /// Maximum text length per node
    const MAX_TEXT_LENGTH: usize = 100;

    /// Create a new SmartArt element
    pub fn new(id: String, smartart_type: SmartArtType) -> Self {
        let layout = Self::default_layout_for_type(&smartart_type);
        Self {
            id,
            smartart_type,
            layout,
            color_style: SmartArtColorStyle::Colorful,
            position: (0.0, 0.0),
            size: (600.0, 400.0),
            nodes: HashMap::new(),
            connections: Vec::new(),
            primary_color: (0, 102, 204),
            secondary_color: (255, 255, 255),
            accent_color: (255, 153, 0),
        }
    }

    /// Get default layout for SmartArt type
    fn default_layout_for_type(smartart_type: &SmartArtType) -> SmartArtLayout {
        match smartart_type {
            SmartArtType::Process => SmartArtLayout::BasicProcess,
            SmartArtType::Cycle => SmartArtLayout::BasicCycle,
            SmartArtType::Hierarchy => SmartArtLayout::HorizontalHierarchy,
            SmartArtType::Relationship => SmartArtLayout::Balance,
            SmartArtType::Matrix => SmartArtLayout::BasicMatrix,
            SmartArtType::Pyramid => SmartArtLayout::BasicPyramid,
            SmartArtType::List => SmartArtLayout::BasicProcess,
            SmartArtType::Chart => SmartArtLayout::BasicProcess,
            SmartArtType::Custom(_) => SmartArtLayout::BasicProcess,
        }
    }

    /// Set layout
    pub fn with_layout(mut self, layout: SmartArtLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set color style
    pub fn with_color_style(mut self, color_style: SmartArtColorStyle) -> Self {
        self.color_style = color_style;
        self
    }

    /// Set position
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// Set size
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// Set primary color
    pub fn with_primary_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.primary_color = (r, g, b);
        self
    }

    /// Set secondary color
    pub fn with_secondary_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.secondary_color = (r, g, b);
        self
    }

    /// Set accent color
    pub fn with_accent_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.accent_color = (r, g, b);
        self
    }

    /// Add a node
    pub fn add_node(&mut self, node: SmartArtNode) -> Result<(), String> {
        // Validate node count
        if self.nodes.len() >= Self::MAX_NODES {
            return Err(format!(
                "Maximum number of nodes ({}) exceeded",
                Self::MAX_NODES
            ));
        }

        // Validate node
        node.validate()?;

        // Add node
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    /// Remove a node
    pub fn remove_node(&mut self, node_id: &str) -> Result<(), String> {
        if !self.nodes.contains_key(node_id) {
            return Err(format!("Node {} not found", node_id));
        }

        // Remove node
        self.nodes.remove(node_id);

        // Remove connections to/from this node
        self.connections.retain(|conn| {
            conn.source_id != node_id && conn.target_id != node_id
        });

        // Update parent/child references
        for node in self.nodes.values_mut() {
            if node.parent_id.as_ref() == Some(&node_id.to_string()) {
                node.parent_id = None;
            }
            node.child_ids.retain(|id| id != node_id);
        }

        Ok(())
    }

    /// Add a connection
    pub fn add_connection(&mut self, connection: SmartArtConnection) -> Result<(), String> {
        // Validate connection count
        if self.connections.len() >= Self::MAX_CONNECTIONS {
            return Err(format!(
                "Maximum number of connections ({}) exceeded",
                Self::MAX_CONNECTIONS
            ));
        }

        // Validate that source and target nodes exist
        if !self.nodes.contains_key(&connection.source_id) {
            return Err(format!("Source node {} not found", connection.source_id));
        }

        if !self.nodes.contains_key(&connection.target_id) {
            return Err(format!("Target node {} not found", connection.target_id));
        }

        // Add connection
        self.connections.push(connection);
        Ok(())
    }

    /// Validate SmartArt settings
    pub fn validate(&self) -> Result<(), String> {
        // Validate position
        if self.position.0 < 0.0 || self.position.1 < 0.0 {
            return Err("Position coordinates cannot be negative".to_string());
        }

        // Validate size
        if self.size.0 <= 0.0 || self.size.1 <= 0.0 {
            return Err("Size dimensions must be positive".to_string());
        }

        // Validate nodes
        for node in self.nodes.values() {
            node.validate()?;
        }

        // Validate connections
        for connection in &self.connections {
            if !self.nodes.contains_key(&connection.source_id) {
                return Err(format!(
                    "Connection source node {} not found",
                    connection.source_id
                ));
            }

            if !self.nodes.contains_key(&connection.target_id) {
                return Err(format!(
                    "Connection target node {} not found",
                    connection.target_id
                ));
            }
        }

        Ok(())
    }

    /// Create a process SmartArt
    pub fn process(id: String) -> Self {
        Self::new(id, SmartArtType::Process)
    }

    /// Create a cycle SmartArt
    pub fn cycle(id: String) -> Self {
        Self::new(id, SmartArtType::Cycle)
    }

    /// Create a hierarchy SmartArt
    pub fn hierarchy(id: String) -> Self {
        Self::new(id, SmartArtType::Hierarchy)
    }

    /// Create a relationship SmartArt
    pub fn relationship(id: String) -> Self {
        Self::new(id, SmartArtType::Relationship)
    }

    /// Create a matrix SmartArt
    pub fn matrix(id: String) -> Self {
        Self::new(id, SmartArtType::Matrix)
    }

    /// Create a pyramid SmartArt
    pub fn pyramid(id: String) -> Self {
        Self::new(id, SmartArtType::Pyramid)
    }

    /// Auto-layout nodes based on SmartArt type
    pub fn auto_layout(&mut self) -> Result<(), String> {
        let node_count = self.nodes.len();
        if node_count == 0 {
            return Ok(());
        }

        let nodes: Vec<_> = self.nodes.values().cloned().collect();
        let width = self.size.0;
        let height = self.size.1;

        match self.layout {
            SmartArtLayout::BasicProcess => {
                // Horizontal layout
                let spacing = width / (node_count as f64 + 1.0);
                for (i, node) in nodes.iter().enumerate() {
                    let x = spacing * (i as f64 + 1.0) - 50.0;
                    let y = height / 2.0 - 25.0;
                    if let Some(n) = self.nodes.get_mut(&node.id) {
                        n.position = (x, y);
                    }
                }
            }
            SmartArtLayout::BasicCycle => {
                // Circular layout
                let center_x = width / 2.0;
                let center_y = height / 2.0;
                let radius = (width.min(height) / 2.0) - 50.0;
                let angle_step = 2.0 * std::f64::consts::PI / node_count as f64;

                for (i, node) in nodes.iter().enumerate() {
                    let angle = angle_step * i as f64;
                    let x = center_x + radius * angle.cos() - 50.0;
                    let y = center_y + radius * angle.sin() - 25.0;
                    if let Some(n) = self.nodes.get_mut(&node.id) {
                        n.position = (x, y);
                    }
                }
            }
            SmartArtLayout::HorizontalHierarchy => {
                // Hierarchical layout (simplified)
                let levels: Vec<_> = nodes.iter().map(|n| n.level).collect();
                let max_level = *levels.iter().max().unwrap_or(&0) as f64;
                let level_height = height / (max_level + 1.0);

                for node in nodes {
                    let y = level_height * node.level as f64;
                    let x = 50.0;
                    if let Some(n) = self.nodes.get_mut(&node.id) {
                        n.position = (x, y);
                    }
                }
            }
            _ => {
                // Default: simple grid layout
                let cols = (node_count as f64).sqrt().ceil() as usize;
                let rows = (node_count as f64 / cols as f64).ceil() as usize;
                let cell_width = width / cols as f64;
                let cell_height = height / rows as f64;

                for (i, node) in nodes.iter().enumerate() {
                    let col = i % cols;
                    let row = i / cols;
                    let x = cell_width * col as f64 + 10.0;
                    let y = cell_height * row as f64 + 10.0;
                    if let Some(n) = self.nodes.get_mut(&node.id) {
                        n.position = (x, y);
                    }
                }
            }
        }

        Ok(())
    }
}

impl SmartArtNode {
    /// Create a new SmartArt node
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            parent_id: None,
            child_ids: Vec::new(),
            position: (0.0, 0.0),
            size: (100.0, 50.0),
            background_color: None,
            text_color: (0, 0, 0),
            font_name: "Arial".to_string(),
            font_size: 12.0,
            shape_type: SmartArtShapeType::Rectangle,
            level: 0,
        }
    }

    /// Set position
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = (x, y);
        self
    }

    /// Set size
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = (width, height);
        self
    }

    /// Set background color
    pub fn with_background_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background_color = Some((r, g, b));
        self
    }

    /// Set text color
    pub fn with_text_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.text_color = (r, g, b);
        self
    }

    /// Set font
    pub fn with_font(mut self, font_name: String, size: f64) -> Self {
        self.font_name = font_name;
        self.font_size = size;
        self
    }

    /// Set shape type
    pub fn with_shape_type(mut self, shape_type: SmartArtShapeType) -> Self {
        self.shape_type = shape_type;
        self
    }

    /// Set level
    pub fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    /// Set parent
    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Validate node
    pub fn validate(&self) -> Result<(), String> {
        // Validate text
        if self.text.is_empty() {
            return Err("Node text cannot be empty".to_string());
        }

        if self.text.len() > SmartArtElement::MAX_TEXT_LENGTH {
            return Err(format!(
                "Text length exceeds maximum of {} characters",
                SmartArtElement::MAX_TEXT_LENGTH
            ));
        }

        // Validate position
        if self.position.0 < 0.0 || self.position.1 < 0.0 {
            return Err("Position coordinates cannot be negative".to_string());
        }

        // Validate size
        if self.size.0 <= 0.0 || self.size.1 <= 0.0 {
            return Err("Size dimensions must be positive".to_string());
        }

        // Validate font size
        if self.font_size <= 0.0 {
            return Err("Font size must be positive".to_string());
        }

        Ok(())
    }
}

impl Default for SmartArtElement {
    fn default() -> Self {
        Self::new("default".to_string(), SmartArtType::Process)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smartart_element_new() {
        let smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process);
        assert_eq!(smartart.id, "1");
        assert_eq!(smartart.smartart_type, SmartArtType::Process);
        assert_eq!(smartart.layout, SmartArtLayout::BasicProcess);
    }

    #[test]
    fn test_smartart_element_with_layout() {
        let smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process)
            .with_layout(SmartArtLayout::ChevronProcess);
        assert_eq!(smartart.layout, SmartArtLayout::ChevronProcess);
    }

    #[test]
    fn test_smartart_element_add_node() {
        let mut smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process);
        let node = SmartArtNode::new("node1".to_string(), "Step 1".to_string());
        assert!(smartart.add_node(node).is_ok());
        assert_eq!(smartart.nodes.len(), 1);
    }

    #[test]
    fn test_smartart_element_add_node_max() {
        let mut smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process);
        for i in 0..SmartArtElement::MAX_NODES {
            let node = SmartArtNode::new(format!("node{}", i), format!("Step {}", i));
            assert!(smartart.add_node(node).is_ok());
        }
        let extra_node = SmartArtNode::new("extra".to_string(), "Extra".to_string());
        assert!(smartart.add_node(extra_node).is_err());
    }

    #[test]
    fn test_smartart_element_remove_node() {
        let mut smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process);
        let node = SmartArtNode::new("node1".to_string(), "Step 1".to_string());
        smartart.add_node(node).unwrap();
        assert!(smartart.remove_node("node1").is_ok());
        assert_eq!(smartart.nodes.len(), 0);
    }

    #[test]
    fn test_smartart_element_add_connection() {
        let mut smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process);
        let node1 = SmartArtNode::new("node1".to_string(), "Step 1".to_string());
        let node2 = SmartArtNode::new("node2".to_string(), "Step 2".to_string());
        smartart.add_node(node1).unwrap();
        smartart.add_node(node2).unwrap();

        let connection = SmartArtConnection {
            id: "conn1".to_string(),
            source_id: "node1".to_string(),
            target_id: "node2".to_string(),
            connection_type: ConnectionType::Straight,
            line_color: (0, 0, 0),
            line_width: 1.0,
            arrow_style: ArrowStyle::End,
        };
        assert!(smartart.add_connection(connection).is_ok());
        assert_eq!(smartart.connections.len(), 1);
    }

    #[test]
    fn test_smartart_node_new() {
        let node = SmartArtNode::new("node1".to_string(), "Step 1".to_string());
        assert_eq!(node.id, "node1");
        assert_eq!(node.text, "Step 1");
        assert_eq!(node.shape_type, SmartArtShapeType::Rectangle);
    }

    #[test]
    fn test_smartart_node_validate_text_empty() {
        let node = SmartArtNode::new("node1".to_string(), "".to_string());
        assert!(node.validate().is_err());
    }

    #[test]
    fn test_smartart_node_validate_position_negative() {
        let node = SmartArtNode::new("node1".to_string(), "Step 1".to_string())
            .with_position(-10.0, 100.0);
        assert!(node.validate().is_err());
    }

    #[test]
    fn test_smartart_element_process() {
        let smartart = SmartArtElement::process("1".to_string());
        assert_eq!(smartart.smartart_type, SmartArtType::Process);
    }

    #[test]
    fn test_smartart_element_cycle() {
        let smartart = SmartArtElement::cycle("1".to_string());
        assert_eq!(smartart.smartart_type, SmartArtType::Cycle);
    }

    #[test]
    fn test_smartart_element_hierarchy() {
        let smartart = SmartArtElement::hierarchy("1".to_string());
        assert_eq!(smartart.smartart_type, SmartArtType::Hierarchy);
    }

    #[test]
    fn test_smartart_element_auto_layout() {
        let mut smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process)
            .with_size(600.0, 400.0);
        
        for i in 0..3 {
            let node = SmartArtNode::new(format!("node{}", i), format!("Step {}", i + 1));
            smartart.add_node(node).unwrap();
        }

        assert!(smartart.auto_layout().is_ok());
    }

    #[test]
    fn test_smartart_element_serialization() {
        let smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process);
        let json = serde_json::to_string(&smartart);
        assert!(json.is_ok());
    }

    #[test]
    fn test_smartart_element_deserialization() {
        let smartart = SmartArtElement::new("1".to_string(), SmartArtType::Process);
        let json = serde_json::to_string(&smartart).unwrap();
        let deserialized: SmartArtElement = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, smartart.id);
        assert_eq!(deserialized.smartart_type, smartart.smartart_type);
    }
}
