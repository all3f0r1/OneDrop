//! # onedrop-renderer
//!
//! GPU rendering pipeline for Milkdrop visualizations using wgpu.
//!
//! This crate provides a complete rendering engine for Milkdrop presets,
//! including waveform rendering, motion effects, and shader-based transformations.

pub mod blend_renderer;
pub mod config;
pub mod error;
pub mod gpu_context;
pub mod renderer;
pub mod waveform;

pub use blend_renderer::BlendRenderer;
pub use config::{AudioLevels, MotionParams, RenderConfig, RenderState, WaveParams};
pub use error::{RenderError, Result};
pub use renderer::MilkRenderer;
pub use waveform::{WaveformMode, WaveformRenderer, WavePoint};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_renderer() {
        let config = RenderConfig::default();
        let renderer = pollster::block_on(MilkRenderer::new(config));
        assert!(renderer.is_ok());
    }
}
