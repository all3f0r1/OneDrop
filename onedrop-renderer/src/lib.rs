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
pub mod per_pixel_pipeline;
pub mod per_vertex_pipeline;

pub use blend_renderer::BlendRenderer;
pub use config::{AudioLevels, MotionParams, RenderConfig, RenderState, WaveParams};
pub use error::{RenderError, Result};
pub use gpu_context::GpuContext;
pub use per_vertex_pipeline::{PerVertexPipeline, VertexVarsUniform};
pub use renderer::MilkRenderer;
pub use waveform::{WaveformMode, WaveformRenderer, WavePoint};
pub use per_pixel_pipeline::{PerPixelPipeline, PixelVarsUniform};

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
