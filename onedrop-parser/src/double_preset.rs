//! Double-preset format (.od2) - Blend two presets simultaneously
//!
//! Inspired by MilkDrop3's .milk2 format, this allows blending two presets
//! with 27 different blending patterns for creative combinations.

use crate::preset::MilkPreset;
use crate::error::{ParseError, Result};
use serde::{Deserialize, Serialize};

/// A double-preset that blends two presets together.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoublePreset {
    /// First preset (A)
    pub preset_a: MilkPreset,
    
    /// Second preset (B)
    pub preset_b: MilkPreset,
    
    /// Blending pattern
    pub blend_pattern: BlendPattern,
    
    /// Blend amount (0.0 = all A, 1.0 = all B)
    pub blend_amount: f32,
    
    /// Whether to animate the blend
    pub animate_blend: bool,
    
    /// Animation speed (if animate_blend is true)
    pub animation_speed: f32,
}

/// Blending patterns for double-presets.
///
/// These 27 patterns are inspired by MilkDrop3 and provide various
/// ways to combine two presets visually.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlendPattern {
    /// Simple alpha blend
    Alpha = 0,
    
    /// Additive blending
    Additive = 1,
    
    /// Multiply blending
    Multiply = 2,
    
    /// Screen blending
    Screen = 3,
    
    /// Overlay blending
    Overlay = 4,
    
    /// Darken (min)
    Darken = 5,
    
    /// Lighten (max)
    Lighten = 6,
    
    /// Color dodge
    ColorDodge = 7,
    
    /// Color burn
    ColorBurn = 8,
    
    /// Hard light
    HardLight = 9,
    
    /// Soft light
    SoftLight = 10,
    
    /// Difference
    Difference = 11,
    
    /// Exclusion
    Exclusion = 12,
    
    /// Plasma blend
    Plasma = 13,
    
    /// Snail blend (spiral pattern)
    Snail = 14,
    
    /// Triangle blend
    Triangle = 15,
    
    /// Donuts blend (circular pattern)
    Donuts = 16,
    
    /// Checkerboard blend
    Checkerboard = 17,
    
    /// Horizontal stripes
    HorizontalStripes = 18,
    
    /// Vertical stripes
    VerticalStripes = 19,
    
    /// Diagonal stripes
    DiagonalStripes = 20,
    
    /// Radial blend (from center)
    Radial = 21,
    
    /// Angular blend (rotating)
    Angular = 22,
    
    /// Perlin noise blend
    PerlinNoise = 23,
    
    /// Voronoi blend
    Voronoi = 24,
    
    /// Wave blend (sine wave pattern)
    Wave = 25,
    
    /// Random pixel blend
    RandomPixel = 26,
}

impl BlendPattern {
    /// Get all available blend patterns.
    pub fn all() -> Vec<BlendPattern> {
        vec![
            BlendPattern::Alpha,
            BlendPattern::Additive,
            BlendPattern::Multiply,
            BlendPattern::Screen,
            BlendPattern::Overlay,
            BlendPattern::Darken,
            BlendPattern::Lighten,
            BlendPattern::ColorDodge,
            BlendPattern::ColorBurn,
            BlendPattern::HardLight,
            BlendPattern::SoftLight,
            BlendPattern::Difference,
            BlendPattern::Exclusion,
            BlendPattern::Plasma,
            BlendPattern::Snail,
            BlendPattern::Triangle,
            BlendPattern::Donuts,
            BlendPattern::Checkerboard,
            BlendPattern::HorizontalStripes,
            BlendPattern::VerticalStripes,
            BlendPattern::DiagonalStripes,
            BlendPattern::Radial,
            BlendPattern::Angular,
            BlendPattern::PerlinNoise,
            BlendPattern::Voronoi,
            BlendPattern::Wave,
            BlendPattern::RandomPixel,
        ]
    }
    
    /// Get pattern name.
    pub fn name(&self) -> &'static str {
        match self {
            BlendPattern::Alpha => "Alpha",
            BlendPattern::Additive => "Additive",
            BlendPattern::Multiply => "Multiply",
            BlendPattern::Screen => "Screen",
            BlendPattern::Overlay => "Overlay",
            BlendPattern::Darken => "Darken",
            BlendPattern::Lighten => "Lighten",
            BlendPattern::ColorDodge => "Color Dodge",
            BlendPattern::ColorBurn => "Color Burn",
            BlendPattern::HardLight => "Hard Light",
            BlendPattern::SoftLight => "Soft Light",
            BlendPattern::Difference => "Difference",
            BlendPattern::Exclusion => "Exclusion",
            BlendPattern::Plasma => "Plasma",
            BlendPattern::Snail => "Snail",
            BlendPattern::Triangle => "Triangle",
            BlendPattern::Donuts => "Donuts",
            BlendPattern::Checkerboard => "Checkerboard",
            BlendPattern::HorizontalStripes => "Horizontal Stripes",
            BlendPattern::VerticalStripes => "Vertical Stripes",
            BlendPattern::DiagonalStripes => "Diagonal Stripes",
            BlendPattern::Radial => "Radial",
            BlendPattern::Angular => "Angular",
            BlendPattern::PerlinNoise => "Perlin Noise",
            BlendPattern::Voronoi => "Voronoi",
            BlendPattern::Wave => "Wave",
            BlendPattern::RandomPixel => "Random Pixel",
        }
    }
    
    /// Get pattern from index.
    pub fn from_index(index: usize) -> Option<BlendPattern> {
        match index {
            0 => Some(BlendPattern::Alpha),
            1 => Some(BlendPattern::Additive),
            2 => Some(BlendPattern::Multiply),
            3 => Some(BlendPattern::Screen),
            4 => Some(BlendPattern::Overlay),
            5 => Some(BlendPattern::Darken),
            6 => Some(BlendPattern::Lighten),
            7 => Some(BlendPattern::ColorDodge),
            8 => Some(BlendPattern::ColorBurn),
            9 => Some(BlendPattern::HardLight),
            10 => Some(BlendPattern::SoftLight),
            11 => Some(BlendPattern::Difference),
            12 => Some(BlendPattern::Exclusion),
            13 => Some(BlendPattern::Plasma),
            14 => Some(BlendPattern::Snail),
            15 => Some(BlendPattern::Triangle),
            16 => Some(BlendPattern::Donuts),
            17 => Some(BlendPattern::Checkerboard),
            18 => Some(BlendPattern::HorizontalStripes),
            19 => Some(BlendPattern::VerticalStripes),
            20 => Some(BlendPattern::DiagonalStripes),
            21 => Some(BlendPattern::Radial),
            22 => Some(BlendPattern::Angular),
            23 => Some(BlendPattern::PerlinNoise),
            24 => Some(BlendPattern::Voronoi),
            25 => Some(BlendPattern::Wave),
            26 => Some(BlendPattern::RandomPixel),
            _ => None,
        }
    }
}

impl Default for DoublePreset {
    fn default() -> Self {
        Self {
            preset_a: MilkPreset::default(),
            preset_b: MilkPreset::default(),
            blend_pattern: BlendPattern::Alpha,
            blend_amount: 0.5,
            animate_blend: false,
            animation_speed: 1.0,
        }
    }
}

impl DoublePreset {
    /// Create a new double-preset from two presets.
    pub fn new(preset_a: MilkPreset, preset_b: MilkPreset) -> Self {
        Self {
            preset_a,
            preset_b,
            blend_pattern: BlendPattern::Alpha,
            blend_amount: 0.5,
            animate_blend: false,
            animation_speed: 1.0,
        }
    }
    
    /// Create with specific blend pattern.
    pub fn with_pattern(mut self, pattern: BlendPattern) -> Self {
        self.blend_pattern = pattern;
        self
    }
    
    /// Set blend amount.
    pub fn with_blend_amount(mut self, amount: f32) -> Self {
        self.blend_amount = amount.clamp(0.0, 1.0);
        self
    }
    
    /// Enable animation.
    pub fn with_animation(mut self, speed: f32) -> Self {
        self.animate_blend = true;
        self.animation_speed = speed;
        self
    }
}

/// Parse a .od2 double-preset file.
pub fn parse_double_preset(content: &str) -> Result<DoublePreset> {
    // .od2 format:
    // [DoublePreset]
    // BlendPattern=<index>
    // BlendAmount=<0.0-1.0>
    // AnimateBlend=<0|1>
    // AnimationSpeed=<float>
    //
    // [PresetA]
    // <preset A content>
    //
    // [PresetB]
    // <preset B content>
    
    let mut blend_pattern = BlendPattern::Alpha;
    let mut blend_amount = 0.5;
    let mut animate_blend = false;
    let mut animation_speed = 1.0;
    
    let mut preset_a_content = String::new();
    let mut preset_b_content = String::new();
    
    let mut current_section = "";
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with('[') && line.ends_with(']') {
            current_section = &line[1..line.len()-1];
            continue;
        }
        
        match current_section {
            "DoublePreset" => {
                if let Some((key, value)) = line.split_once('=') {
                    match key.trim() {
                        "BlendPattern" => {
                            if let Ok(index) = value.trim().parse::<usize>() {
                                blend_pattern = BlendPattern::from_index(index)
                                    .unwrap_or(BlendPattern::Alpha);
                            }
                        }
                        "BlendAmount" => {
                            if let Ok(amount) = value.trim().parse::<f32>() {
                                blend_amount = amount.clamp(0.0, 1.0);
                            }
                        }
                        "AnimateBlend" => {
                            animate_blend = value.trim() == "1";
                        }
                        "AnimationSpeed" => {
                            if let Ok(speed) = value.trim().parse::<f32>() {
                                animation_speed = speed;
                            }
                        }
                        _ => {}
                    }
                }
            }
            "PresetA" => {
                preset_a_content.push_str(line);
                preset_a_content.push('\n');
            }
            "PresetB" => {
                preset_b_content.push_str(line);
                preset_b_content.push('\n');
            }
            _ => {}
        }
    }
    
    // Parse both presets
    let preset_a = crate::parse_preset(&preset_a_content)?;
    let preset_b = crate::parse_preset(&preset_b_content)?;
    
    Ok(DoublePreset {
        preset_a,
        preset_b,
        blend_pattern,
        blend_amount,
        animate_blend,
        animation_speed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_blend_pattern_all() {
        let patterns = BlendPattern::all();
        assert_eq!(patterns.len(), 27);
    }
    
    #[test]
    fn test_blend_pattern_from_index() {
        assert_eq!(BlendPattern::from_index(0), Some(BlendPattern::Alpha));
        assert_eq!(BlendPattern::from_index(13), Some(BlendPattern::Plasma));
        assert_eq!(BlendPattern::from_index(26), Some(BlendPattern::RandomPixel));
        assert_eq!(BlendPattern::from_index(27), None);
    }
    
    #[test]
    fn test_double_preset_creation() {
        let preset_a = MilkPreset::default();
        let preset_b = MilkPreset::default();
        
        let double = DoublePreset::new(preset_a, preset_b)
            .with_pattern(BlendPattern::Plasma)
            .with_blend_amount(0.7)
            .with_animation(2.0);
        
        assert_eq!(double.blend_pattern, BlendPattern::Plasma);
        assert_eq!(double.blend_amount, 0.7);
        assert!(double.animate_blend);
        assert_eq!(double.animation_speed, 2.0);
    }
}
