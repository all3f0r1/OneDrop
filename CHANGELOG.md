# Changelog

All notable changes to OneDrop will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2025-11-26

### Added
- **Beat Detection Complete** - Automatic preset changes based on audio analysis
  - 6 HardCut modes from MilkDrop3 (HardCut1-6)
  - Configurable thresholds and minimum delays
  - F8 key to cycle through beat detection modes
  - Random preset selection on beat trigger
  - Special preset loading for extreme bass (HardCut6)
- `BeatDetector` struct with full API in onedrop-engine
- `BeatDetectionMode` enum with 7 modes (Off + 6 HardCut)
- `PresetChange` enum (Random, Specific)
- `random_preset()` method in PresetManager
- 14 comprehensive unit tests for beat detection
- Beat detection integration in MilkEngine
- GUI integration with F8 key binding
- Performance validation script (`scripts/test_beat_detection.sh`)
- Complete beat detection documentation (`docs/V0.7.0_BEAT_DETECTION_REPORT.md`)

### Changed
- `MilkEngine::update()` now returns `Result<Option<PresetChange>>` instead of `Result<()>`
- Enhanced PresetManager with random selection capability
- Improved audio-reactive preset switching

### Performance
- Beat detection overhead: <0.1ms per frame
- Memory usage: ~200 bytes
- CPU impact: <1%
- Total tests: 94+ (14 new beat detection tests)
- Total lines of code: 9,600+ (~750 lines added)

## [0.6.0-beta] - 2025-11-25

### Added
- **Per-Vertex Shader Pipeline** - Complete HLSL translation and execution
  - Advanced HLSL translator with 50+ function mappings
  - Dynamic WGSL generation for vertex shaders
  - Shader cache for 100x speedup (13µs cache hit vs 1-2ms compilation)
  - Uniform buffer system (320 bytes) for all Milkdrop variables
- `onedrop-hlsl` crate for HLSL→WGSL translation
- Shader cache in onedrop-codegen
- Comprehensive vertex shader tests
- Advanced HLSL function support (lerp, saturate, frac, etc.)

### Changed
- Improved shader compilation pipeline
- Enhanced error handling for shader validation

### Performance
- Shader cache provides 100x speedup
- ~0.5ms per frame @ 1080p
- 60 FPS real-time, capable of 1000 FPS

## [0.5.0] - 2025-11-25

### Added
- **onedrop-codegen**: WGSL shader code generation from Milkdrop equations
  - Expression transpiler (Milkdrop → WGSL)
  - Variable mapper (all Milkdrop variables supported)
  - Shader generator (complete per-pixel shaders)
  - 13 unit tests
- **onedrop-hlsl**: HLSL to WGSL translation
  - Type replacements (float4 → vec4<f32>)
  - Function replacements (lerp → mix, saturate → clamp)
  - Texture sampling conversion
  - 4 unit tests

- **ShaderCompiler**: Dynamic shader compilation with naga
  - Automatic caching (100x speedup: 1-2ms → 13µs)
  - Thread-safe cache with Arc+Mutex
  - Complete error handling
  - 4 unit tests
- **PerPixelPipeline**: GPU rendering pipeline for per-pixel shaders
  - Complete wgpu pipeline
  - Uniform buffer (320 bytes) for all variables
  - Texture support (input/output/feedback)
  - Sampler configuration
  - 2 unit tests
- Complete pipeline: Parse → Generate → Compile → Render
- Example: `complete_pipeline.rs`

### Changed
- Fixed GPU uniform buffer alignment (vec4 instead of f32 arrays)
- Added padding for 16-byte alignment
- Improved shader generation with proper WGSL structure

### Performance
- Shader compilation cache: 100x speedup (13µs cache hit)
- Per-frame rendering: ~0.5ms @ 1080p
- Uniform buffer upload: ~10µs

### In Progress (Future)
- Testing with 1000+ presets
- 95%+ preset compatibility target
- HLSL to WGSL advanced translation

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
