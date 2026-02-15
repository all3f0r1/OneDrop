//! End-to-End tests for the complete OneDrop pipeline.
//!
//! Tests the full flow: Preset Loading → Audio Analysis → Evaluation → Rendering

use onedrop_engine::{EngineConfig, MilkEngine};
use onedrop_parser::parse_preset;
use std::fs;
use std::time::Duration;
use std::path::PathBuf;

/// Helper to find preset path (works from workspace root or onedrop-engine dir)
fn preset_path(name: &str) -> PathBuf {
    let candidates = vec![
        PathBuf::from(format!("test-presets/{}", name)),
        PathBuf::from(format!("../test-presets/{}", name)),
    ];
    
    for path in candidates {
        if path.exists() {
            return path;
        }
    }
    
    panic!("Preset not found: {}", name);
}

#[test]
fn test_e2e_preset_loading() {
    // Test that we can load a preset and create an engine
    let preset_file = preset_path("$$$ Royal - Mashup (151).milk");

    // Parse preset
    let content = fs::read_to_string(&preset_file).expect("Failed to read preset file");
    let preset = parse_preset(&content);
    assert!(preset.is_ok(), "Failed to parse preset: {:?}", preset.err());

    let preset = preset.unwrap();
    // Preset version should be > 0 (valid)
    assert!(preset.version > 0, "Preset version should be > 0");

    // Create engine with default config
    let config = EngineConfig::default();

    // Create engine
    let engine = pollster::block_on(MilkEngine::new(config));
    assert!(engine.is_ok(), "Failed to create engine: {:?}", engine.err());
}

#[test]
fn test_e2e_audio_processing() {
    // Test audio sample processing
    let config = EngineConfig::default();
    
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    // Simulate audio samples (sine wave @ 440 Hz)
    let sample_rate = 44100.0;
    let duration = 1.0 / 60.0; // One frame @ 60 FPS
    let samples_per_frame = (sample_rate * duration) as usize;
    
    let mut audio_samples = Vec::with_capacity(samples_per_frame);
    for i in 0..samples_per_frame {
        let t = i as f32 / sample_rate;
        let sample = (2.0 * std::f32::consts::PI * 440.0 * t).sin();
        audio_samples.push(sample);
    }
    
    // Update engine with audio (also renders)
    let delta_time = duration as f32;
    let result = engine.update(&audio_samples, delta_time);
    
    // Should succeed
    assert!(result.is_ok(), "Failed to update engine: {:?}", result.err());
}

#[test]
fn test_e2e_render_frame() {
    // Test rendering a single frame
    let config = EngineConfig::default();
    
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    // Simulate audio (silence)
    let audio_samples = vec![0.0; 735]; // ~44100 / 60
    let delta_time = 1.0 / 60.0;
    
    // Update (includes render)
    let result = engine.update(&audio_samples, delta_time);
    assert!(result.is_ok(), "Failed to update/render: {:?}", result.err());
}

#[test]
fn test_e2e_multiple_frames() {
    // Test rendering multiple frames in sequence
    let config = EngineConfig::default();
    
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    let audio_samples = vec![0.0; 735];
    let delta_time = 1.0 / 60.0;
    
    // Render 60 frames (1 second)
    for frame in 0..60 {
        let update_result = engine.update(&audio_samples, delta_time);
        assert!(update_result.is_ok(), "Frame {} update failed", frame);
    }
    
    // Verify state progressed
    let state = engine.state();
    assert_eq!(state.frame, 60, "Frame counter should be 60");
    assert!((state.time - 1.0).abs() < 0.01, "Time should be ~1.0 second");
}

#[test]
fn test_e2e_preset_switching() {
    // Test switching between presets
    let config = EngineConfig::default();

    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();

    // Load first preset
    let preset1 = preset_path("$$$ Royal - Mashup (151).milk");
    let result = engine.load_preset(preset1);
    assert!(result.is_ok(), "Failed to load preset 1");

    // Render a few frames
    let audio_samples = vec![0.0; 735];
    let delta_time = 1.0 / 60.0;

    for _ in 0..10 {
        engine.update(&audio_samples, delta_time).unwrap();
    }

    // Switch to second preset
    let preset2 = preset_path("$$$ Royal - Mashup (246).milk");
    let result = engine.load_preset(preset2);
    assert!(result.is_ok(), "Failed to load preset 2");

    // Render more frames
    for _ in 0..10 {
        engine.update(&audio_samples, delta_time).unwrap();
    }

    // Should have rendered 20 frames total
    assert_eq!(engine.state().frame, 20);
}

#[test]
fn test_e2e_beat_detection() {
    // Test beat detection integration
    let config = EngineConfig::default();
    
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    // Enable beat detection
    engine.enable_beat_detection();
    
    // Simulate bass-heavy audio
    let sample_rate = 44100.0;
    let duration = 1.0 / 60.0;
    let samples_per_frame = (sample_rate * duration) as usize;
    
    let mut audio_samples = Vec::with_capacity(samples_per_frame);
    for i in 0..samples_per_frame {
        let t = i as f32 / sample_rate;
        // 60 Hz bass frequency
        let sample = (2.0 * std::f32::consts::PI * 60.0 * t).sin() * 2.0; // Loud bass
        audio_samples.push(sample);
    }
    
    // Update with bass-heavy audio
    let delta_time = duration as f32;
    let result = engine.update(&audio_samples, delta_time);
    
    // Should detect beat and potentially change preset
    assert!(result.is_ok());
    
    // Beat detector should be enabled
    assert!(engine.beat_detector().is_enabled());
}

#[test]
fn test_e2e_performance_baseline() {
    // Baseline performance test
    use std::time::Instant;
    
    let config = EngineConfig::default();
    
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    let audio_samples = vec![0.0; 735];
    let delta_time = 1.0 / 60.0;
    
    // Warm up
    for _ in 0..10 {
        engine.update(&audio_samples, delta_time).unwrap();
    }
    
    // Measure 60 frames
    let start = Instant::now();
    for _ in 0..60 {
        engine.update(&audio_samples, delta_time).unwrap();
    }
    let elapsed = start.elapsed();
    
    // Should be able to render 60 frames in less than 1 second
    // (This is a baseline, actual performance depends on GPU)
    println!("Rendered 60 frames in {:?}", elapsed);
    println!("Average frame time: {:?}", elapsed / 60);
    
    // Very lenient check (10 seconds for 60 frames = 6 FPS minimum)
    // Real target is 60 FPS (1 second for 60 frames)
    assert!(elapsed < Duration::from_secs(10), 
            "Performance too slow: {:?} for 60 frames", elapsed);
}

#[test]
fn test_e2e_with_preset() {
    // Test complete pipeline with a real preset loaded
    let config = EngineConfig::default();

    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();

    // Load a preset
    let preset_file = preset_path("$$$ Royal - Mashup (151).milk");
    engine.load_preset(&preset_file).expect("Failed to load preset");
    
    // Simulate varied audio
    let sample_rate = 44100.0;
    let duration = 1.0 / 60.0;
    let samples_per_frame = (sample_rate * duration) as usize;
    
    // Render 120 frames (2 seconds) with varying audio
    for frame in 0..120 {
        let mut audio_samples = Vec::with_capacity(samples_per_frame);
        
        for i in 0..samples_per_frame {
            let t = (frame * samples_per_frame + i) as f32 / sample_rate;
            // Mix of bass (60 Hz) and treble (4000 Hz)
            let bass = (2.0 * std::f32::consts::PI * 60.0 * t).sin() * 0.5;
            let treb = (2.0 * std::f32::consts::PI * 4000.0 * t).sin() * 0.3;
            audio_samples.push(bass + treb);
        }
        
        let result = engine.update(&audio_samples, duration);
        assert!(result.is_ok(), "Frame {} failed", frame);
    }
    
    // Verify state
    let state = engine.state();
    assert_eq!(state.frame, 120);
    assert!((state.time - 2.0).abs() < 0.01);
}
