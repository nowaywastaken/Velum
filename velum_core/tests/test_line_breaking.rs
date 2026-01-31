// Test file to verify line breaking modules compile correctly

use velum_core::line_breaking::{LineBreaker, Line, BreakType};
use velum_core::line_layout::LineLayout;

#[test]
fn test_line_breaker_creation() {
    let breaker = LineBreaker::new();
    assert!(breaker.config.max_width > 0.0);
}

#[test]
fn test_line_breaker_with_width() {
    let breaker = LineBreaker::with_width(100.0);
    assert_eq!(breaker.config.max_width, 100.0);
}

#[test]
fn test_calculate_text_width() {
    let mut breaker = LineBreaker::new();
    let width = breaker.calculate_text_width("hello");
    assert!(width > 0.0);
}

#[test]
fn test_break_lines() {
    let mut breaker = LineBreaker::with_width(100.0);
    let lines = breaker.break_lines("Hello world", None);
    assert!(!lines.is_empty());
}

#[test]
fn test_empty_text() {
    let mut breaker = LineBreaker::new();
    let lines = breaker.break_lines("", None);
    assert!(lines.is_empty());
}

#[test]
fn test_line_layout_creation() {
    let layout = LineLayout::new();
    assert!(layout.breaker().config.max_width > 0.0);
}

#[test]
fn test_layout_paragraph() {
    let mut layout = LineLayout::new();
    let result = layout.layout_paragraph("Test paragraph", 100.0);
    assert!(!result.lines.is_empty());
}

#[test]
fn test_layout_to_json() {
    let mut layout = LineLayout::new();
    let json = layout.layout_to_json("Hello", 100.0);
    assert!(json.starts_with('{'));
}
