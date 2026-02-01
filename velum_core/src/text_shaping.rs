use harfbuzz_rs::{Face, Font, UnicodeBuffer, shape, Owned};
use std::sync::Arc;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::Path;

/// Represents a shaped glyph with positioning information
#[derive(Debug, Clone, Copy)]
pub struct GlyphInfo {
    /// The glyph ID in the font
    pub codepoint: u32,
    /// The cluster index (character index) this glyph belongs to
    pub cluster: u32,
    /// X advance width in logical pixels
    pub x_advance: f32,
    /// Y advance height in logical pixels
    pub y_advance: f32,
    /// X offset in logical pixels
    pub x_offset: f32,
    /// Y offset in logical pixels
    pub y_offset: f32,
}

/// A text shaper that uses HarfBuzz
#[derive(Debug)]
pub struct TextShaper {
    font: Owned<Font<'static>>,
    /// Units per EM for the current font
    upem: i32,
    /// Current font size in points
    font_size_pt: f32,
    /// Scaling factor from font units to logical pixels
    /// pixel = unit * scale_factor
    scale_factor: f32,
}

impl TextShaper {
    /// Creates a new text shaper with a default system font
    pub fn new() -> Self {
        // Broad search for a usable font
        // Prioritize TTF to avoid potential TTC issues in basic loading
        let candidate_paths = [
            // macOS
            "/Library/Fonts/Arial.ttf",
            "/System/Library/Fonts/Supplemental/Arial.ttf",
            "/System/Library/Fonts/Helvetica.ttc",
            // Windows
            "C:\\Windows\\Fonts\\arial.ttf",
            "C:\\Windows\\Fonts\\seguiemj.ttf",
            // Linux
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "/usr/share/fonts/liberation/LiberationSans-Regular.ttf",
        ];

        let mut font_data: Option<&'static [u8]> = None;
        let mut face_index = 0;

        for path_str in candidate_paths.iter() {
            let path = Path::new(path_str);
            if path.exists() {
                if let Ok(bytes) = std::fs::read(path) {
                    println!("Loading font from: {}", path_str);
                    // Leak bytes to keep them static for HarfBuzz
                    font_data = Some(Box::leak(bytes.into_boxed_slice()));
                    break;
                }
            }
        }

        // Emergency fallback if no file found (e.g. CI environment)
        // In a real app we might embed a font, but for now we proceed or panic with clear message
        let bytes = font_data.expect("CRITICAL: No suitable system font found. Please install Arial or Helvetica.");
        
        // Create Face
        let face = unsafe { Face::from_bytes(bytes, face_index) };
        let mut font = Font::new(face);
        
        let upem = 1000; // Standardize for calculation, though we rely on HB's internal scaling
        font.set_scale(upem, upem);

        // Default 12pt font
        let font_size_pt = 12.0;
        
        // 1 pt = 1.333 px (96 DPI)
        // logical_px = points * (96 / 72) = points * 1.3333...
        // scale_factor = (font_size_pt * 1.3333) / upem
        let pixels_per_em = font_size_pt * (96.0 / 72.0);
        let scale_factor = pixels_per_em / (upem as f32);

        TextShaper {
            font,
            upem,
            font_size_pt,
            scale_factor,
        }
    }
    
    /// Create from specific bytes (for testing or specific loading)
    pub fn new_from_bytes(bytes: &'static [u8], font_size_pt: f32) -> Self {
         let face = unsafe { Face::from_bytes(bytes, 0) };
         let mut font = Font::new(face);
         let upem = 1000;
         font.set_scale(upem, upem);
         
         let pixels_per_em = font_size_pt * (96.0 / 72.0);
         let scale_factor = pixels_per_em / (upem as f32);

         TextShaper { font, upem, font_size_pt, scale_factor }
    }

    /// Shapes text and returns the total width and glyph infos in logical pixels
    pub fn shape(&self, text: &str) -> (f32, Vec<GlyphInfo>) {
        let buffer = UnicodeBuffer::new().add_str(text);
        let output = shape(&self.font, buffer, &[]);

        let positions = output.get_glyph_positions();
        let infos = output.get_glyph_infos();
        
        let mut total_width_px = 0.0;
        let mut glyphs = Vec::with_capacity(positions.len());

        for (position, info) in positions.iter().zip(infos.iter()) {
            let x_advance_px = position.x_advance as f32 * self.scale_factor;
            let y_advance_px = position.y_advance as f32 * self.scale_factor;
            let x_offset_px = position.x_offset as f32 * self.scale_factor;
            let y_offset_px = position.y_offset as f32 * self.scale_factor;

            total_width_px += x_advance_px;
            
            glyphs.push(GlyphInfo {
                codepoint: info.codepoint,
                cluster: info.cluster,
                x_advance: x_advance_px,
                y_advance: y_advance_px,
                x_offset: x_offset_px,
                y_offset: y_offset_px,
            });
        }
        
        (total_width_px, glyphs)
    }
    
    /// Measure text width in logical pixels
    pub fn measure_width(&self, text: &str) -> f32 {
        let (width, _) = self.shape(text);
        width
    }
}
