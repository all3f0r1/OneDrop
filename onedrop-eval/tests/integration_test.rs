use approx::assert_relative_eq;
use onedrop_eval::MilkEvaluator;

#[test]
fn test_real_world_per_frame_equations() {
    let mut eval = MilkEvaluator::new();

    // Simulate time progression
    eval.context_mut().set_time(1.0);
    eval.context_mut().set_frame(60.0);

    // Simulate audio data
    eval.context_mut().set_audio(0.5, 0.3, 0.7);

    // Real equations from preset "10.milk"
    let equations = vec![
        "wave_r = 0.3".to_string(),
        "wave_g = 0.25".to_string(),
        "wave_b = 0.6 + 0.40 * (0.60 * sin(1.251 * time) + 0.40 * sin(1.055 * time))".to_string(),
        "rot = rot + 0.010 * sin(time * 0.1)".to_string(),
        "zoom = 0.996".to_string(),
    ];

    eval.eval_per_frame(&equations).unwrap();

    // Verify results
    assert_relative_eq!(eval.context().get_var("wave_r").unwrap(), 0.3);
    assert_relative_eq!(eval.context().get_var("wave_g").unwrap(), 0.25);
    assert_relative_eq!(eval.context().get_var("zoom").unwrap(), 0.996);

    // wave_b should be computed correctly
    let wave_b = eval.context().get_var("wave_b").unwrap();
    assert!(
        wave_b > 0.0 && wave_b < 2.0,
        "wave_b out of expected range: {}",
        wave_b
    );
}

#[test]
fn test_real_world_per_pixel_equations() {
    let mut eval = MilkEvaluator::new();

    // Initialize context with audio levels
    eval.context_mut().set_time(1.0);
    eval.context_mut().set_audio(0.8, 0.5, 0.9);
    // Set attenuated audio values (normally computed by audio analyzer)
    eval.context_mut().set("bass_att", 0.8);
    eval.context_mut().set("treb_att", 0.9);
    eval.context_mut().set_var("zoom", 1.0);

    // Real equations from preset "10.milk"
    let equations = vec![
        "zoom = zoom + 0.08 * rad * treb_att * bass_att".to_string(),
        "rot = rot + 0.05 * sin(rad * time * 20)".to_string(),
    ];

    // Test at center pixel
    eval.eval_per_pixel(0.5, 0.5, 0.0, 0.0, &equations).unwrap();
    let zoom_center = eval.context().get_var("zoom").unwrap();
    assert_relative_eq!(zoom_center, 1.0, epsilon = 0.01); // At center, rad=0, so zoom unchanged

    // Test at edge pixel
    eval.context_mut().set_var("zoom", 1.0);
    eval.context_mut().set_var("rot", 0.0);
    eval.eval_per_pixel(1.0, 1.0, 0.707, 0.785, &equations)
        .unwrap();
    let zoom_edge = eval.context().get_var("zoom").unwrap();
    assert!(
        zoom_edge > 1.0,
        "Zoom should increase at edge: {}",
        zoom_edge
    );
}

#[test]
fn test_q_variables_persistence() {
    let mut eval = MilkEvaluator::new();

    // Set q variables in init
    eval.eval("q1 = 1.5").unwrap();
    eval.eval("q2 = 2.5").unwrap();
    eval.eval("q3 = q1 + q2").unwrap();

    assert_relative_eq!(eval.context().get_var("q1").unwrap(), 1.5);
    assert_relative_eq!(eval.context().get_var("q2").unwrap(), 2.5);
    assert_relative_eq!(eval.context().get_var("q3").unwrap(), 4.0);

    // Use q variables in per-frame
    eval.eval("wave_r = q1 * 0.5").unwrap();
    assert_relative_eq!(eval.context().get_var("wave_r").unwrap(), 0.75);
}

#[test]
fn test_complex_math_expressions() {
    let mut eval = MilkEvaluator::new();
    eval.context_mut().set_time(1.0);

    // Complex nested expression
    let result = eval
        .eval("sin(time) * cos(time * 2) + sqrt(abs(time - 0.5))")
        .unwrap();

    let time = 1.0_f64;
    let expected = time.sin() * (time * 2.0).cos() + (time - 0.5).abs().sqrt();
    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_conditional_like_behavior() {
    let mut eval = MilkEvaluator::new();

    // Milkdrop uses above/below functions, but we can simulate with min/max
    eval.context_mut().set_var("bass", 0.8);

    // If bass > 0.5, use 1.0, else use 0.5
    // In Milkdrop: above(bass, 0.5) * 0.5 + 0.5
    // We can approximate with: min(1, max(0, (bass - 0.5) * 10)) * 0.5 + 0.5
    let result = eval
        .eval("min(1, max(0, (bass - 0.5) * 10)) * 0.5 + 0.5")
        .unwrap();
    assert!(result > 0.5);
}

#[test]
fn test_multiple_assignments_in_sequence() {
    let mut eval = MilkEvaluator::new();

    // Chain of assignments
    eval.eval("a = 1").unwrap();
    eval.eval("b = a + 1").unwrap();
    eval.eval("c = a + b").unwrap();
    eval.eval("d = a * b * c").unwrap();

    assert_relative_eq!(eval.context().get_var("a").unwrap(), 1.0);
    assert_relative_eq!(eval.context().get_var("b").unwrap(), 2.0);
    assert_relative_eq!(eval.context().get_var("c").unwrap(), 3.0);
    assert_relative_eq!(eval.context().get_var("d").unwrap(), 6.0);
}

#[test]
fn test_edge_cases() {
    let mut eval = MilkEvaluator::new();

    // Empty expression
    let result = eval.eval("").unwrap();
    assert_relative_eq!(result, 0.0);

    // Expression with semicolon - assignment returns 0.0 but sets variable
    let result = eval.eval("zoom = 1.5;").unwrap();
    assert_relative_eq!(result, 0.0); // Assignments return Empty -> 0.0
    let zoom = eval.context().get_var("zoom").unwrap();
    assert_relative_eq!(zoom, 1.5); // But the variable is set

    // Expression with whitespace
    let result = eval.eval("  zoom = 2.0  ").unwrap();
    assert_relative_eq!(result, 0.0); // Assignments return Empty -> 0.0
    let zoom = eval.context().get_var("zoom").unwrap();
    assert_relative_eq!(zoom, 2.0); // But the variable is set
}

#[test]
fn test_performance_simulation() {
    let mut eval = MilkEvaluator::new();
    eval.context_mut().set_time(0.0);

    // Simulate 60 frames
    for frame in 0..60 {
        let time = frame as f64 / 60.0;
        eval.context_mut().set_time(time);
        eval.context_mut().set_frame(frame as f64);

        // Typical per-frame equations
        eval.eval("wave_r = 0.5 + 0.3 * sin(time * 2)").unwrap();
        eval.eval("wave_g = 0.5 + 0.3 * cos(time * 3)").unwrap();
        eval.eval("wave_b = 0.5 + 0.3 * sin(time * 4)").unwrap();
        eval.eval("zoom = 1.0 + 0.1 * sin(time)").unwrap();
        eval.eval("rot = rot + 0.01").unwrap();
    }

    // Verify final state
    let rot = eval.context().get_var("rot").unwrap();
    assert_relative_eq!(rot, 0.6, epsilon = 0.01);
}

#[test]
fn test_audio_reactive_equations() {
    let mut eval = MilkEvaluator::new();

    // Simulate beat
    eval.context_mut().set_audio(1.0, 0.5, 0.3);

    // Audio-reactive zoom
    eval.eval("zoom = 1.0 + 0.2 * bass").unwrap();
    let zoom = eval.context().get_var("zoom").unwrap();
    assert_relative_eq!(zoom, 1.2);

    // Audio-reactive color
    eval.eval("wave_r = bass").unwrap();
    eval.eval("wave_g = mid").unwrap();
    eval.eval("wave_b = treb").unwrap();

    assert_relative_eq!(eval.context().get_var("wave_r").unwrap(), 1.0);
    assert_relative_eq!(eval.context().get_var("wave_g").unwrap(), 0.5);
    assert_relative_eq!(eval.context().get_var("wave_b").unwrap(), 0.3);
}
