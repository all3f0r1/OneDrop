# OneDrop Integration Guide for OneAmp

Complete guide for integrating OneDrop visualization engine into OneAmp audio player.

## Overview

OneDrop is designed as a modular, standalone visualizer that can be easily integrated into OneAmp. This guide covers the integration process step-by-step.

## Architecture

```
OneAmp
├── oneamp-core (audio playback)
├── oneamp-desktop (GUI)
└── onedrop (visualization) ← NEW
    ├── onedrop-parser
    ├── onedrop-eval
    ├── onedrop-renderer
    └── onedrop-engine
```

## Integration Steps

### Step 1: Add OneDrop as Dependency

**File**: `oneamp-desktop/Cargo.toml`

```toml
[dependencies]
# Existing dependencies
oneamp-core = { path = "../oneamp-core" }

# Add OneDrop
onedrop-engine = { git = "https://github.com/all3f0r1/OneDrop", version = "0.4" }
onedrop-parser = { git = "https://github.com/all3f0r1/OneDrop", version = "0.4" }
```

### Step 2: Create Visualizer Module

**File**: `oneamp-desktop/src/visualizer/onedrop.rs`

```rust
use onedrop_engine::{MilkEngine, AudioSamples};
use onedrop_parser::parse_preset;
use std::path::Path;

pub struct OneDropVisualizer {
    engine: MilkEngine,
    current_preset: Option<String>,
}

impl OneDropVisualizer {
    pub fn new(width: u32, height: u32) -> anyhow::Result<Self> {
        let engine = pollster::block_on(MilkEngine::new(width, height))?;
        
        Ok(Self {
            engine,
            current_preset: None,
        })
    }
    
    pub fn load_preset(&mut self, path: &Path) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let preset = parse_preset(&content)?;
        self.engine.load_preset(preset)?;
        self.current_preset = Some(path.display().to_string());
        Ok(())
    }
    
    pub fn update(&mut self, audio_samples: &[f32]) -> anyhow::Result<()> {
        // Convert OneAmp audio samples to OneDrop format
        let samples = AudioSamples::from_interleaved(audio_samples);
        self.engine.update(&samples)?;
        Ok(())
    }
    
    pub fn render(&mut self) -> anyhow::Result<&wgpu::Texture> {
        self.engine.render()
    }
    
    pub fn resize(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        self.engine.resize(width, height)
    }
}
```

### Step 3: Integrate into OneAmp GUI

**File**: `oneamp-desktop/src/main_window.rs`

```rust
use crate::visualizer::onedrop::OneDropVisualizer;

pub struct MainWindow {
    // Existing fields
    audio_player: AudioPlayer,
    
    // Add visualizer
    visualizer: Option<OneDropVisualizer>,
    visualizer_enabled: bool,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            audio_player: AudioPlayer::new(),
            visualizer: None,
            visualizer_enabled: false,
        }
    }
    
    pub fn enable_visualizer(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        self.visualizer = Some(OneDropVisualizer::new(width, height)?);
        self.visualizer_enabled = true;
        Ok(())
    }
    
    pub fn update(&mut self) -> anyhow::Result<()> {
        // Get audio samples from player
        let samples = self.audio_player.get_samples();
        
        // Update visualizer if enabled
        if self.visualizer_enabled {
            if let Some(viz) = &mut self.visualizer {
                viz.update(&samples)?;
            }
        }
        
        Ok(())
    }
    
    pub fn render_visualizer(&mut self) -> anyhow::Result<Option<&wgpu::Texture>> {
        if !self.visualizer_enabled {
            return Ok(None);
        }
        
        if let Some(viz) = &mut self.visualizer {
            Ok(Some(viz.render()?))
        } else {
            Ok(None)
        }
    }
}
```

### Step 4: Add Visualizer UI Panel

**File**: `oneamp-desktop/src/ui/visualizer_panel.rs`

```rust
use egui::{Ui, Vec2};

pub struct VisualizerPanel {
    enabled: bool,
    preset_path: Option<String>,
    fullscreen: bool,
}

impl VisualizerPanel {
    pub fn new() -> Self {
        Self {
            enabled: false,
            preset_path: None,
            fullscreen: false,
        }
    }
    
    pub fn show(&mut self, ui: &mut Ui) {
        ui.heading("Visualizer");
        
        ui.checkbox(&mut self.enabled, "Enable Visualizer");
        
        if self.enabled {
            ui.horizontal(|ui| {
                ui.label("Preset:");
                if ui.button("Load...").clicked() {
                    // Open file dialog
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Milkdrop Preset", &["milk", "od2"])
                        .pick_file()
                    {
                        self.preset_path = Some(path.display().to_string());
                    }
                }
                
                if let Some(path) = &self.preset_path {
                    ui.label(path);
                }
            });
            
            ui.checkbox(&mut self.fullscreen, "Fullscreen");
            
            // Visualizer preview area
            ui.allocate_space(Vec2::new(ui.available_width(), 300.0));
        }
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn preset_path(&self) -> Option<&str> {
        self.preset_path.as_deref()
    }
}
```

### Step 5: Connect Audio Pipeline

**File**: `oneamp-desktop/src/audio_bridge.rs`

```rust
use oneamp_core::AudioCaptureBuffer;

pub struct AudioBridge {
    capture_buffer: AudioCaptureBuffer,
}

impl AudioBridge {
    pub fn new() -> Self {
        Self {
            capture_buffer: AudioCaptureBuffer::new(2048),
        }
    }
    
    pub fn capture_samples(&mut self, samples: &[f32]) {
        self.capture_buffer.write(samples);
    }
    
    pub fn get_samples_for_visualizer(&self) -> Vec<f32> {
        // Get last 512 samples for visualizer
        self.capture_buffer.read_latest(512)
    }
}
```

## Audio Format Conversion

OneDrop expects audio in the following format:

```rust
pub struct AudioSamples {
    pub left: Vec<f32>,   // Left channel
    pub right: Vec<f32>,  // Right channel
    pub sample_rate: u32, // Usually 44100 or 48000
}

impl AudioSamples {
    pub fn from_interleaved(samples: &[f32]) -> Self {
        let mut left = Vec::with_capacity(samples.len() / 2);
        let mut right = Vec::with_capacity(samples.len() / 2);
        
        for chunk in samples.chunks_exact(2) {
            left.push(chunk[0]);
            right.push(chunk[1]);
        }
        
        Self {
            left,
            right,
            sample_rate: 44100,
        }
    }
}
```

## Preset Management

### Loading Presets

```rust
// Load single preset
visualizer.load_preset(Path::new("presets/cool_preset.milk"))?;

// Load double preset
visualizer.load_double_preset(Path::new("presets/blend.od2"))?;
```

### Preset Directory Structure

```
oneamp/
└── presets/
    ├── default/
    │   ├── preset1.milk
    │   ├── preset2.milk
    │   └── ...
    ├── favorites/
    │   └── ...
    └── custom/
        └── ...
```

## Performance Optimization

### 1. Audio Buffer Size

```rust
// Smaller buffer = lower latency, higher CPU
const BUFFER_SIZE: usize = 512;

// Larger buffer = higher latency, lower CPU
const BUFFER_SIZE: usize = 2048;
```

### 2. Render Resolution

```rust
// Full HD (high quality)
visualizer.resize(1920, 1080)?;

// HD (balanced)
visualizer.resize(1280, 720)?;

// SD (performance)
visualizer.resize(854, 480)?;
```

### 3. Frame Rate Limiting

```rust
// Limit to 60 FPS
const FRAME_TIME: Duration = Duration::from_millis(16);

if last_frame.elapsed() >= FRAME_TIME {
    visualizer.render()?;
    last_frame = Instant::now();
}
```

## Feature Flags

Add optional features to `Cargo.toml`:

```toml
[features]
default = ["visualizer"]
visualizer = ["onedrop-engine", "onedrop-parser"]
double-presets = ["visualizer"]
```

## Testing Integration

**File**: `oneamp-desktop/tests/visualizer_test.rs`

```rust
#[test]
fn test_visualizer_integration() {
    let mut viz = OneDropVisualizer::new(800, 600).unwrap();
    
    // Load test preset
    viz.load_preset(Path::new("../OneDrop/test-presets/test.milk")).unwrap();
    
    // Generate test audio
    let samples: Vec<f32> = (0..1024)
        .map(|i| (i as f32 * 0.01).sin())
        .collect();
    
    // Update and render
    viz.update(&samples).unwrap();
    let texture = viz.render().unwrap();
    
    assert_eq!(texture.width(), 800);
    assert_eq!(texture.height(), 600);
}
```

## UI Integration Example

```rust
// In main event loop
loop {
    // Handle events
    for event in event_loop.poll() {
        match event {
            Event::AudioData(samples) => {
                audio_bridge.capture_samples(&samples);
            }
            Event::Render => {
                // Get samples for visualizer
                let viz_samples = audio_bridge.get_samples_for_visualizer();
                
                // Update visualizer
                if let Some(viz) = &mut visualizer {
                    viz.update(&viz_samples)?;
                    let texture = viz.render()?;
                    
                    // Display texture in UI
                    ui.image(texture);
                }
            }
            _ => {}
        }
    }
}
```

## Configuration

**File**: `oneamp-desktop/config/visualizer.toml`

```toml
[visualizer]
enabled = true
preset_dir = "presets/default"
auto_switch = true
switch_interval = 30  # seconds
fullscreen = false
resolution = [1280, 720]

[visualizer.effects]
motion_blur = true
bloom = false
chromatic_aberration = false

[visualizer.audio]
buffer_size = 512
fft_size = 2048
smoothing = 0.8
```

## Troubleshooting

### Issue: Visualizer not updating

**Solution**: Ensure audio samples are being captured and passed to visualizer:

```rust
// Debug audio flow
println!("Samples: {} | Max: {:.2}", 
    samples.len(), 
    samples.iter().map(|s| s.abs()).fold(0.0, f32::max)
);
```

### Issue: Low frame rate

**Solution**: Reduce resolution or enable frame skipping:

```rust
// Skip every other frame
if frame_count % 2 == 0 {
    visualizer.render()?;
}
```

### Issue: High CPU usage

**Solution**: Use lower quality settings:

```rust
// Reduce FFT size
engine.set_fft_size(1024)?;  // Instead of 2048

// Reduce render resolution
visualizer.resize(640, 360)?;
```

## Example: Complete Integration

**File**: `oneamp-desktop/examples/visualizer_demo.rs`

```rust
use oneamp_desktop::visualizer::OneDropVisualizer;
use std::time::{Duration, Instant};

fn main() -> anyhow::Result<()> {
    // Initialize visualizer
    let mut viz = OneDropVisualizer::new(1280, 720)?;
    
    // Load preset
    viz.load_preset(Path::new("presets/default/cool.milk"))?;
    
    // Simulate audio playback
    let mut time = 0.0;
    let sample_rate = 44100.0;
    let buffer_size = 512;
    
    loop {
        // Generate test audio (sine wave)
        let samples: Vec<f32> = (0..buffer_size)
            .map(|i| {
                let t = time + (i as f32 / sample_rate);
                (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * 0.5
            })
            .collect();
        
        time += buffer_size as f32 / sample_rate;
        
        // Update visualizer
        viz.update(&samples)?;
        
        // Render
        let texture = viz.render()?;
        
        // Display texture (pseudo-code)
        // window.display(texture);
        
        // Limit to 60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }
}
```

## Next Steps

1. **Test Integration**: Run tests to ensure everything works
2. **Add UI Controls**: Preset selection, effects, etc.
3. **Optimize Performance**: Profile and optimize hot paths
4. **Add Features**: Fullscreen mode, preset randomization
5. **Documentation**: Update OneAmp docs with visualizer guide

## Resources

- [OneDrop Repository](https://github.com/all3f0r1/OneDrop)
- [OneDrop Documentation](https://github.com/all3f0r1/OneDrop/tree/main/docs)
- [Blend Patterns Guide](./BLEND_PATTERNS.md)
- [OneAmp Repository](https://github.com/all3f0r1/oneamp)

## License

MIT - See LICENSE file for details

---

*OneDrop v0.4.0 - Ready for OneAmp Integration*
