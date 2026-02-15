//! Complete pipeline example: Parse → Generate → Compile
//!
//! This example demonstrates the full workflow from a Milkdrop preset
//! to a compiled, validated WGSL shader.

use onedrop_codegen::{ShaderCompiler, ShaderGenerator};
use onedrop_parser::MilkPreset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    println!("=== OneDrop Complete Pipeline Example ===\n");

    // Step 1: Create a simple preset
    println!("Step 1: Creating a simple preset...");
    let mut preset = MilkPreset::default();
    preset
        .per_pixel_equations
        .push("x = x + 0.01*sin(time)".to_string());
    preset
        .per_pixel_equations
        .push("y = y + 0.01*cos(time)".to_string());
    println!(
        "  ✓ Preset created with {} per-pixel equations\n",
        preset.per_pixel_equations.len()
    );

    // Step 2: Generate WGSL shader
    println!("Step 2: Generating WGSL shader...");
    let generator = ShaderGenerator::new();
    let shader_source = generator.generate_per_pixel_shader(&preset)?;
    println!("  ✓ Shader generated ({} bytes)\n", shader_source.len());

    // Print a snippet of the generated shader
    println!("Generated shader snippet:");
    println!("---");
    for (i, line) in shader_source.lines().take(20).enumerate() {
        println!("{:3} | {}", i + 1, line);
    }
    println!(
        "    ... ({} more lines)",
        shader_source.lines().count() - 20
    );
    println!("---\n");

    // Step 3: Compile and validate shader
    println!("Step 3: Compiling and validating shader...");
    let mut compiler = ShaderCompiler::new();
    let compiled = compiler.compile(&shader_source)?;
    println!("  ✓ Shader compiled and validated successfully\n");

    // Step 4: Show compilation stats
    println!("Step 4: Compilation statistics:");
    let stats = compiler.cache_stats();
    println!("  Cache size: {} shaders", stats.size);
    println!("  Total cached: {} bytes", stats.total_source_bytes);
    println!("  Module functions: {}", compiled.module.functions.len());
    println!(
        "  Module entry points: {}",
        compiled.module.entry_points.len()
    );

    // Step 5: Test cache hit
    println!("\nStep 5: Testing cache...");
    let start = std::time::Instant::now();
    let _compiled2 = compiler.compile(&shader_source)?;
    let duration = start.elapsed();
    println!("  ✓ Cache hit! Compilation took {:?}", duration);

    println!("\n=== Pipeline Complete ===");
    println!("✓ All steps successful!");
    println!("  1. Preset created");
    println!("  2. Shader generated");
    println!("  3. Shader compiled");
    println!("  4. Shader validated");
    println!("  5. Cache working");

    Ok(())
}
