# onedrop-renderer

GPU rendering pipeline for Milkdrop visualizations using wgpu.

## Overview

This crate provides a complete GPU-accelerated rendering engine for Milkdrop presets. It uses modern graphics APIs (Vulkan, Metal, DX12, OpenGL) through wgpu for cross-platform compatibility and high performance.

## Features

- **Modern GPU acceleration** - Uses wgpu for Vulkan/Metal/DX12/OpenGL
- **Composite effects** - Zoom, rotation, translation, decay
- **Waveform rendering** - Audio-reactive waveforms
- **Shader support** - Custom WGSL shaders
- **Feedback effects** - Previous frame sampling for trails
- **High performance** - Optimized for 60+ FPS at HD resolutions

## Usage

### Basic rendering

```rust
use milk_renderer::{MilkRenderer, RenderConfig, RenderState};

// Create renderer
let config = RenderConfig {
    width: 1280,
    height: 720,
    ..Default::default()
};

let mut renderer = MilkRenderer::new(config).await?;

// Render loop
loop {
    // Update state
    let mut state = RenderState::default();
    state.time = get_time();
    state.motion.zoom = 1.0 + 0.1 * state.time.sin();
    
    renderer.update_state(state);
    
    // Render frame
    renderer.render()?;
}
```

### With audio data

```rust
use milk_renderer::{RenderState, AudioLevels};

let mut state = RenderState::default();

// Set audio levels from FFT analysis
state.audio = AudioLevels {
    bass: 0.8,
    mid: 0.5,
    treb: 0.3,
    bass_att: 0.6,
    mid_att: 0.4,
    treb_att: 0.2,
};

// Audio-reactive zoom
state.motion.zoom = 1.0 + 0.2 * state.audio.bass;

renderer.update_state(state);
renderer.render()?;
```

### Motion effects

```rust
use milk_renderer::MotionParams;

let mut motion = MotionParams::default();

// Zoom in/out
motion.zoom = 1.1;

// Rotate
motion.rot = 0.1; // radians

// Translate
motion.dx = 0.05;
motion.dy = -0.05;

// Stretch
motion.sx = 1.2;
motion.sy = 0.8;

state.motion = motion;
```

## Architecture

### Rendering Pipeline

1. **Composite Pass** - Apply motion effects (zoom, rotation, etc.) to previous frame
2. **Waveform Pass** - Render audio waveforms
3. **Effects Pass** - Apply additional effects (blur, glow, etc.)
4. **Output** - Final frame ready for display

### Shaders

- `composite.wgsl` - Motion effects and frame blending
- `waveform.wgsl` - Waveform rendering
- Custom shaders can be loaded from preset files

### Textures

- **Render Texture** - Current frame being rendered
- **Previous Texture** - Previous frame for feedback effects
- Double-buffering for smooth transitions

## Performance

Optimized for real-time rendering:
- GPU-accelerated for all operations
- Efficient texture management
- Minimal CPU overhead
- 60+ FPS at 1080p on modern GPUs

## Configuration

```rust
use milk_renderer::{RenderConfig, TextureFormat};

let config = RenderConfig {
    width: 1920,
    height: 1080,
    texture_format: TextureFormat::Bgra8UnormSrgb,
    msaa_samples: 4, // Anti-aliasing
    vsync: true,
    target_fps: 60,
};
```

## Testing

```bash
cargo test
cargo test -- --nocapture  # Show output
```

## Examples

See the `examples/` directory for complete usage examples:
- `basic.rs` - Simple rendering setup
- `audio_reactive.rs` - Audio-reactive visualization
- `custom_shader.rs` - Using custom shaders

## Requirements

- GPU with Vulkan, Metal, DX12, or OpenGL support
- Rust 1.70+

## License

MIT

## Part of OneDrop

This crate is part of the OneDrop project, a pure-Rust reimplementation of the Milkdrop music visualizer.
