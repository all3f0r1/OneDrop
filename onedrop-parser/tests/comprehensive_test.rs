use onedrop_parser::parse_preset;
use std::fs;
use std::path::Path;

#[test]
fn test_200_random_presets() {
    let preset_dir = Path::new("../test-presets-200");
    
    if !preset_dir.exists() {
        println!("Skipping comprehensive test: preset directory not found");
        return;
    }
    
    let mut total = 0;
    let mut success = 0;
    let mut failures = Vec::new();
    
    for entry in fs::read_dir(preset_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("milk") {
            continue;
        }
        
        total += 1;
        
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                failures.push((path.file_name().unwrap().to_string_lossy().to_string(), 
                              format!("Read error: {}", e)));
                continue;
            }
        };
        
        match parse_preset(&content) {
            Ok(preset) => {
                success += 1;
                
                // Basic validation
                assert!(preset.version > 0, "Invalid version in {:?}", path);
                assert!(preset.parameters.decay() >= 0.0 && preset.parameters.decay() <= 1.0,
                       "Invalid decay in {:?}", path);
            }
            Err(e) => {
                failures.push((path.file_name().unwrap().to_string_lossy().to_string(),
                              format!("Parse error: {}", e)));
            }
        }
    }
    
    let success_rate = (success as f32 / total as f32) * 100.0;
    
    println!("\n=== Comprehensive Preset Test Results ===");
    println!("Total presets: {}", total);
    println!("Successful: {} ({:.1}%)", success, success_rate);
    println!("Failed: {}", failures.len());
    
    if !failures.is_empty() {
        println!("\nFailures:");
        for (name, error) in failures.iter().take(10) {
            println!("  - {}: {}", name, error);
        }
        if failures.len() > 10 {
            println!("  ... and {} more", failures.len() - 10);
        }
    }
    
    // We expect at least 80% success rate
    assert!(success_rate >= 80.0, 
            "Success rate too low: {:.1}% (expected >= 80%)", success_rate);
}

#[test]
fn test_preset_statistics() {
    let preset_dir = Path::new("../test-presets-200");
    
    if !preset_dir.exists() {
        println!("Skipping statistics test: preset directory not found");
        return;
    }
    
    let mut stats = PresetStats::default();
    
    for entry in fs::read_dir(preset_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) != Some("milk") {
            continue;
        }
        
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(preset) = parse_preset(&content) {
                stats.total += 1;
                stats.per_frame_count += preset.per_frame_equations.len();
                stats.per_pixel_count += preset.per_pixel_equations.len();
                stats.wave_count += preset.waves.len();
                stats.shape_count += preset.shapes.len();
                
                if preset.warp_shader.is_some() {
                    stats.with_warp_shader += 1;
                }
                if preset.comp_shader.is_some() {
                    stats.with_comp_shader += 1;
                }
            }
        }
    }
    
    if stats.total > 0 {
        println!("\n=== Preset Statistics ===");
        println!("Total presets analyzed: {}", stats.total);
        println!("Average per-frame equations: {:.1}", 
                 stats.per_frame_count as f32 / stats.total as f32);
        println!("Average per-pixel equations: {:.1}", 
                 stats.per_pixel_count as f32 / stats.total as f32);
        println!("Average waves: {:.1}", 
                 stats.wave_count as f32 / stats.total as f32);
        println!("Average shapes: {:.1}", 
                 stats.shape_count as f32 / stats.total as f32);
        println!("With warp shader: {} ({:.1}%)", 
                 stats.with_warp_shader,
                 stats.with_warp_shader as f32 / stats.total as f32 * 100.0);
        println!("With comp shader: {} ({:.1}%)", 
                 stats.with_comp_shader,
                 stats.with_comp_shader as f32 / stats.total as f32 * 100.0);
    }
}

#[derive(Default)]
struct PresetStats {
    total: usize,
    per_frame_count: usize,
    per_pixel_count: usize,
    wave_count: usize,
    shape_count: usize,
    with_warp_shader: usize,
    with_comp_shader: usize,
}
