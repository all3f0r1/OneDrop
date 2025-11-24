use milk_parser::{parse_preset, MilkPreset};
use std::fs;
use std::path::Path;

#[test]
fn test_parse_single_preset() {
    let test_presets_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("test-presets");
    
    if let Ok(entries) = fs::read_dir(&test_presets_dir) {
        let mut found = false;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("milk") {
                    let content = fs::read_to_string(&path).unwrap();
                    let result = parse_preset(&content);
                    
                    assert!(result.is_ok(), "Failed to parse {}: {:?}", 
                            path.file_name().unwrap().to_string_lossy(), 
                            result.err());
                    
                    let preset = result.unwrap();
                    assert_eq!(preset.version, 201, "Expected version 201 in {}", 
                              path.file_name().unwrap().to_string_lossy());
                    
                    found = true;
                    break;
                }
            }
        }
        assert!(found, "No .milk files found in test-presets directory");
    } else {
        panic!("test-presets directory not found");
    }
}

#[test]
fn test_parse_all_test_presets() {
    let test_presets_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("test-presets");
    
    if !test_presets_dir.exists() {
        println!("Skipping test: test-presets directory not found");
        return;
    }
    
    let mut success_count = 0;
    let mut fail_count = 0;
    let mut failures = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&test_presets_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("milk") {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    
                    match fs::read_to_string(&path) {
                        Ok(content) => {
                            match parse_preset(&content) {
                                Ok(preset) => {
                                    success_count += 1;
                                    assert_eq!(preset.version, 201, "Invalid version in {}", filename);
                                }
                                Err(e) => {
                                    fail_count += 1;
                                    failures.push((filename.clone(), e.to_string()));
                                }
                            }
                        }
                        Err(e) => {
                            fail_count += 1;
                            failures.push((filename.clone(), format!("IO error: {}", e)));
                        }
                    }
                }
            }
        }
    }
    
    println!("\n=== Parsing Results ===");
    println!("✓ Successfully parsed: {}", success_count);
    println!("✗ Failed to parse: {}", fail_count);
    
    if !failures.is_empty() {
        println!("\nFailures:");
        for (filename, error) in &failures {
            println!("  - {}: {}", filename, error);
        }
    }
    
    let total = success_count + fail_count;
    let success_rate = if total > 0 {
        (success_count as f64 / total as f64) * 100.0
    } else {
        0.0
    };
    
    println!("\nSuccess rate: {:.1}%", success_rate);
    assert!(success_rate >= 90.0, "Success rate too low: {:.1}%", success_rate);
    assert!(success_count > 0, "Should parse at least one preset successfully");
}

#[test]
fn test_preset_structure_statistics() {
    let test_presets_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("test-presets");
    
    if !test_presets_dir.exists() {
        println!("Skipping test: test-presets directory not found");
        return;
    }
    
    let mut total = 0;
    let mut with_per_frame = 0;
    let mut with_per_pixel = 0;
    let mut with_warp_shader = 0;
    let mut with_comp_shader = 0;
    let mut with_waves = 0;
    let mut with_shapes = 0;
    
    if let Ok(entries) = fs::read_dir(&test_presets_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("milk") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(preset) = parse_preset(&content) {
                            total += 1;
                            if !preset.per_frame_equations.is_empty() { with_per_frame += 1; }
                            if !preset.per_pixel_equations.is_empty() { with_per_pixel += 1; }
                            if preset.warp_shader.is_some() { with_warp_shader += 1; }
                            if preset.comp_shader.is_some() { with_comp_shader += 1; }
                            if !preset.waves.is_empty() { with_waves += 1; }
                            if !preset.shapes.is_empty() { with_shapes += 1; }
                        }
                    }
                }
            }
        }
    }
    
    println!("\n=== Preset Structure Statistics ===");
    println!("Total presets: {}", total);
    if total > 0 {
        println!("With per-frame equations: {} ({:.1}%)", with_per_frame, (with_per_frame as f64 / total as f64) * 100.0);
        println!("With per-pixel equations: {} ({:.1}%)", with_per_pixel, (with_per_pixel as f64 / total as f64) * 100.0);
        println!("With warp shader: {} ({:.1}%)", with_warp_shader, (with_warp_shader as f64 / total as f64) * 100.0);
        println!("With comp shader: {} ({:.1}%)", with_comp_shader, (with_comp_shader as f64 / total as f64) * 100.0);
        println!("With custom waves: {} ({:.1}%)", with_waves, (with_waves as f64 / total as f64) * 100.0);
        println!("With custom shapes: {} ({:.1}%)", with_shapes, (with_shapes as f64 / total as f64) * 100.0);
    }
    
    assert!(total > 0, "Should analyze at least one preset");
}
