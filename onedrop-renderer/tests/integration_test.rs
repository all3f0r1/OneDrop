use onedrop_renderer::{AudioLevels, MilkRenderer, MotionParams, RenderConfig, RenderState};

#[test]
fn test_renderer_initialization() {
    env_logger::try_init().ok();
    
    let config = RenderConfig::default();
    let renderer = pollster::block_on(MilkRenderer::new(config));
    
    assert!(renderer.is_ok(), "Failed to create renderer");
}

#[test]
fn test_single_frame_render() {
    env_logger::try_init().ok();
    
    let config = RenderConfig {
        width: 800,
        height: 600,
        ..Default::default()
    };
    
    let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
    
    let result = renderer.render();
    assert!(result.is_ok(), "Failed to render frame");
}

#[test]
fn test_multiple_frames() {
    env_logger::try_init().ok();
    
    let config = RenderConfig::default();
    let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
    
    // Render 10 frames
    for i in 0..10 {
        let mut state = RenderState::default();
        state.time = i as f32 * 0.016; // ~60 FPS
        state.frame = i;
        
        renderer.update_state(state);
        let result = renderer.render();
        assert!(result.is_ok(), "Failed to render frame {}", i);
    }
}

#[test]
fn test_motion_effects() {
    env_logger::try_init().ok();
    
    let config = RenderConfig::default();
    let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
    
    let mut state = RenderState::default();
    
    // Test zoom
    state.motion.zoom = 1.5;
    renderer.update_state(state.clone());
    assert!(renderer.render().is_ok());
    
    // Test rotation
    state.motion.rot = 0.5;
    renderer.update_state(state.clone());
    assert!(renderer.render().is_ok());
    
    // Test translation
    state.motion.dx = 0.1;
    state.motion.dy = -0.1;
    renderer.update_state(state.clone());
    assert!(renderer.render().is_ok());
}

#[test]
fn test_audio_reactive() {
    env_logger::try_init().ok();
    
    let config = RenderConfig::default();
    let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
    
    // Simulate audio beat
    let mut state = RenderState::default();
    state.audio = AudioLevels {
        bass: 1.0,
        mid: 0.5,
        treb: 0.3,
        bass_att: 0.8,
        mid_att: 0.4,
        treb_att: 0.2,
    };
    
    // Audio-reactive zoom
    state.motion.zoom = 1.0 + 0.2 * state.audio.bass;
    
    renderer.update_state(state);
    assert!(renderer.render().is_ok());
}

#[test]
fn test_animated_sequence() {
    env_logger::try_init().ok();
    
    let config = RenderConfig::default();
    let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
    
    // Render animated sequence
    for frame in 0..60 {
        let time = frame as f32 / 60.0;
        
        let mut state = RenderState::default();
        state.time = time;
        state.frame = frame;
        
        // Animated zoom
        state.motion.zoom = 1.0 + 0.1 * (time * 2.0 * std::f32::consts::PI).sin();
        
        // Animated rotation
        state.motion.rot = time * 0.5;
        
        renderer.update_state(state);
        let result = renderer.render();
        assert!(result.is_ok(), "Failed to render frame {} of animated sequence", frame);
    }
}

#[test]
fn test_different_resolutions() {
    env_logger::try_init().ok();
    
    let resolutions = vec![
        (640, 480),
        (1280, 720),
        (1920, 1080),
    ];
    
    for (width, height) in resolutions {
        let config = RenderConfig {
            width,
            height,
            ..Default::default()
        };
        
        let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
        let result = renderer.render();
        assert!(result.is_ok(), "Failed to render at {}x{}", width, height);
    }
}

#[test]
fn test_state_persistence() {
    env_logger::try_init().ok();
    
    let config = RenderConfig::default();
    let mut renderer = pollster::block_on(MilkRenderer::new(config)).unwrap();
    
    // Set initial state
    let mut state = RenderState::default();
    state.motion.zoom = 1.5;
    state.motion.rot = 0.3;
    renderer.update_state(state.clone());
    
    // Render
    renderer.render().unwrap();
    
    // Verify state persists
    assert_eq!(renderer.state().motion.zoom, 1.5);
    assert_eq!(renderer.state().motion.rot, 0.3);
}
