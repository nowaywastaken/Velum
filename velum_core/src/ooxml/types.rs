use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Content types defined in [Content_Types].xml
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
    /// Main document body (word/document.xml)
    MainDocument,
    /// Document styles (word/styles.xml)
    Styles,
    /// Theme colors and fonts (word/theme/theme1.xml)
    Theme,
    /// Document settings (word/settings.xml)
    Settings,
    /// Core properties (docProps/core.xml)
    CoreProperties,
    /// App properties (docProps/app.xml)
    AppProperties,
    /// Web settings (word/webSettings.xml)
    WebSettings,
    /// Numbering definitions (word/numbering.xml)
    Numbering,
    /// Custom XML properties
    CustomXml,
    /// Thumbnail image
    Thumbnail,
    /// Relationships file
    Relationships,
    /// Unknown content type
    Unknown(String),
}

impl ContentType {
    /// Parse content type string into enum
    pub fn from_string(s: &str) -> Self {
        match s {
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml" => ContentType::MainDocument,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml" => ContentType::Styles,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.theme+xml" => ContentType::Theme,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml" => ContentType::Settings,
            "application/vnd.openxmlformats-package.core-properties+xml" => ContentType::CoreProperties,
            "application/vnd.openxmlformats-officedocument.extended-properties+xml" => ContentType::AppProperties,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml" => ContentType::WebSettings,
            "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml" => ContentType::Numbering,
            "application/xml" | "application/vnd.openxmlformats-officedocument.customXmlProperties+xml" => ContentType::CustomXml,
            "image/png" | "image/jpeg" | "image/gif" | "image/bmp" => ContentType::Thumbnail,
            "application/vnd.openxmlformats-package.relationships+xml" => ContentType::Relationships,
            _ => ContentType::Unknown(s.to_string()),
        }
    }

    /// Get the part name for this content type
    pub fn default_part_name(&self) -> Option<&'static str> {
        match self {
            ContentType::MainDocument => Some("/word/document.xml"),
            ContentType::Styles => Some("/word/styles.xml"),
            ContentType::Theme => Some("/word/theme/theme1.xml"),
            ContentType::Settings => Some("/word/settings.xml"),
            ContentType::CoreProperties => Some("/docProps/core.xml"),
            ContentType::AppProperties => Some("/docProps/app.xml"),
            ContentType::WebSettings => Some("/word/webSettings.xml"),
            ContentType::Numbering => Some("/word/numbering.xml"),
            _ => None,
        }
    }
}

/// Relationship type constants (ECMA-376)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Main document relationship
    Document,
    /// Styles relationship
    Styles,
    /// Theme relationship
    Theme,
    /// Settings relationship
    Settings,
    /// Core properties relationship
    CoreProperties,
    /// Custom XML relationship
    CustomXml,
    /// Thumbnail relationship
    Thumbnail,
    /// Office document relationship
    OfficeDocument,
    /// Unknown relationship type
    Unknown(String),
}

impl RelationshipType {
    /// Parse relationship type string into enum
    pub fn from_string(s: &str) -> Self {
        match s {
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" => RelationshipType::OfficeDocument,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/mainDocument" => RelationshipType::Document,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" => RelationshipType::Styles,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" => RelationshipType::Theme,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" => RelationshipType::Settings,
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" => RelationshipType::CoreProperties,
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml" => RelationshipType::CustomXml,
            "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail" => RelationshipType::Thumbnail,
            _ => RelationshipType::Unknown(s.to_string()),
        }
    }
}

/// Represents a relationship between parts in the package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Relationship ID (e.g., "rId1")
    pub id: String,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Target URI (can be relative or absolute)
    pub target: String,
    /// Target mode (Internal or External)
    pub target_mode: Option<String>,
}

/// A part in the OPC package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackagePart {
    /// Part name (e.g., "/word/document.xml")
    pub name: String,
    /// Content type of the part
    pub content_type: ContentType,
    /// Raw binary data of the part
    pub data: Vec<u8>,
}

/// Represents a parsed paragraph in the document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paragraph {
    /// Paragraph text content
    pub text: String,
    /// Paragraph properties (indentation, alignment, etc.)
    pub properties: ParagraphProperties,
    /// List of runs in this paragraph
    pub runs: Vec<Run>,
}

/// Properties of a paragraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParagraphProperties {
    /// Paragraph alignment
    pub alignment: Option<String>,
    /// Left indentation in twips (1/20 of a point)
    pub indent_left: Option<i32>,
    /// Right indentation in twips
    pub indent_right: Option<i32>,
    /// First line indentation in twips
    pub indent_first_line: Option<i32>,
    /// Space before paragraph in twips
    pub spacing_before: Option<i32>,
    /// Space after paragraph in twips
    pub spacing_after: Option<i32>,
    /// Line spacing
    pub spacing_line: Option<i32>,
}

/// Represents a run of text with common formatting
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Run {
    /// Text content of the run
    pub text: String,
    /// Run properties
    pub properties: RunProperties,
}

/// Properties of a run (text formatting)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RunProperties {
    /// Bold formatting
    pub bold: Option<bool>,
    /// Italic formatting
    pub italic: Option<bool>,
    /// Underline type
    pub underline: Option<String>,
    /// Font size in half-points
    pub font_size: Option<i32>,
    /// Font name
    pub font_name: Option<String>,
    /// Text color (hex RGB)
    pub color: Option<String>,
    /// Background color (hex RGB)
    pub background_color: Option<String>,
}

/// Represents a style definition
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Style {
    /// Style ID (e.g., "Normal", "Heading1")
    pub id: String,
    /// Style name (e.g., "Normal", "Heading 1")
    pub name: Option<String>,
    /// Style type (paragraph, character, table, number)
    pub style_type: String,
    /// Style ID of the parent style
    pub based_on: Option<String>,
    /// Paragraph properties
    pub paragraph_properties: ParagraphProperties,
    /// Run properties
    pub run_properties: RunProperties,
    /// Whether this is the default style
    pub is_default: bool,
}

/// Theme colors and fonts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Color scheme
    pub colors: HashMap<String, String>,
    /// Font scheme
    pub fonts: ThemeFonts,
}

/// Theme font definitions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeFonts {
    /// Major font (headings)
    pub major_font: String,
    /// Minor font (body text)
    pub minor_font: String,
    /// Symbol font
    pub symbol_font: String,
}
