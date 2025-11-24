# onedrop-gui

Graphical user interface for OneDrop Milkdrop visualizer.

## Overview

`onedrop-gui` is a standalone desktop application for running Milkdrop visualizations. It provides a window with real-time visualization rendering and keyboard controls.

## Features

- **Real-time rendering** - 60 FPS visualization
- **Preset management** - Load and switch between presets
- **Keyboard controls** - Navigate presets and control playback
- **Cross-platform** - Works on Windows, macOS, and Linux
- **GPU-accelerated** - Uses wgpu for high performance

## Installation

```bash
cd onedrop-gui
cargo build --release
```

Run:
```bash
cargo run --release
```

Or install:
```bash
cargo install --path .
onedrop-gui
```

## Controls

| Key | Action |
|-----|--------|
| `Space` | Toggle play/pause |
| `→` or `N` | Next preset |
| `←` or `P` | Previous preset |
| `R` | Reset visualization |
| `Esc` or `Q` | Quit |

## Usage

### Basic usage

```bash
onedrop-gui
```

The application will:
1. Open a window (1280x720 by default)
2. Load presets from `../test-presets/` if available
3. Start rendering the first preset
4. Accept keyboard input for navigation

### With custom presets

Place your `.milk` preset files in the `test-presets` directory:

```
OneDrop/
├── test-presets/
│   ├── preset1.milk
│   ├── preset2.milk
│   └── preset3.milk
└── onedrop-gui/
```

Then run:
```bash
cargo run --release
```

### Audio input

Currently, the GUI generates demo audio (sine wave). To use real audio input:

1. Implement audio capture using `cpal` or similar
2. Pass captured audio to `engine.update()`

Example integration:
```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

// Capture audio from default input device
let host = cpal::default_host();
let device = host.default_input_device().unwrap();
let config = device.default_input_config().unwrap();

let stream = device.build_input_stream(
    &config.into(),
    move |data: &[f32], _: &_| {
        // Pass audio to engine
        engine.update(data, delta_time);
    },
    |err| eprintln!("Error: {}", err),
    None,
)?;

stream.play()?;
```

## Architecture

```
┌─────────────────────────────────────────┐
│         Winit Window                    │
│  - Event loop                           │
│  - Keyboard input                       │
│  - Window management                    │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         WGPU Surface                    │
│  - GPU context                          │
│  - Surface configuration                │
│  - Render target                        │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         MilkEngine                      │
│  - Preset loading                       │
│  - Audio analysis                       │
│  - Equation evaluation                  │
│  - GPU rendering                        │
└─────────────────────────────────────────┘
```

## Configuration

The application uses default configuration:
- **Resolution**: 1280x720
- **FPS**: 60 (VSync)
- **Audio sample rate**: 44100 Hz

To customize, modify `EngineConfig` in `main.rs`:

```rust
let engine_config = EngineConfig {
    render_config: RenderConfig {
        width: 1920,
        height: 1080,
        target_fps: 60,
        vsync: true,
        ..Default::default()
    },
    sample_rate: 48000.0,
    enable_per_frame: true,
    enable_per_pixel: false,
};
```

## Performance

Expected performance on modern hardware:
- **1080p**: 60+ FPS on GTX 1050 or equivalent
- **1440p**: 60+ FPS on GTX 1060 or equivalent
- **4K**: 60+ FPS on RTX 2060 or equivalent

Tips for better performance:
- Disable per-pixel equations (expensive)
- Use lower resolution
- Close other GPU-intensive applications

## Development

Build:
```bash
cargo build
```

Run:
```bash
cargo run
```

Run with logging:
```bash
RUST_LOG=debug cargo run
```

## Troubleshooting

### Window doesn't open
- Check GPU drivers are up to date
- Try running with `RUST_LOG=debug` to see errors

### Low FPS
- Check GPU usage in task manager
- Disable per-pixel equations
- Lower resolution

### Presets not loading
- Ensure presets are in `../test-presets/` directory
- Check preset file format is valid
- Run `onedrop-cli validate preset.milk` to check

## Future features

- [ ] Audio input from microphone/system
- [ ] Preset browser UI
- [ ] Full-screen mode
- [ ] Settings panel
- [ ] Preset transitions
- [ ] Beat detection visualization
- [ ] Recording to video

## License

MIT

## Part of OneDrop

This is the graphical user interface for the OneDrop project, a pure-Rust reimplementation of the Milkdrop music visualizer.
