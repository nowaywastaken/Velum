//! WordProcessingML document parser

use std::collections::HashMap;

use super::opc::OpcPackage;
use super::types::{Paragraph, ParagraphProperties, Run, RunProperties, Style, Theme, ThemeFonts};
use super::error::OoxmlError;

/// WordProcessingML document parser
#[derive(Debug, Clone)]
pub struct WordDocument {
    /// Extracted text content
    pub text: String,
    /// Parsed paragraphs
    pub paragraphs: Vec<Paragraph>,
    /// Document styles indexed by style ID
    pub styles: HashMap<String, Style>,
    /// Document theme (colors/fonts)
    pub theme: Option<Theme>,
    /// Core properties (title, author, etc.)
    pub core_properties: Option<CoreProperties>,
}

/// Core document properties
#[derive(Debug, Clone, Default)]
pub struct CoreProperties {
    pub title: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub keywords: Option<String>,
    pub description: Option<String>,
    pub last_modified_by: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

impl WordDocument {
    /// Create a new WordDocument by parsing the OPC package
    pub fn parse(package: &OpcPackage) -> Result<Self, OoxmlError> {
        let mut document = WordDocument {
            text: String::new(),
            paragraphs: Vec::new(),
            styles: HashMap::new(),
            theme: None,
            core_properties: None,
        };

        document.parse_main_document(package)?;
        document.parse_styles(package)?;
        document.parse_theme(package)?;
        document.parse_core_properties(package)?;

        Ok(document)
    }

    /// Parse the main document body (word/document.xml)
    fn parse_main_document(&mut self, package: &OpcPackage) -> Result<(), OoxmlError> {
        let main_part_name = "/word/document.xml".to_string();

        let main_part = package.get_part(&main_part_name)
            .ok_or_else(|| OoxmlError::PartNotFound(main_part_name.clone()))?;

        let xml_str = String::from_utf8_lossy(&main_part.data);

        // Parse paragraphs - look for <w:p> elements
        let para_pattern = regex::Regex::new(r#"<w:p[^>]*>(.*?)</w:p>"#).unwrap();
        
        for para_cap in para_pattern.captures(&xml_str) {
            let para_xml = match para_cap.get(1) {
                Some(m) => m.as_str(),
                None => continue,
            };
            
            let mut paragraph = Paragraph::default();
            
            // Parse runs within paragraph
            let run_pattern = regex::Regex::new(r#"<w:r[^>]*>(.*?)</w:r>"#).unwrap();
            for run_cap in run_pattern.captures(para_xml) {
                let run_xml = match run_cap.get(1) {
                    Some(m) => m.as_str(),
                    None => continue,
                };
                
                let mut run = Run::default();
                
                // Parse text in run
                let text_pattern = regex::Regex::new(r#"<w:t[^>]*>([^<]*)</w:t>"#).unwrap();
                for text_cap in text_pattern.captures(run_xml) {
                    if let Some(text_match) = text_cap.get(1) {
                        run.text = text_match.as_str().to_string();
                        break;
                    }
                }
                
                // Parse run properties
                let rpr_pattern = regex::Regex::new(r#"<w:rPr[^>]*>(.*?)</w:rPr>"#).unwrap();
                if let Some(rpr_cap) = rpr_pattern.captures(run_xml) {
                    if let Some(rpr_xml) = rpr_cap.get(1) {
                        Self::parse_run_properties(rpr_xml.as_str(), &mut run.properties);
                    }
                }
                
                if !run.text.is_empty() || !run.properties.is_default() {
                    paragraph.runs.push(run);
                }
            }
            
            if !paragraph.runs.is_empty() {
                paragraph.text = paragraph.runs
                    .iter()
                    .map(|r| r.text.clone())
                    .collect();
                self.paragraphs.push(paragraph);
            }
        }

        self.text = self.paragraphs
            .iter()
            .map(|p| p.text.clone())
            .collect::<Vec<_>>()
            .join("\n");

        Ok(())
    }

    /// Parse run properties from XML
    fn parse_run_properties(xml: &str, props: &mut RunProperties) {
        // Bold
        if let Some(caps) = regex::Regex::new(r#"<w:b[^>]*val="([^"]*)""#).unwrap().captures(xml) {
            if let Some(m) = caps.get(1) {
                props.bold = Some(m.as_str() != "0");
            }
        }
        
        // Italic
        if let Some(caps) = regex::Regex::new(r#"<w:i[^>]*val="([^"]*)""#).unwrap().captures(xml) {
            if let Some(m) = caps.get(1) {
                props.italic = Some(m.as_str() != "0");
            }
        }
        
        // Underline
        if let Some(caps) = regex::Regex::new(r#"<w:u[^>]*val="([^"]*)""#).unwrap().captures(xml) {
            if let Some(m) = caps.get(1) {
                props.underline = Some(m.as_str().to_string());
            }
        }
        
        // Font size
        if let Some(caps) = regex::Regex::new(r#"<w:sz[^>]*val="(\d+)""#).unwrap().captures(xml) {
            if let Some(m) = caps.get(1) {
                if let Ok(size) = m.as_str().parse::<i32>() {
                    props.font_size = Some(size / 2);
                }
            }
        }
        
        // Color
        if let Some(caps) = regex::Regex::new(r#"<w:color[^>]*val="([^"]*)""#).unwrap().captures(xml) {
            if let Some(m) = caps.get(1) {
                props.color = Some(m.as_str().to_string());
            }
        }
        
        // Font name
        if let Some(caps) = regex::Regex::new(r#"<w:rFonts[^>]*w:ascii="([^"]*)""#).unwrap().captures(xml) {
            if let Some(m) = caps.get(1) {
                props.font_name = Some(m.as_str().to_string());
            }
        }
    }

    /// Parse styles (word/styles.xml)
    fn parse_styles(&mut self, package: &OpcPackage) -> Result<(), OoxmlError> {
        let styles_part_name = "/word/styles.xml";
        
        let styles_part = if let Some(part) = package.get_part(styles_part_name) {
            part
        } else {
            return Ok(());
        };

        let xml_str = String::from_utf8_lossy(&styles_part.data);
        
        // Parse style elements
        let style_pattern = regex::Regex::new(
            r#"<w:style[^>]*w:styleId="([^"]*)"[^>]*w:type="([^"]*)"[^>]*>(.*?)</w:style>"#
        ).unwrap();
        
        for cap in style_pattern.captures(&xml_str) {
            let style_id = match cap.get(1) {
                Some(m) => m.as_str().to_string(),
                None => continue,
            };
            
            let style_type = match cap.get(2) {
                Some(m) => m.as_str().to_string(),
                None => "paragraph".to_string(),
            };
            
            let style_xml = match cap.get(3) {
                Some(m) => m.as_str(),
                None => "",
            };
            
            let mut style = Style {
                id: style_id.clone(),
                name: None,
                style_type,
                based_on: None,
                paragraph_properties: ParagraphProperties::default(),
                run_properties: RunProperties::default(),
                is_default: false,
            };
            
            // Get style name
            if let Some(name_cap) = regex::Regex::new(r#"<w:name[^>]*w:val="([^"]*)""#).unwrap().captures(style_xml) {
                if let Some(m) = name_cap.get(1) {
                    style.name = Some(m.as_str().to_string());
                }
            }
            
            // Get basedOn
            if let Some(based_cap) = regex::Regex::new(r#"<w:basedOn[^>]*w:val="([^"]*)""#).unwrap().captures(style_xml) {
                if let Some(m) = based_cap.get(1) {
                    style.based_on = Some(m.as_str().to_string());
                }
            }
            
            // Check if default
            if regex::Regex::new(r#"w:default="1""#).unwrap().is_match(style_xml) {
                style.is_default = true;
            }
            
            self.styles.insert(style_id, style);
        }

        Ok(())
    }

    /// Parse theme (word/theme/theme1.xml)
    fn parse_theme(&mut self, package: &OpcPackage) -> Result<(), OoxmlError> {
        let theme_part_names = ["/word/theme/theme1.xml", "/word/theme/theme.xml", "/word/themes/theme1.xml"];
        
        let theme_part = theme_part_names.iter()
            .find(|&name| package.get_part(name).is_some())
            .and_then(|name| package.get_part(name));

        if theme_part.is_none() {
            return Ok(());
        }

        let theme_part = theme_part.unwrap();
        let theme = Theme {
            name: "Office Theme".to_string(),
            colors: HashMap::new(),
            fonts: ThemeFonts {
                major_font: "Calibri".to_string(),
                minor_font: "Calibri".to_string(),
                symbol_font: "Symbol".to_string(),
            },
        };

        self.theme = Some(theme);
        Ok(())
    }

    /// Parse core properties (docProps/core.xml)
    fn parse_core_properties(&mut self, package: &OpcPackage) -> Result<(), OoxmlError> {
        let core_part_name = "/docProps/core.xml";
        
        let core_part = if let Some(part) = package.get_part(core_part_name) {
            part
        } else {
            return Ok(());
        };

        let xml_str = String::from_utf8_lossy(&core_part.data);
        let mut props = CoreProperties::default();

        // Parse title
        if let Some(caps) = regex::Regex::new(r#"<dc:title[^>]*>([^<]*)</dc:title>"#).unwrap().captures(&xml_str) {
            if let Some(m) = caps.get(1) {
                props.title = Some(m.as_str().to_string());
            }
        }
        
        // Parse creator
        if let Some(caps) = regex::Regex::new(r#"<dc:creator[^>]*>([^<]*)</dc:creator>"#).unwrap().captures(&xml_str) {
            if let Some(m) = caps.get(1) {
                props.creator = Some(m.as_str().to_string());
            }
        }
        
        // Parse created
        if let Some(caps) = regex::Regex::new(r#"<dcterms:created[^>]*>([^<]*)</dcterms:created>"#).unwrap().captures(&xml_str) {
            if let Some(m) = caps.get(1) {
                props.created = Some(m.as_str().to_string());
            }
        }
        
        // Parse modified
        if let Some(caps) = regex::Regex::new(r#"<dcterms:modified[^>]*>([^<]*)</dcterms:modified>"#).unwrap().captures(&xml_str) {
            if let Some(m) = caps.get(1) {
                props.modified = Some(m.as_str().to_string());
            }
        }

        self.core_properties = Some(props);
        Ok(())
    }
}

impl RunProperties {
    /// Check if properties are default (no formatting)
    fn is_default(&self) -> bool {
        self.bold.is_none() 
            && self.italic.is_none() 
            && self.underline.is_none() 
            && self.font_size.is_none() 
            && self.font_name.is_none() 
            && self.color.is_none() 
            && self.background_color.is_none()
    }
}
