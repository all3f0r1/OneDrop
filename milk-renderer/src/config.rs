//! Configuration for the renderer.

use serde::{Deserialize, Serialize};

/// Renderer configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    /// Output resolution width
    pub width: u32,
    
    /// Output resolution height
    pub height: u32,
    
    /// Texture format
    pub texture_format: TextureFormat,
    
    /// Enable multisampling
    pub msaa_samples: u32,
    
    /// Enable VSync
    pub vsync: bool,
    
    /// Target FPS (0 = unlimited)
    pub target_fps: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            texture_format: TextureFormat::Bgra8UnormSrgb,
            msaa_samples: 1,
            vsync: true,
            target_fps: 60,
        }
    }
}

/// Texture format options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureFormat {
    Bgra8UnormSrgb,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Rgba8Unorm,
}

impl TextureFormat {
    /// Convert to wgpu texture format.
    pub fn to_wgpu(&self) -> wgpu::TextureFormat {
        match self {
            TextureFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
            TextureFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            TextureFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
            TextureFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
        }
    }
}

/// Render state containing dynamic parameters.
#[derive(Debug, Clone)]
pub struct RenderState {
    /// Current time in seconds
    pub time: f32,
    
    /// Current frame number
    pub frame: u32,
    
    /// Audio levels (bass, mid, treble)
    pub audio: AudioLevels,
    
    /// Motion parameters
    pub motion: MotionParams,
    
    /// Wave parameters
    pub wave: WaveParams,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            time: 0.0,
            frame: 0,
            audio: AudioLevels::default(),
            motion: MotionParams::default(),
            wave: WaveParams::default(),
        }
    }
}

/// Audio levels.
#[derive(Debug, Clone, Copy)]
pub struct AudioLevels {
    pub bass: f32,
    pub mid: f32,
    pub treb: f32,
    pub bass_att: f32,
    pub mid_att: f32,
    pub treb_att: f32,
}

impl Default for AudioLevels {
    fn default() -> Self {
        Self {
            bass: 0.0,
            mid: 0.0,
            treb: 0.0,
            bass_att: 0.0,
            mid_att: 0.0,
            treb_att: 0.0,
        }
    }
}

/// Motion parameters.
#[derive(Debug, Clone, Copy)]
pub struct MotionParams {
    pub zoom: f32,
    pub rot: f32,
    pub cx: f32,
    pub cy: f32,
    pub dx: f32,
    pub dy: f32,
    pub warp: f32,
    pub sx: f32,
    pub sy: f32,
}

impl Default for MotionParams {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            rot: 0.0,
            cx: 0.5,
            cy: 0.5,
            dx: 0.0,
            dy: 0.0,
            warp: 0.0,
            sx: 1.0,
            sy: 1.0,
        }
    }
}

/// Wave parameters.
#[derive(Debug, Clone, Copy)]
pub struct WaveParams {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub x: f32,
    pub y: f32,
    pub mode: i32,
}

impl Default for WaveParams {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
            x: 0.5,
            y: 0.5,
            mode: 0,
        }
    }
}
