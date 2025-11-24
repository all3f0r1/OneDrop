use milk_engine::{EngineConfig, MilkEngine, PresetManager};
use std::path::Path;

#[test]
fn test_engine_initialization() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let engine = pollster::block_on(MilkEngine::new(config));
    
    assert!(engine.is_ok());
}

#[test]
fn test_engine_update_without_preset() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    let audio_samples = vec![0.0; 1024];
    let result = engine.update(&audio_samples, 0.016);
    
    assert!(result.is_ok());
}

#[test]
fn test_engine_multiple_frames() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    for i in 0..60 {
        let audio_samples: Vec<f32> = (0..1024)
            .map(|j| ((i + j) as f32 * 0.1).sin() * 0.5)
            .collect();
        
        let result = engine.update(&audio_samples, 0.016);
        assert!(result.is_ok(), "Failed at frame {}", i);
    }
    
    assert_eq!(engine.state().frame, 60);
}

#[test]
fn test_engine_with_preset() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    // Try to load a test preset if available
    let test_preset_path = "../test-presets/10.milk";
    
    if Path::new(test_preset_path).exists() {
        let result = engine.load_preset(test_preset_path);
        assert!(result.is_ok(), "Failed to load preset");
        
        // Update with preset loaded
        let audio_samples = vec![0.5; 1024];
        let result = engine.update(&audio_samples, 0.016);
        assert!(result.is_ok());
        
        // Verify preset is loaded
        assert!(engine.current_preset().is_some());
    } else {
        println!("Skipping preset test: test preset not found");
    }
}

#[test]
fn test_audio_analysis() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    // Generate audio with strong bass
    let audio_samples: Vec<f32> = (0..1024)
        .map(|i| (i as f32 * 0.01).sin())
        .collect();
    
    engine.update(&audio_samples, 0.016).unwrap();
    
    let state = engine.state();
    assert!(state.audio.bass > 0.0, "Bass level should be > 0");
}

#[test]
fn test_time_progression() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    let audio_samples = vec![0.0; 1024];
    
    // First frame
    engine.update(&audio_samples, 0.016).unwrap();
    let time1 = engine.state().time;
    
    // Second frame
    engine.update(&audio_samples, 0.016).unwrap();
    let time2 = engine.state().time;
    
    // Time should have progressed
    assert!(time2 > time1);
    assert!((time2 - time1 - 0.016).abs() < 0.001);
}

#[test]
fn test_engine_reset() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    // Run some frames
    let audio_samples = vec![0.5; 1024];
    for _ in 0..10 {
        engine.update(&audio_samples, 0.016).unwrap();
    }
    
    assert!(engine.state().time > 0.0);
    assert!(engine.state().frame > 0);
    
    // Reset
    engine.reset();
    
    assert_eq!(engine.state().time, 0.0);
    assert_eq!(engine.state().frame, 0);
}

#[test]
fn test_preset_manager() {
    let mut manager = PresetManager::new();
    
    manager.add_preset("preset1.milk");
    manager.add_preset("preset2.milk");
    manager.add_preset("preset3.milk");
    
    assert_eq!(manager.preset_count(), 3);
    
    // Test navigation
    assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset1.milk");
    
    manager.next_preset();
    assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset2.milk");
    
    manager.prev_preset();
    assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset1.milk");
}

#[test]
fn test_preset_transitions() {
    let mut manager = PresetManager::new();
    
    manager.add_preset("preset1.milk");
    manager.add_preset("preset2.milk");
    
    // Start transition
    manager.start_transition(1.0);
    assert!(manager.is_transitioning());
    
    // Update halfway
    manager.update_transition(0.5);
    assert!(manager.is_transitioning());
    assert!((manager.transition_progress() - 0.5).abs() < 0.01);
    
    // Complete transition
    let complete = manager.update_transition(0.6);
    assert!(complete);
    assert!(!manager.is_transitioning());
}

#[test]
fn test_engine_state_consistency() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    let audio_samples = vec![0.5; 1024];
    
    for i in 0..30 {
        engine.update(&audio_samples, 0.016).unwrap();
        
        let state = engine.state();
        
        // Verify state consistency
        assert_eq!(state.frame, i + 1);
        assert!((state.time - (i + 1) as f32 * 0.016).abs() < 0.001);
        
        // Audio levels should be reasonable
        assert!(state.audio.bass >= 0.0 && state.audio.bass <= 1.0);
        assert!(state.audio.mid >= 0.0 && state.audio.mid <= 1.0);
        assert!(state.audio.treb >= 0.0 && state.audio.treb <= 1.0);
    }
}

#[test]
fn test_different_audio_patterns() {
    env_logger::try_init().ok();
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config)).unwrap();
    
    // Test with silence
    let silence = vec![0.0; 1024];
    engine.update(&silence, 0.016).unwrap();
    assert!(engine.state().audio.bass < 0.1);
    
    // Test with loud signal
    let loud: Vec<f32> = (0..1024).map(|i| (i as f32 * 0.1).sin()).collect();
    engine.update(&loud, 0.016).unwrap();
    assert!(engine.state().audio.bass > 0.0);
}
