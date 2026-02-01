use harfbuzz_rs::{Face, Font, UnicodeBuffer, shape, Owned};
use std::sync::Arc;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Represents a shaped glyph with positioning information
#[derive(Debug, Clone, Copy)]
pub struct GlyphInfo {
    /// The glyph ID in the font
    pub codepoint: u32,
    /// The cluster index (character index) this glyph belongs to
    pub cluster: u32,
    /// X advance width
    pub x_advance: i32,
    /// Y advance height
    pub y_advance: i32,
    /// X offset
    pub x_offset: i32,
    /// Y offset
    pub y_offset: i32,
}

/// A text shaper that uses HarfBuzz
#[derive(Debug)]
pub struct TextShaper {
    font: Owned<Font<'static>>,
    scale: i32,
}

impl TextShaper {
    /// Creates a new text shaper with a default font
    /// Note: In a real app, you would load a specific font file
    pub fn new() -> Self {
        // For this milestone, we'll try to load a system font or fallback
        // Since we can't easily rely on a specific path, we might need to mock slightly
        // or require a font file to be present.
        // For now, let's assume we can load a standard font if available, 
        // or provide a mechanism to load bytes.
        
        // TODO: Load actual font bytes. 
        // For the purpose of this "1:1 Word Clone" which implies high fidelity,
        // we ideally need a real font. 
        // As a fallback for development if no file is found, we might panic or use a dummy.
        
        // For safety in this environment, let's create a "dummy" shaper if font fails,
        // but the goal is to use real HarfBuzz. 
        // We will try to read a common font location on macOS.
        
        let font_path = "/System/Library/Fonts/Helvetica.ttc"; // Common on macOS
        let index = 0;
        
        let face = match std::fs::read(font_path) {
            Ok(bytes) => {
                 // leak bytes to keep them static for HarfBuzz (common pattern for long lived fonts)
                 let bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
                 unsafe { Face::from_bytes(bytes, index) }
            },
            Err(_) => {
                // Fallback or panic? For this task, let's try another common one or fail.
                // We'll panic since .spec.md demands high fidelity.
                // Actually, let's try a safer fallback for CI/headless envs?
                // /System/Library/Fonts/MarkerFelt.ttc is less likely to be main.
                // Let's genericise.
                panic!("Could not load system font for TextShaper. Ensure '{}' exists.", font_path);
            }
        };

        let mut font = Font::new(face);
        // Set scale (often units per em, or desired pixel size * 64)
        // Let's assume standard 12pt at 96 DPI for now -> 16px
        // HarfBuzz uses 26.6 fixed point usually, or just units. 
        // Let's standardise to 1000 units per EM or similar if we can,
        // or just use the font's upem.
        // For simplicity, let's set it to a high resolution.
        font.set_scale(1000, 1000); 

        TextShaper {
            font,
            scale: 1000,
        }
    }
    
    pub fn new_from_bytes(bytes: &'static [u8]) -> Self {
         let face = unsafe { Face::from_bytes(bytes, 0) };
         let mut font = Font::new(face);
         font.set_scale(1000, 1000);
         TextShaper { font, scale: 1000 }
    }

    /// Shapes text and returns the total width and glyph infos
    pub fn shape(&self, text: &str) -> (f32, Vec<GlyphInfo>) {
        let buffer = UnicodeBuffer::new().add_str(text);
        let output = shape(&self.font, buffer, &[]);

        let positions = output.get_glyph_positions();
        let infos = output.get_glyph_infos();
        
        let mut total_width_units = 0;
        let mut glyphs = Vec::with_capacity(positions.len());

        for (position, info) in positions.iter().zip(infos.iter()) {
            total_width_units += position.x_advance;
            
            glyphs.push(GlyphInfo {
                codepoint: info.codepoint,
                cluster: info.cluster,
                x_advance: position.x_advance,
                y_advance: position.y_advance,
                x_offset: position.x_offset,
                y_offset: position.y_offset,
            });
        }
        
        // Convert units to abstract pixels (if we assume 1000 units = 1 EM usually ~12-16px)
        // For 1:1 replica, we need precise control.
        // Let's normalize: if scale is 1000, and we want 12pt font (~16px),
        // width_px = width_units * (font_size_px / scale)
        // We'll return units for now or normalized? 
        // Let's return raw units and let layout scale it, OR normalize to "1.0 = 1 EM".
        
        let width_em = total_width_units as f32 / self.scale as f32;
        
        (width_em, glyphs)
    }
    
    pub fn measure_width(&self, text: &str) -> f32 {
        let (width, _) = self.shape(text);
        width
    }
}
