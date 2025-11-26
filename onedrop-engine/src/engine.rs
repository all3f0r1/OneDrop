//! Main Milkdrop engine implementation.

use crate::audio::AudioAnalyzer;
use crate::beat_detection::{BeatDetector, BeatDetectionMode, PresetChange};
use crate::error::{EngineError, Result};
use onedrop_eval::MilkEvaluator;
use onedrop_parser::{parse_preset, MilkPreset};
use onedrop_renderer::{MilkRenderer, MotionParams, RenderConfig, RenderState, WaveParams};
use wgpu;
use std::fs;
use std::path::Path;

/// Main Milkdrop visualization engine.
pub struct MilkEngine {
    /// Renderer
    renderer: MilkRenderer,
    
    /// Expression evaluator
    evaluator: MilkEvaluator,
    
    /// Audio analyzer
    audio_analyzer: AudioAnalyzer,
    
    /// Beat detector for automatic preset changes
    beat_detector: BeatDetector,
    
    /// Current preset
    current_preset: Option<MilkPreset>,
    
    /// Current render state
    state: RenderState,
    
    /// Engine configuration
    config: EngineConfig,
}

/// Engine configuration.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Render configuration
    pub render_config: RenderConfig,
    
    /// Audio sample rate
    pub sample_rate: f32,
    
    /// Enable per-frame equations
    pub enable_per_frame: bool,
    
    /// Enable per-pixel equations
    pub enable_per_pixel: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            render_config: RenderConfig::default(),
            sample_rate: 44100.0,
            enable_per_frame: true,
            enable_per_pixel: false, // Disabled by default (expensive)
        }
    }
}

impl MilkEngine {
    /// Create a new engine.
    pub async fn new(config: EngineConfig) -> Result<Self> {
        let renderer = MilkRenderer::new(config.render_config.clone()).await?;
        let evaluator = MilkEvaluator::new();
        let audio_analyzer = AudioAnalyzer::new(config.sample_rate);
        
        Ok(Self {
            renderer,
            evaluator,
            audio_analyzer,
            beat_detector: BeatDetector::new(),
            current_preset: None,
            state: RenderState::default(),
            config,
        })
    }
    
    /// Load a preset from file.
    pub fn load_preset<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path_ref = path.as_ref();
        log::info!("Loading preset: {}", path_ref.display());
        
        // Read file
        let content = fs::read_to_string(path_ref).map_err(|e| {
            log::error!("Failed to read preset file {}: {}", path_ref.display(), e);
            EngineError::PresetLoadFailed(format!("Cannot read file: {}", e))
        })?;
        
        // Parse preset
        let preset = parse_preset(&content).map_err(|e| {
            log::error!("Failed to parse preset {}: {}", path_ref.display(), e);
            e
        })?;
        
        // Validate preset
        if preset.per_frame_equations.is_empty() && preset.per_pixel_equations.is_empty() {
            log::warn!("Preset {} has no equations, using default parameters", path_ref.display());
        }
        
        self.load_preset_from_data(preset)
    }
    
    /// Load the default preset.
    /// This is useful as a fallback when no preset is available or loading fails.
    pub fn load_default_preset(&mut self) -> Result<()> {
        log::info!("Loading default preset");
        let preset = crate::default_preset::default_preset();
        self.load_preset_from_data(preset)
    }
    
    /// Load a preset from parsed data.
    pub fn load_preset_from_data(&mut self, preset: MilkPreset) -> Result<()> {
        log::info!("Loading preset version {}", preset.version);
        
        // Initialize evaluator context with preset parameters
        self.init_evaluator_from_preset(&preset);
        
        self.current_preset = Some(preset);
        
        Ok(())
    }
    
    /// Initialize evaluator context from preset parameters.
    fn init_evaluator_from_preset(&mut self, preset: &MilkPreset) {
        let ctx = self.evaluator.context_mut();
        let params = &preset.parameters;
        
        // Set motion parameters
        ctx.set_var("zoom", params.zoom as f64);
        ctx.set_var("zoomexp", params.zoomexp() as f64);
        ctx.set_var("rot", params.rot as f64);
        ctx.set_var("warp", params.warp as f64);
        ctx.set_var("cx", params.cx as f64);
        ctx.set_var("cy", params.cy as f64);
        ctx.set_var("dx", params.dx as f64);
        ctx.set_var("dy", params.dy as f64);
        ctx.set_var("sx", params.sx as f64);
        ctx.set_var("sy", params.sy as f64);
        
        // Set wave parameters
        ctx.set_var("wave_r", params.wave_r as f64);
        ctx.set_var("wave_g", params.wave_g as f64);
        ctx.set_var("wave_b", params.wave_b as f64);
        ctx.set_var("wave_a", params.wave_a() as f64);
        ctx.set_var("wave_x", params.wave_x as f64);
        ctx.set_var("wave_y", params.wave_y as f64);
        ctx.set_var("wave_mode", params.wave_mode() as f64);
        
        // Set other parameters
        ctx.set_var("decay", params.decay() as f64);
        ctx.set_var("gamma", params.gamma() as f64);
        ctx.set_var("echo_zoom", params.echo_zoom() as f64);
        ctx.set_var("echo_alpha", params.echo_alpha() as f64);
        ctx.set_var("darken_center", if params.darken_center() { 1.0 } else { 0.0 });
        ctx.set_var("wrap", if params.wrap() { 1.0 } else { 0.0 });
        ctx.set_var("invert", if params.invert() { 1.0 } else { 0.0 });
        ctx.set_var("brighten", if params.brighten() { 1.0 } else { 0.0 });
        ctx.set_var("darken", if params.darken() { 1.0 } else { 0.0 });
        ctx.set_var("solarize", if params.solarize() { 1.0 } else { 0.0 });
    }
    
    /// Update engine with audio data and render a frame.
    /// Returns Some(PresetChange) if beat detection triggered a preset change.
    pub fn update(&mut self, audio_samples: &[f32], delta_time: f32) -> Result<Option<PresetChange>> {
        // Analyze audio
        let audio_levels = self.audio_analyzer.analyze(audio_samples);
        
        // Update time
        self.state.time += delta_time;
        
        // Update audio in state
        self.state.audio = audio_levels;
        
        // Check beat detection for automatic preset change
        let preset_change = self.beat_detector.should_change_preset(
            audio_levels.bass,
            audio_levels.mid,
            audio_levels.treb,
        );
        
        // Update evaluator context
        let ctx = self.evaluator.context_mut();
        ctx.set_time(self.state.time as f64);
        ctx.set_frame(self.state.frame as f64);
        ctx.set_audio(
            audio_levels.bass as f64,
            audio_levels.mid as f64,
            audio_levels.treb as f64,
        );
        ctx.set("bass_att", audio_levels.bass_att as f64);
        ctx.set("mid_att", audio_levels.mid_att as f64);
        ctx.set("treb_att", audio_levels.treb_att as f64);
        
        // Execute per-frame equations if enabled and preset loaded
        if self.config.enable_per_frame {
            if let Some(preset) = &self.current_preset {
                let equations = preset.per_frame_equations.clone();
                
                // Try to evaluate equations, but don't fail the entire frame if one fails
                if let Err(e) = self.evaluator.eval_per_frame(&equations) {
                    log::warn!("Per-frame equation evaluation failed: {}. Continuing with previous state.", e);
                    // Continue rendering with previous state instead of failing
                }
            }
        }
        
        // Update render state from evaluator
        self.update_render_state_from_evaluator();
        
        // Update renderer state
        self.renderer.update_state(self.state.clone());
        
        // Render frame
        self.renderer.render()?;
        
        // Increment frame counter
        self.state.frame += 1;
        
        Ok(preset_change)
    }
    
    /// Execute per-frame equations.
    fn execute_per_frame_equations(&mut self, preset: &MilkPreset) -> Result<()> {
        for equation in &preset.per_frame_equations {
            self.evaluator.eval(equation)?;
        }
        Ok(())
    }
    
    /// Update render state from evaluator context.
    fn update_render_state_from_evaluator(&mut self) {
        let ctx = self.evaluator.context();
        
        // Update motion parameters
        self.state.motion = MotionParams {
            zoom: ctx.get_var("zoom").unwrap_or(1.0) as f32,
            rot: ctx.get_var("rot").unwrap_or(0.0) as f32,
            cx: ctx.get_var("cx").unwrap_or(0.5) as f32,
            cy: ctx.get_var("cy").unwrap_or(0.5) as f32,
            dx: ctx.get_var("dx").unwrap_or(0.0) as f32,
            dy: ctx.get_var("dy").unwrap_or(0.0) as f32,
            warp: ctx.get_var("warp").unwrap_or(0.0) as f32,
            sx: ctx.get_var("sx").unwrap_or(1.0) as f32,
            sy: ctx.get_var("sy").unwrap_or(1.0) as f32,
        };
        
        // Update wave parameters
        self.state.wave = WaveParams {
            r: ctx.get_var("wave_r").unwrap_or(1.0) as f32,
            g: ctx.get_var("wave_g").unwrap_or(1.0) as f32,
            b: ctx.get_var("wave_b").unwrap_or(1.0) as f32,
            a: ctx.get_var("wave_a").unwrap_or(1.0) as f32,
            x: ctx.get_var("wave_x").unwrap_or(0.5) as f32,
            y: ctx.get_var("wave_y").unwrap_or(0.5) as f32,
            mode: ctx.get_var("wave_mode").unwrap_or(0.0) as i32,
        };
    }
    
    /// Get the current render texture.
    pub fn render_texture(&self) -> &wgpu::Texture {
        self.renderer.render_texture()
    }
    
    /// Get current state.
    pub fn state(&self) -> &RenderState {
        &self.state
    }
    
    /// Get current preset.
    pub fn current_preset(&self) -> Option<&MilkPreset> {
        self.current_preset.as_ref()
    }
    
    /// Get the beat detector.
    pub fn beat_detector(&self) -> &BeatDetector {
        &self.beat_detector
    }
    
    /// Get the beat detector mutably.
    pub fn beat_detector_mut(&mut self) -> &mut BeatDetector {
        &mut self.beat_detector
    }
    
    /// Set beat detection mode.
    pub fn set_beat_detection_mode(&mut self, mode: BeatDetectionMode) {
        self.beat_detector.set_mode(mode);
    }
    
    /// Toggle beat detection to next mode.
    pub fn next_beat_detection_mode(&mut self) {
        self.beat_detector.next_mode();
    }
    
    /// Enable beat detection.
    pub fn enable_beat_detection(&mut self) {
        self.beat_detector.enable();
    }
    
    /// Disable beat detection.
    pub fn disable_beat_detection(&mut self) {
        self.beat_detector.disable();
    }
    
    /// Reset engine state.
    pub fn reset(&mut self) {
        self.state = RenderState::default();
        self.evaluator.reset();
        self.audio_analyzer.reset();
    }
    
    /// Resize the renderer.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        env_logger::try_init().ok();
        
        let config = EngineConfig::default();
        let engine = pollster::block_on(MilkEngine::new(config));
        
        assert!(engine.is_ok());
    }

    #[test]
    fn test_update_without_preset() {
        env_logger::try_init().ok();
        
        let config = EngineConfig::default();
        let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
        
        let audio_samples = vec![0.0; 1024];
        let result = engine.update(&audio_samples, 0.016);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_updates() {
        env_logger::try_init().ok();
        
        let config = EngineConfig::default();
        let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
        
        // Simulate 60 frames
        for i in 0..60 {
            let audio_samples: Vec<f32> = (0..1024)
                .map(|j| ((i + j) as f32 * 0.1).sin() * 0.5)
                .collect();
            
            let result = engine.update(&audio_samples, 0.016);
            assert!(result.is_ok(), "Failed at frame {}", i);
        }
        
        assert_eq!(engine.state().frame, 60);
    }
}
