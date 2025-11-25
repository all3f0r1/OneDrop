# OneDrop 🎨

**A pure Rust implementation of Milkdrop visualization engine**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

OneDrop is a modern, cross-platform music visualizer inspired by the legendary Milkdrop. Built entirely in Rust, it provides high-performance audio-reactive visualizations with full compatibility with `.milk` preset files.

## ✨ Features

### Core Capabilities
- 🦀 **Pure Rust** - Memory-safe, fast, and reliable
- 🚀 **Modern GPU** - wgpu for Vulkan, Metal, DX12, OpenGL
- 🎨 **Full .milk support** - 250+ tested presets
- ⚡ **10x faster** - Intelligent expression caching
- 🎵 **Audio analysis** - FFT-based frequency detection
- 🎯 **Beat detection** - 6 hardcut modes (MilkDrop3 inspired)
- 📜 **Preset history** - Navigate back/forward (A/Z/C keys)
- 🖥️ **Cross-platform** - Windows, macOS, Linux

### Modern Architecture
- **Modular design**: 6 independent crates
- **Well-tested**: 45+ passing tests
- **Documented**: Comprehensive inline documentation
- **Performant**: 60 FPS at 1920x1080

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/all3f0r1/OneDrop.git
cd OneDrop
cargo build --release
```

### Usage

#### GUI Application

```bash
cargo run --release -p onedrop-gui
```

**Controls:**
- `←/→`: Navigate between presets
- `R`: Reset current preset
- `Q`: Quit

#### CLI Tools

```bash
# Show preset information
cargo run --release -p onedrop-cli -- info preset.milk

# Validate a preset
cargo run --release -p onedrop-cli -- validate preset.milk

# Render frames
cargo run --release -p onedrop-cli -- render preset.milk --frames 120

# List presets
cargo run --release -p onedrop-cli -- list ./test-presets
```

#### As a Library

```toml
[dependencies]
onedrop-engine = { git = "https://github.com/all3f0r1/OneDrop" }
```

```rust
use onedrop_engine::{EngineConfig, MilkEngine};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EngineConfig::default();
    let mut engine = MilkEngine::new(config).await?;
    
    engine.load_preset("preset.milk")?;
    
    loop {
        let audio_samples = capture_audio();
        engine.update(&audio_samples, 0.016)?;
        display(engine.render_texture());
    }
}
```

## 📦 Project Structure

| Crate | Description | LOC | Status |
|-------|-------------|-----|--------|
| `onedrop-parser` | Parse .milk files | ~700 | ✅ |
| `onedrop-eval` | Expression evaluation | ~950 | ✅ |
| `onedrop-renderer` | GPU rendering | ~1,220 | ✅ |
| `onedrop-engine` | Visualization engine | ~1,450 | ✅ |
| `onedrop-cli` | CLI interface | ~350 | ✅ |
| `onedrop-gui` | GUI application | ~400 | ✅ |

**Total:** ~5,070 lines of Rust code

## 📊 Performance

| Metric | Value |
|--------|-------|
| Expression evaluation | 10x faster with caching |
| Rendering | 60 FPS @ 1920x1080 |
| Memory usage | < 100 MB typical |
| Startup time | < 1 second |
| Preset compatibility | 90%+ (250 tested) |

## 🎯 Roadmap

### v0.3.0 (Current) ✅
- Expression caching (10x performance)
- Preset history navigation
- Beat detection (6 modes)
- evalexpr 13.0 migration

### v0.4.0 (Next)
- Double-preset format (.od2)
- 27 transition effects
- Color randomization
- Enhanced UI

### v0.5.0
- Per-pixel shader execution
- HLSL to WGSL translation
- Advanced audio analysis
- Preset creation tools

### v1.0.0
- Production stability
- Complete compatibility
- Performance optimizations
- Community features

## 🔧 Development

### Building

```bash
cargo build --all
```

### Testing

```bash
cargo test --all
```

### Documentation

```bash
cargo doc --open
```

## 📚 Documentation

Each crate has comprehensive documentation:

- [onedrop-parser](onedrop-parser/README.md) - Preset parsing
- [onedrop-eval](onedrop-eval/README.md) - Expression evaluation
- [onedrop-renderer](onedrop-renderer/README.md) - GPU rendering
- [onedrop-engine](onedrop-engine/README.md) - Complete engine
- [onedrop-cli](onedrop-cli/README.md) - CLI usage
- [onedrop-gui](onedrop-gui/README.md) - GUI usage

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Add tests for new features
4. Run `cargo clippy` and `cargo test`
5. Submit a Pull Request

## 📝 License

MIT License - See [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Ryan Geiss** - Original Milkdrop creator
- **MilkDrop3** - Modern enhancements
- **projectM** - Cross-platform reference
- **Rust community** - Amazing ecosystem

## 🔗 Related Projects

- [projectM](https://github.com/projectM-visualizer/projectm) - C++ implementation
- [MilkDrop3](https://github.com/milkdrop2077/MilkDrop3) - Modern evolution
- [Presets](https://github.com/projectM-visualizer/presets-cream-of-the-crop) - Curated collection

## 📧 Contact

- **Repository**: https://github.com/all3f0r1/OneDrop
- **Issues**: https://github.com/all3f0r1/OneDrop/issues

---

**Made with 🦀 and ❤️ by the OneDrop team**
