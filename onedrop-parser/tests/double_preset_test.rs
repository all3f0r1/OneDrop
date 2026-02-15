use onedrop_parser::{BlendPattern, parse_double_preset};

#[test]
fn test_parse_plasma_blend() {
    let content = std::fs::read_to_string("../examples/double-presets/plasma_blend.od2")
        .expect("Failed to read plasma_blend.od2");

    let result = parse_double_preset(&content);
    assert!(
        result.is_ok(),
        "Failed to parse plasma_blend.od2: {:?}",
        result.err()
    );

    let double = result.unwrap();

    // Check blend settings
    assert_eq!(double.blend_pattern, BlendPattern::Plasma);
    assert_eq!(double.blend_amount, 0.5);
    assert!(double.animate_blend);
    assert_eq!(double.animation_speed, 1.0);

    // Check that both presets were parsed
    assert_eq!(double.preset_a.version, 201);
    assert_eq!(double.preset_b.version, 201);

    // Check that per_frame equations were parsed
    assert!(!double.preset_a.per_frame_equations.is_empty());
    assert!(!double.preset_b.per_frame_equations.is_empty());

    println!("✅ Plasma blend preset parsed successfully!");
    println!("   Pattern: {}", double.blend_pattern.name());
    println!(
        "   Preset A equations: {}",
        double.preset_a.per_frame_equations.len()
    );
    println!(
        "   Preset B equations: {}",
        double.preset_b.per_frame_equations.len()
    );
}

#[test]
fn test_blend_pattern_names() {
    let patterns = BlendPattern::all();
    assert_eq!(patterns.len(), 27);

    for (i, pattern) in patterns.iter().enumerate() {
        assert_eq!(BlendPattern::from_index(i), Some(*pattern));
        println!("{:2}. {}", i, pattern.name());
    }
}

#[test]
fn test_double_preset_builder() {
    use onedrop_parser::{DoublePreset, MilkPreset};

    let preset_a = MilkPreset::default();
    let preset_b = MilkPreset::default();

    let double = DoublePreset::new(preset_a, preset_b)
        .with_pattern(BlendPattern::Voronoi)
        .with_blend_amount(0.75)
        .with_animation(2.5);

    assert_eq!(double.blend_pattern, BlendPattern::Voronoi);
    assert_eq!(double.blend_amount, 0.75);
    assert!(double.animate_blend);
    assert_eq!(double.animation_speed, 2.5);
}
