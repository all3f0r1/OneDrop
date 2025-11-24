//! # milk-engine
//!
//! Complete Milkdrop visualization engine assembling parser, evaluator, and renderer.
//!
//! This crate provides a high-level API for running Milkdrop visualizations,
//! handling preset loading, audio analysis, equation evaluation, and rendering.

pub mod audio;
pub mod engine;
pub mod error;
pub mod preset_manager;

pub use audio::AudioAnalyzer;
pub use engine::{EngineConfig, MilkEngine};
pub use error::{EngineError, Result};
pub use preset_manager::{PresetManager, TransitionState};

// Re-export commonly used types
pub use milk_parser::MilkPreset;
pub use milk_renderer::{AudioLevels, MotionParams, RenderConfig, RenderState, WaveParams};

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
