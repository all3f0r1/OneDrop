//! # onedrop-engine
//!
//! Complete Milkdrop visualization engine assembling parser, evaluator, and renderer.
//!
//! This crate provides a high-level API for running Milkdrop visualizations,
//! handling preset loading, audio analysis, equation evaluation, and rendering.

pub mod audio;
pub mod engine;
pub mod error;
pub mod fft;
pub mod preset_manager;
pub mod transition;

pub use audio::AudioAnalyzer;
pub use engine::{EngineConfig, MilkEngine};
pub use error::{EngineError, Result};
pub use fft::FFTAnalyzer;
pub use preset_manager::{PresetManager, TransitionState};
pub use transition::{Transition, TransitionManager, TransitionMode};

// Re-export commonly used types
pub use onedrop_parser::MilkPreset;
pub use onedrop_renderer::{AudioLevels, MotionParams, RenderConfig, RenderState, WaveParams};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_engine() {
        let config = EngineConfig::default();
        let engine = pollster::block_on(MilkEngine::new(config));
        assert!(engine.is_ok());
    }
}
