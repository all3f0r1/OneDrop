//! # onedrop-parser
//!
//! A parser for Milkdrop `.milk` preset files.
//!
//! This crate provides functionality to parse Milkdrop visualization presets
//! into structured Rust data types that can be used for rendering or analysis.

pub mod double_preset;
pub mod error;
pub mod parser;
pub mod preset;

pub use double_preset::{BlendPattern, DoublePreset, parse_double_preset};
pub use error::{ParseError, Result};
pub use preset::MilkPreset;

/// Parse a `.milk` preset file from a string.
///
/// # Examples
///
/// ```
/// use onedrop_parser::parse_preset;
///
/// let content = r#"MILKDROP_PRESET_VERSION=201
/// [preset00]
/// fRating=5.000000
/// zoom=0.99197
/// "#;
/// let preset = parse_preset(content).unwrap();
/// println!("Preset version: {}", preset.version);
/// ```
pub fn parse_preset(input: &str) -> Result<MilkPreset> {
    parser::parse_milk_preset(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_preset() {
        let input = r#"MILKDROP_PRESET_VERSION=201
PSVERSION=2
[preset00]
fRating=5.000000
zoom=0.99197
rot=0.00000
cx=0.500
cy=0.500
wave_r=1.000
wave_g=0.000
wave_b=0.000
per_frame_1=wave_r = 0.5;
per_pixel_1=zoom=zoom+0.1;
"#;

        let result = parse_preset(input);
        assert!(result.is_ok());
        
        let preset = result.unwrap();
        assert_eq!(preset.version, 201);
    }
}
