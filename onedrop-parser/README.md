# onedrop-parser

A Rust parser for Milkdrop `.milk` preset files.

## Overview

This crate provides functionality to parse Milkdrop visualization presets into structured Rust data types. It supports the full Milkdrop preset format including:

- Static parameters (zoom, rotation, colors, etc.)
- Per-frame equations (executed once per frame)
- Per-pixel equations (executed for each pixel)
- Custom waveforms (up to 4)
- Custom shapes (up to 4)
- HLSL/GLSL shader code (warp and composite shaders)

## Usage

```rust
use onedrop_parser::parse_preset;
use std::fs;

// Read a .milk file
let content = fs::read_to_string("preset.milk")?;

// Parse it
let preset = parse_preset(&content)?;

// Access parsed data
println!("Preset version: {}", preset.version);
println!("Zoom: {}", preset.parameters.zoom);
println!("Per-frame equations: {}", preset.per_frame_equations.len());
```

## Features

- **Zero-copy parsing** where possible for performance
- **Comprehensive error handling** with detailed error messages
- **Serde support** for serialization/deserialization
- **Well-tested** with real-world presets

## Preset Structure

A parsed preset contains:

```rust
pub struct MilkPreset {
    pub version: u32,
    pub ps_version_warp: u32,
    pub ps_version_comp: u32,
    pub parameters: PresetParameters,
    pub per_frame_equations: Vec<String>,
    pub per_pixel_equations: Vec<String>,
    pub per_frame_init_equations: Vec<String>,
    pub waves: Vec<WaveCode>,
    pub shapes: Vec<ShapeCode>,
    pub warp_shader: Option<String>,
    pub comp_shader: Option<String>,
}
```

## Testing

Run tests with:

```bash
cargo test
```

Integration tests use real Milkdrop presets from the `presets-cream-of-the-crop` repository.

## License

MIT

## Part of OneAmp

This crate is part of the OneAmp audio player project, a modern Rust-based audio player with Milkdrop visualization support.
