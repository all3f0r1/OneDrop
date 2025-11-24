# OneDrop

**A modern, pure-Rust reimplementation of the Milkdrop music visualizer.**

OneDrop is a complete rewrite of the legendary Milkdrop visualization engine in Rust, designed for performance, safety, and modern GPU standards. It maintains full compatibility with the `.milk` preset format while leveraging modern technologies like `wgpu` for cross-platform GPU acceleration.

## Project Status

🚧 **Active Development** - Currently implementing core components

### Completed
- ✅ **milk-parser** - Complete parser for `.milk` preset files

### In Progress
- 🔨 **milk-eval** - Expression evaluator for per-frame and per-pixel equations

### Planned
- 📋 **milk-renderer** - GPU rendering pipeline with wgpu
- 📋 **milk-engine** - Complete visualization engine
- 📋 **onedrop-cli** - Command-line interface
- 📋 **onedrop-gui** - Standalone GUI application

## Architecture

OneDrop is structured as a workspace of independent crates:

```
OneDrop/
├── milk-parser/      # Parse .milk preset files
├── milk-eval/        # Evaluate mathematical expressions
├── milk-renderer/    # GPU rendering with wgpu
├── milk-engine/      # Complete visualization engine
├── onedrop-cli/      # CLI application
└── onedrop-gui/      # GUI application
```

## Features

### Current
- **Complete .milk parsing** - Supports all Milkdrop preset features
  - Static parameters (zoom, rotation, colors, etc.)
  - Per-frame equations
  - Per-pixel equations
  - Custom waveforms (up to 4)
  - Custom shapes (up to 4)
  - HLSL/GLSL shader code

### Planned
- **Pure Rust implementation** - No C/C++ dependencies
- **Modern GPU acceleration** - Using wgpu for Vulkan/Metal/DX12/OpenGL
- **Cross-platform** - Windows, macOS, Linux, and WebAssembly
- **High performance** - Optimized expression evaluation and rendering
- **Preset compatibility** - Works with 130,000+ existing Milkdrop presets

## Usage

### Parsing a preset

```rust
use milk_parser::parse_preset;
use std::fs;

let content = fs::read_to_string("preset.milk")?;
let preset = parse_preset(&content)?;

println!("Version: {}", preset.version);
println!("Zoom: {}", preset.parameters.zoom);
println!("Per-frame equations: {}", preset.per_frame_equations.len());
```

## Building

```bash
# Clone the repository
git clone https://github.com/yourusername/OneDrop.git
cd OneDrop

# Build all crates
cargo build --release

# Run tests
cargo test

# Run with a specific preset
cargo run --release -- preset.milk
```

## Testing

OneDrop is tested against real-world presets from the official [presets-cream-of-the-crop](https://github.com/projectM-visualizer/presets-cream-of-the-crop) collection.

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific crate tests
cargo test -p milk-parser
```

## Roadmap

### Phase 1: Foundation (Weeks 1-2) ✅
- [x] Create project structure
- [x] Implement milk-parser
- [x] Test with real presets

### Phase 2: Expression Evaluation (Weeks 2-3) 🔨
- [ ] Implement milk-eval
- [ ] Support all Milkdrop variables
- [ ] Support all Milkdrop functions
- [ ] Optimize for performance

### Phase 3: Rendering (Weeks 4-6)
- [ ] Implement milk-renderer with wgpu
- [ ] Basic waveform rendering
- [ ] Per-pixel shader execution
- [ ] Custom shapes and waves

### Phase 4: Engine Integration (Week 7)
- [ ] Assemble milk-engine
- [ ] Audio input handling
- [ ] Preset transitions
- [ ] Beat detection

### Phase 5: Applications (Week 8)
- [ ] CLI application
- [ ] GUI application
- [ ] Integration with audio players

## Compatibility

OneDrop aims for maximum compatibility with existing Milkdrop presets:

- **Target**: 95%+ compatibility with popular presets
- **Format**: Full support for `.milk` format version 201
- **Extensions**: Support for MilkDrop3 extensions (q1-q64, etc.)

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

MIT License - See LICENSE file for details

## Acknowledgments

- **Ryan Geiss** - Original Milkdrop creator
- **projectM team** - Reference implementation
- **ISOSCELES** - Curated preset collection
- **Rust community** - Amazing ecosystem

## Related Projects

- [projectM](https://github.com/projectM-visualizer/projectm) - C++ reference implementation
- [MilkDrop3](https://github.com/milkdrop2077/MilkDrop3) - Modern Milkdrop evolution
- [OneAmp](https://github.com/yourusername/oneamp) - Rust audio player (integration target)
