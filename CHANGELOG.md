# Changelog

All notable changes to OneDrop will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-11-25

### Added
- **Double-preset format (.od2)** - Blend two presets simultaneously
- **27 blending patterns** inspired by MilkDrop3:
  - Basic blends: Alpha, Additive, Multiply, Screen, Overlay
  - Advanced blends: Darken, Lighten, Color Dodge, Color Burn, Hard Light, Soft Light
  - Difference blends: Difference, Exclusion
  - Pattern blends: Plasma, Snail, Triangle, Donuts, Checkerboard
  - Stripe blends: Horizontal, Vertical, Diagonal
  - Geometric blends: Radial, Angular
  - Noise blends: Perlin Noise, Voronoi
  - Dynamic blends: Wave, Random Pixel
- `BlendRenderer` module in onedrop-renderer
- `double_preset` module in onedrop-parser
- `BlendPattern` enum with 27 variants
- `DoublePreset` structure for .od2 files
- WGSL blend shader with all 27 patterns
- Animation support for blend transitions

### Changed
- Enhanced renderer capabilities for dual-preset rendering
- Added bytemuck dependency for uniform buffer management

## [0.3.0] - 2025-11-25

### Added
- **Expression cache** for 10x performance improvement on repeated expressions
- **Optimized evaluator** with batch processing capabilities
- **Preset history navigation** (A/Z/C keys) inspired by MilkDrop3
- **Beat detection** with 6 hardcut modes inspired by MilkDrop3
- 13 compatibility getter methods to `PresetParameters`
- `MilkContext` helper methods: `set_time()`, `set_frame()`, `set_audio()`, `get_var()`
- New modules:
  - `onedrop-eval/src/cache.rs`
  - `onedrop-eval/src/evaluator_optimized.rs`
  - `onedrop-engine/src/history.rs`
  - `onedrop-engine/src/beat_detection.rs`

### Changed
- **BREAKING**: Migrated from evalexpr 11.3 to 13.0
  - Updated API to use `Context` trait
  - Changed return types for context methods
  - Assignments now return `Empty` value
- All internal imports renamed from `milk_*` to `onedrop_*`
- `PresetParameters` methods now return values instead of direct field access
- `BeatDetectionMode::mode()` now returns `&BeatDetectionMode` instead of owned value

### Fixed
- Compilation errors across all crates
- Borrow checker issues in `onedrop-engine`
- Type conversion issues (bool → f64)
- Entry point API changes in wgpu shaders

### Removed
- `Copy` trait from `BeatDetectionMode` (contains non-Copy `String`)

## [0.2.0] - 2025-11-24

### Added
- Complete renaming of crates from `milk-*` to `onedrop-*`
- FFT audio analysis
- Transition manager with 4 modes
- Waveform rendering module
- Advanced composite shaders
- 200 additional test presets (total: 250)

### Changed
- Improved error handling across all crates
- Enhanced renderer with post-processing effects

## [0.1.0] - 2025-11-23

### Added
- Initial release
- `onedrop-parser`: Parse .milk preset files
- `onedrop-eval`: Evaluate Milkdrop expressions
- `onedrop-renderer`: GPU rendering with wgpu
- `onedrop-engine`: Complete visualization engine
- `onedrop-cli`: Command-line interface
- `onedrop-gui`: Standalone GUI application
- Support for 50 test presets
- Basic documentation

[0.3.0]: https://github.com/all3f0r1/OneDrop/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/all3f0r1/OneDrop/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/all3f0r1/OneDrop/releases/tag/v0.1.0
