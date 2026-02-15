# onedrop-engine

Complete Milkdrop visualization engine assembling parser, evaluator, and renderer.

## Overview

This crate provides a high-level API for running Milkdrop visualizations. It integrates the parser, evaluator, and renderer into a cohesive engine that handles preset loading, audio analysis, equation evaluation, and rendering.

## Features

- **Complete integration** - Assembles all OneDrop components
- **Preset management** - Load and switch between presets
- **Audio analysis** - Extract bass, mid, and treble levels
- **Per-frame equations** - Execute preset equations in real-time
- **Transition support** - Smooth transitions between presets
- **High-level API** - Simple interface for complex visualizations

## Usage

### Basic usage

```rust
use milk_engine::{EngineConfig, MilkEngine};

// Create engine
let config = EngineConfig::default();
let mut engine = MilkEngine::new(config).await?;

// Main loop
loop {
    // Get audio samples from your audio source
    let audio_samples = get_audio_samples(); // Your audio input
    
    // Update engine (60 FPS = 0.016s per frame)
    engine.update(&audio_samples, 0.016)?;
    
    // Get render texture for display
    let texture = engine.render_texture();
}
```

### Loading presets

```rust
use milk_engine::MilkEngine;

let mut engine = MilkEngine::new(config).await?;

// Load a preset from file
engine.load_preset("path/to/preset.milk")?;

// Now the preset's equations will be evaluated each frame
engine.update(&audio_samples, 0.016)?;
```

### Managing presets

```rust
use milk_engine::PresetManager;

let mut manager = PresetManager::new();

// Add presets to queue
manager.add_preset("preset1.milk");
manager.add_preset("preset2.milk");
manager.add_preset("preset3.milk");

// Navigate
let next = manager.next_preset();
let prev = manager.prev_preset();
let current = manager.current_preset();

// Shuffle
manager.shuffle();

// Transitions
manager.start_transition(2.0); // 2 second transition
while manager.is_transitioning() {
    manager.update_transition(delta_time);
    let progress = manager.transition_progress(); // 0.0 to 1.0
}
```

### Audio-reactive visualization

```rust
use milk_engine::{EngineConfig, MilkEngine};

let mut engine = MilkEngine::new(config).await?;
engine.load_preset("audio_reactive.milk")?;

loop {
    // Get real-time audio
    let audio_samples = capture_audio();
    
    // Engine analyzes audio and updates visualization
    engine.update(&audio_samples, 0.016)?;
    
    // Access audio levels
    let state = engine.state();
    println!("Bass: {:.2}, Mid: {:.2}, Treb: {:.2}",
             state.audio.bass,
             state.audio.mid,
             state.audio.treb);
}
```

### Configuration

```rust
use milk_engine::{EngineConfig, RenderConfig};

let config = EngineConfig {
    render_config: RenderConfig {
        width: 1920,
        height: 1080,
        target_fps: 60,
        ..Default::default()
    },
    sample_rate: 44100.0,
    enable_per_frame: true,
    enable_per_pixel: false, // Expensive, disable for performance
};

let engine = MilkEngine::new(config).await?;
```

## Architecture

The engine integrates three main components:

```
┌─────────────────────────────────────────┐
│         Audio Input (PCM samples)       │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         AudioAnalyzer                   │
│  - Extract bass, mid, treble            │
│  - Calculate attenuated values          │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         MilkEvaluator                   │
│  - Execute per-frame equations          │
│  - Update variables (zoom, rot, etc.)   │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         MilkRenderer                    │
│  - Apply motion effects                 │
│  - Render waveforms                     │
│  - Output frame texture                 │
└─────────────────────────────────────────┘
```

## Components

### MilkEngine

The main engine class that orchestrates all components.

**Key methods**:
- `new(config)` - Create engine
- `load_preset(path)` - Load preset from file
- `update(audio, delta_time)` - Update and render frame
- `state()` - Get current state
- `reset()` - Reset to initial state
- `resize(width, height)` - Resize output

### AudioAnalyzer

Analyzes audio samples to extract frequency bands.

**Features**:
- Bass, mid, treble extraction
- Attenuated values for smooth transitions
- Configurable attenuation factor

### PresetManager

Manages preset queue and transitions.

**Features**:
- Preset queue management
- Navigation (next, previous)
- Shuffle
- Smooth transitions

## State

The engine maintains a `RenderState` containing:

- **Time** - Current time in seconds
- **Frame** - Current frame number
- **Audio** - Bass, mid, treble levels (current and attenuated)
- **Motion** - Zoom, rotation, translation, warp, stretch
- **Wave** - Waveform colors and position

## Performance

Optimized for real-time rendering:
- **60+ FPS** at 1080p on modern hardware
- **GPU-accelerated** rendering
- **Efficient audio analysis** (simple RMS, no FFT overhead)
- **Optional per-pixel equations** (disable for better performance)

## Examples

See the `examples/` directory:
- `basic.rs` - Minimal engine usage
- `audio_reactive.rs` - Audio-reactive visualization

Run examples:
```bash
cargo run --example basic
cargo run --example audio_reactive
```

## Testing

```bash
cargo test
cargo test -- --nocapture  # Show output
```

## Integration

This crate is designed to be integrated into audio players and visualization applications:

```rust
// In your audio player
impl AudioPlayer {
    fn on_audio_data(&mut self, samples: &[f32]) {
        // Pass audio to visualization engine
        self.vis_engine.update(samples, self.delta_time)?;
        
        // Display the rendered texture
        self.display_texture(self.vis_engine.render_texture());
    }
}
```

## License

MIT

## Part of OneDrop

This crate is part of the OneDrop project, a pure-Rust reimplementation of the Milkdrop music visualizer.
