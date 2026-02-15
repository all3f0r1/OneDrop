# OneDrop - Project Context for Claude Code

A pure Rust implementation of Milkdrop visualization engine with 98% preset compatibility.

## Project Overview

OneDrop is a modern, cross-platform music visualizer built entirely in Rust. It provides high-performance audio-reactive visualizations with full compatibility with `.milk` preset files.

## Workspace Structure

8 interconnected crates:

| Crate | Purpose | Key Files |
|-------|---------|-----------|
| `onedrop-parser` | Parse .milk preset files | `src/parser.rs`, `src/preset.rs` |
| `onedrop-eval` | Expression evaluation with 30+ math functions | `src/evaluator.rs`, `src/math_functions.rs` |
| `onedrop-renderer` | GPU rendering pipeline (wgpu) | `src/renderer.rs`, `src/pipelines/` |
| `onedrop-engine` | Main engine assembling all components | `src/engine.rs`, `src/audio.rs`, `src/beat_detection.rs` |
| `onedrop-cli` | Command-line interface | `src/main.rs` |
| `onedrop-gui` | GUI application (winit) | `src/main.rs` |
| `onedrop-hlsl` | HLSL shader utilities | `src/lib.rs`, `src/advanced.rs` |
| `onedrop-codegen` | Code generation tools | `src/generator.rs` |

## Key Technologies

- **GPU**: wgpu 23.0 (Vulkan, Metal, DX12, OpenGL)
- **Window**: winit 0.30
- **CLI**: clap 4.5 (derive macros)
- **Audio**: cpal 0.15, rustfft 6.1
- **Math**: glam 0.29
- **Error Handling**: thiserror 2.0, anyhow 1.0

## Architecture Decisions

### Shared GPU Context (v0.9.0)
The renderer uses a shared `GpuContext` pattern for efficient resource management across pipelines. See `onedrop-renderer/src/gpu_context.rs`.

### Error Recovery Philosophy
The engine **never crashes** - it always continues rendering even with invalid presets. See `onedrop-engine/src/error.rs` for error types and `onedrop-engine/src/safe_loader.rs` for preset loading safety.

### Beat Detection
6 HardCut modes implemented (MilkDrop3 complete). See `onedrop-engine/src/beat_detection.rs`.

### Audio Input (Optional)
Enable with feature flag: `cargo build --features audio-input`. Uses cpal for cross-platform audio capture and rustfft for spectrum analysis.

## Commands

```bash
# Build all crates
cargo build --all

# Build release (optimized)
cargo build --release --all

# Run GUI
cargo run --release -p onedrop-gui

# Run CLI
cargo run --release -p onedrop-cli -- info preset.milk
cargo run --release -p onedrop-cli -- validate preset.milk
cargo run --release -p onedrop-cli -- render preset.milk --frames 120

# Run tests
cargo test --all

# Run with audio support
cargo run --release -p onedrop-gui --features audio-input

# Check all code
cargo clippy --all --all-targets -- -D warnings

# Generate docs
cargo doc --open
```

## Testing

- **94+ tests** across all crates
- Integration tests in `tests/` directories
- Benchmarks in `benches/` directories
- Preset compatibility tests in `scripts/test_all_presets.sh`
- 200 test presets in `test-presets-200/`

### Preset Compatibility

- **Overall**: 98% (49/50 tested)
- Simple presets: 100% (19/19)
- Medium presets: 100% (20/20)
- Complex presets: 91% (10/11)

## Roadmap

- v0.9.0 (Current) ✅: milkif() function, 98% compatibility
- v1.0.0: GUI rendering complete, audio input, binary releases
- v1.1.0: Per-pixel equations, custom shapes, waveform modes
- v1.2.0: Double-preset rendering, blend patterns, transitions

## Code Style

- Standard Rust formatting (`cargo fmt`)
- Clippy compliance required
- Comprehensive inline documentation
- No `unwrap()` in production code paths (use `?` or proper error handling)
- Safety comments required for all `unsafe` blocks

## Repository

- GitHub: https://github.com/all3f0r1/OneDrop
- License: MIT
- Rust version: 1.70+

## Files to Avoid Editing

- `Cargo.lock` - Generated, excluded from git
- `target/` - Build artifacts
- `test-presets/` - Large preset collections (excluded from git)
