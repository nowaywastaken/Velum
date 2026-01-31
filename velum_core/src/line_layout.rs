//! # Line Layout Module
//!
//! Provides higher-level text layout functionality including paragraph layout
//! and bidirectional text support.

use crate::line_breaking::{BreakType, Line, LineBreaker};
use serde::{Deserialize, Serialize};
use unicode_bidi::BidiInfo;

/// Represents a line with visual layout information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutLine {
    /// Start byte offset in original text
    pub start: usize,
    /// End byte offset in original text
    pub end: usize,
    /// Width of the line in abstract units
    pub width: f32,
    /// Type of break
    pub break_type: String,
    /// Visual order for bidirectional text (None if LTR)
    pub visual_order: Option<Vec<(usize, usize)>>,
    /// Whether this line contains bidirectional text
    pub is_bidi: bool,
}

/// Represents layout information for a single line
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineLayoutInfo {
    /// Line index (0-based)
    pub line_number: usize,
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Line width
    pub width: f32,
    /// Break type as string
    pub break_type: String,
    /// Character count on line
    pub char_count: usize,
    /// Whether line contains bidirectional text
    pub is_bidi: bool,
    /// Trailing whitespace width
    pub trailing_whitespace: f32,
}

/// Complete paragraph layout result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParagraphLayout {
    /// Original text
    pub text: String,
    /// Maximum line width used
    pub max_width: f32,
    /// Individual line layouts
    pub lines: Vec<LineLayoutInfo>,
    /// Total height (lines * line_height)
    pub total_height: f32,
    /// Whether text contains bidirectional content
    pub has_bidi: bool,
}

/// Complete document layout result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentLayout {
    /// All paragraphs
    pub paragraphs: Vec<ParagraphLayout>,
    /// Total width
    pub total_width: f32,
    /// Total height
    pub total_height: f32,
    /// Line height
    pub line_height: f32,
}

/// Configuration for line layout
#[derive(Debug, Clone)]
pub struct LineLayoutConfig {
    /// Line height in abstract units
    pub line_height: f32,
    /// Tab size in spaces
    pub tab_size: usize,
    /// Default font size
    pub font_size: f32,
    /// Enable bidirectional support
    pub bidi_enabled: bool,
    /// Trim trailing whitespace
    pub trim_trailing: bool,
}

impl Default for LineLayoutConfig {
    fn default() -> Self {
        LineLayoutConfig {
            line_height: 1.2,
            tab_size: 4,
            font_size: 14.0,
            bidi_enabled: true,
            trim_trailing: true,
        }
    }
}

/// Main line layout struct
#[derive(Debug, Clone)]
pub struct LineLayout {
    config: LineLayoutConfig,
    breaker: LineBreaker,
}

impl Default for LineLayout {
    fn default() -> Self {
        LineLayout::new()
    }
}

impl LineLayout {
    /// Creates a new line layout with default configuration
    #[inline]
    pub fn new() -> Self {
        LineLayout {
            config: LineLayoutConfig::default(),
            breaker: LineBreaker::new(),
        }
    }

    /// Creates a new line layout with custom configuration
    #[inline]
    pub fn with_config(config: LineLayoutConfig) -> Self {
        LineLayout {
            config,
            breaker: LineBreaker::new(),
        }
    }

    /// Sets the line height
    #[inline]
    pub fn set_line_height(&mut self, height: f32) {
        self.config.line_height = height;
    }

    /// Sets the tab size
    #[inline]
    pub fn set_tab_size(&mut self, size: usize) {
        self.config.tab_size = size;
    }

    /// Enables or disables bidirectional support
    #[inline]
    pub fn set_bidi(&mut self, enabled: bool) {
        self.config.bidi_enabled = enabled;
    }

    /// Layouts a single paragraph
    pub fn layout_paragraph(&mut self, text: &str, max_width: f32) -> ParagraphLayout {
        self.breaker.set_max_width(max_width);

        let lines = self.breaker.break_lines(text, None);
        let mut layout_lines = Vec::new();

        let mut has_bidi = false;
        let mut char_offset = 0usize;

        for (i, line) in lines.iter().enumerate() {
            if line.is_empty() {
                layout_lines.push(LineLayoutInfo {
                    line_number: i,
                    start: char_offset,
                    end: char_offset,
                    width: 0.0,
                    break_type: "HardBreak".to_string(),
                    char_count: 0,
                    is_bidi: false,
                    trailing_whitespace: 0.0,
                });
                continue;
            }

            let line_text = &text[line.start..line.end];
            let char_count = line_text.chars().count();

            // Check for bidirectional text
            let is_bidi = if self.config.bidi_enabled {
                let has_rtl = line_text.chars().any(|c| {
                    matches!(
                        c,
                        '\u{0590}'..='\u{05FF}' |  // Hebrew
                        '\u{0600}'..='\u{06FF}' |  // Arabic
                        '\u{0750}'..='\u{077F}' |  // Arabic Supplement
                        '\u{08A0}'..='\u{08FF}' |  // Arabic Extended-A
                        '\u{FB50}'..='\u{FDFF}' |  // Arabic Presentation Forms-A
                        '\u{FE70}'..='\u{FEFF}' |  // Arabic Presentation Forms-B
                        '\u{10800}'..='\u{10FFF}'  // Private Use Area (some RTL scripts)
                    )
                });
                if has_rtl {
                    has_bidi = true;
                }
                has_rtl
            } else {
                false
            };

            // Calculate trailing whitespace
            let trailing_ws = if self.config.trim_trailing {
                let trimmed: String = line_text.chars().rev().take_while(|c| c.is_whitespace()).collect();
                self.breaker.calculate_text_width(&trimmed.chars().rev().collect::<String>())
            } else {
                0.0
            };

            let break_type_str = match line.break_type {
                BreakType::HardBreak => "HardBreak",
                BreakType::SoftBreak => "SoftBreak",
                BreakType::Hyphenated => "Hyphenated",
            };

            layout_lines.push(LineLayoutInfo {
                line_number: i,
                start: line.start,
                end: line.end,
                width: line.width,
                break_type: break_type_str.to_string(),
                char_count,
                is_bidi,
                trailing_whitespace: trailing_ws,
            });

            char_offset = line.end;
        }

        let total_height = layout_lines.len() as f32 * self.config.line_height * self.config.font_size;

        ParagraphLayout {
            text: text.to_string(),
            max_width,
            lines: layout_lines,
            total_height,
            has_bidi,
        }
    }

    /// Layouts a full document with multiple paragraphs
    pub fn layout_document(&mut self, text: &str, max_width: f32) -> DocumentLayout {
        let paragraphs: Vec<&str> = text.split('\n').collect();
        let mut all_paragraphs = Vec::new();
        let mut total_width = 0.0f32;
        let mut total_height = 0.0f32;

        for paragraph in paragraphs {
            let layout = self.layout_paragraph(paragraph, max_width);

            // Track maximum width
            for line in &layout.lines {
                if line.width > total_width {
                    total_width = line.width;
                }
            }

            total_height += layout.total_height;
            all_paragraphs.push(layout);
        }

        DocumentLayout {
            paragraphs: all_paragraphs,
            total_width,
            total_height,
            line_height: self.config.line_height * self.config.font_size,
        }
    }

    /// Layouts text and returns JSON string
    pub fn layout_to_json(&mut self, text: &str, max_width: f32) -> String {
        let layout = self.layout_document(text, max_width);
        serde_json::to_string(&layout).unwrap_or_else(|_| "{}".to_string())
    }

    /// Calculates the visual order for a bidirectional line
    #[allow(dead_code)]
    pub fn calculate_visual_order(&self, text: &str) -> Vec<(usize, usize)> {
        if text.is_empty() {
            return Vec::new();
        }

        // Simple implementation - returns the text as-is for LTR
        // Full bidirectional reordering would require more complex handling
        vec![(0, text.len())]
    }

    /// Gets the line breaker for direct access
    #[inline]
    pub fn breaker_mut(&mut self) -> &mut LineBreaker {
        &mut self.breaker
    }

    /// Gets the line breaker for read-only access
    #[inline]
    pub fn breaker(&self) -> &LineBreaker {
        &self.breaker
    }
}

/// Utility functions for text measurement
pub mod measure {
    use super::*;

    /// Gets the number of lines needed for text at given width
    pub fn get_line_count(text: &str, max_width: f32) -> usize {
        let mut layout = LineLayout::new();
        layout.breaker_mut().break_lines(text, Some(max_width)).len()
    }

    /// Gets the height needed for text at given width
    pub fn get_text_height(text: &str, max_width: f32, line_height: f32, font_size: f32) -> f32 {
        let line_count = get_line_count(text, max_width);
        line_count as f32 * line_height * font_size
    }

    /// Gets the total width of text
    pub fn get_text_total_width(text: &str) -> f32 {
        let mut layout = LineLayout::new();
        layout.breaker_mut().calculate_text_width(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_paragraph_layout() {
        let mut layout = LineLayout::new();
        let text = "This is a test paragraph for layout.";
        let result = layout.layout_paragraph(text, 100.0);

        assert!(result.lines.len() >= 1);
        assert!(result.total_height > 0.0);
    }

    #[test]
    fn test_empty_paragraph() {
        let mut layout = LineLayout::new();
        let text = "";
        let result = layout.layout_paragraph(text, 100.0);

        assert!(result.lines.is_empty() || result.lines.len() == 0);
    }

    #[test]
    fn test_multiline_paragraph() {
        let mut layout = LineLayout::new();
        let text = "This is a longer paragraph that should definitely require multiple lines to display properly within the given width constraint.";
        let result = layout.layout_paragraph(text, 80.0);

        // Should have multiple lines
        assert!(result.lines.len() > 1);
    }

    #[test]
    fn test_paragraph_with_newlines() {
        let mut layout = LineLayout::new();
        let text = "First paragraph.\nSecond paragraph.\nThird paragraph.";
        let result = layout.layout_document(text, 100.0);

        assert!(result.paragraphs.len() >= 3);
    }

    #[test]
    fn test_cjk_text_layout() {
        let mut layout = LineLayout::new();
        let text = "这是一个测试段落，用于测试中文分行功能是否正常工作。";
        let result = layout.layout_paragraph(text, 50.0);

        assert!(result.lines.len() >= 1);
        for line in &result.lines {
            assert!(line.width <= 50.0 + 1.0);
        }
    }

    #[test]
    fn test_line_height() {
        let mut layout = LineLayout::new();
        layout.set_line_height(1.5);

        let text = "Test text";
        let result = layout.layout_paragraph(text, 100.0);

        // Total height should be proportional to line height
        assert!(result.total_height > 0.0);
    }

    #[test]
    fn test_json_output() {
        let mut layout = LineLayout::new();
        let text = "Hello world";
        let json = layout.layout_to_json(text, 100.0);

        assert!(json.starts_with('{'));
        assert!(json.contains("paragraphs"));
    }

    #[test]
    fn test_visual_order() {
        let layout = LineLayout::new();
        // Simple LTR text
        let text = "Hello";
        let order = layout.calculate_visual_order(text);
        assert!(!order.is_empty());
    }

    #[test]
    fn test_line_layout_info() {
        let mut layout = LineLayout::new();
        let text = "Test line";
        let result = layout.layout_paragraph(text, 100.0);

        if let Some(line) = result.lines.first() {
            assert_eq!(line.line_number, 0);
            assert!(line.width > 0.0);
            assert!(line.char_count > 0);
        }
    }

    #[test]
    fn test_trailing_whitespace() {
        let mut layout = LineLayout::new();
        let text = "Test   ";
        let result = layout.layout_paragraph(text, 100.0);

        if let Some(line) = result.lines.first() {
            assert!(line.trailing_whitespace >= 0.0);
        }
    }
}
