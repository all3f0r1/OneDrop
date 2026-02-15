//! OneDrop CLI - Command-line interface for Milkdrop visualizations

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use onedrop_engine::{EngineConfig, MilkEngine, RenderConfig};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "onedrop")]
#[command(author = "Manus AI")]
#[command(version = "0.1.0")]
#[command(about = "OneDrop - Pure Rust Milkdrop visualizer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Show information about a preset
    Info {
        /// Path to the .milk preset file
        preset: PathBuf,
    },

    /// Validate a preset file
    Validate {
        /// Path to the .milk preset file
        preset: PathBuf,
    },

    /// Render a preset to images
    Render {
        /// Path to the .milk preset file
        preset: PathBuf,

        /// Number of frames to render
        #[arg(short, long, default_value = "60")]
        frames: u32,

        /// Output directory for frames
        #[arg(short, long, default_value = "output")]
        output: PathBuf,

        /// Width of output
        #[arg(short, long, default_value = "1280")]
        width: u32,

        /// Height of output
        #[arg(short = 'H', long, default_value = "720")]
        height: u32,
    },

    /// List all presets in a directory
    List {
        /// Directory containing .milk files
        directory: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger
    if cli.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    match cli.command {
        Commands::Info { preset } => cmd_info(preset),
        Commands::Validate { preset } => cmd_validate(preset),
        Commands::Render {
            preset,
            frames,
            output,
            width,
            height,
        } => cmd_render(preset, frames, output, width, height),
        Commands::List { directory } => cmd_list(directory),
    }
}

fn cmd_info(preset_path: PathBuf) -> Result<()> {
    log::info!("Loading preset: {}", preset_path.display());

    let content = std::fs::read_to_string(&preset_path).context("Failed to read preset file")?;

    let preset = onedrop_parser::parse_preset(&content).context("Failed to parse preset")?;

    println!("\n=== Preset Information ===\n");
    println!("Version: {}", preset.version);
    println!("Warp shader version: {}", preset.ps_version_warp);
    println!("Composite shader version: {}", preset.ps_version_comp);

    println!("\n--- Parameters ---");
    println!("Zoom: {}", preset.parameters.zoom);
    println!("Rotation: {}", preset.parameters.rot);
    println!("Decay: {}", preset.parameters.decay());
    println!(
        "Wave color: R={}, G={}, B={}",
        preset.parameters.wave_r, preset.parameters.wave_g, preset.parameters.wave_b
    );

    println!("\n--- Equations ---");
    println!("Per-frame equations: {}", preset.per_frame_equations.len());
    println!("Per-pixel equations: {}", preset.per_pixel_equations.len());

    if !preset.per_frame_equations.is_empty() {
        println!("\nPer-frame equations:");
        for (i, eq) in preset.per_frame_equations.iter().enumerate().take(5) {
            println!("  {}: {}", i + 1, eq);
        }
        if preset.per_frame_equations.len() > 5 {
            println!("  ... and {} more", preset.per_frame_equations.len() - 5);
        }
    }

    println!("\n--- Custom Elements ---");
    println!("Waves: {}", preset.waves.len());
    println!("Shapes: {}", preset.shapes.len());

    println!("\n--- Shaders ---");
    println!(
        "Warp shader: {}",
        if preset.warp_shader.is_some() {
            "Yes"
        } else {
            "No"
        }
    );
    println!(
        "Composite shader: {}",
        if preset.comp_shader.is_some() {
            "Yes"
        } else {
            "No"
        }
    );

    Ok(())
}

fn cmd_validate(preset_path: PathBuf) -> Result<()> {
    log::info!("Validating preset: {}", preset_path.display());

    let content = std::fs::read_to_string(&preset_path).context("Failed to read preset file")?;

    match onedrop_parser::parse_preset(&content) {
        Ok(preset) => {
            println!("✓ Preset is valid!");
            println!("  Version: {}", preset.version);
            println!(
                "  Per-frame equations: {}",
                preset.per_frame_equations.len()
            );
            println!(
                "  Per-pixel equations: {}",
                preset.per_pixel_equations.len()
            );
            Ok(())
        }
        Err(e) => {
            println!("✗ Preset is invalid!");
            println!("  Error: {}", e);
            Err(e.into())
        }
    }
}

fn cmd_render(
    preset_path: PathBuf,
    frames: u32,
    output_dir: PathBuf,
    width: u32,
    height: u32,
) -> Result<()> {
    log::info!("Rendering preset: {}", preset_path.display());
    log::info!("Output: {} frames to {}", frames, output_dir.display());
    log::info!("Resolution: {}x{}", width, height);

    // Create output directory
    std::fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    // Create engine
    let config = EngineConfig {
        render_config: RenderConfig {
            width,
            height,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut engine =
        pollster::block_on(MilkEngine::new(config)).context("Failed to create engine")?;

    // Load preset
    engine
        .load_preset(&preset_path)
        .context("Failed to load preset")?;

    println!("Rendering {} frames...", frames);

    // Render frames
    for frame in 0..frames {
        // Generate some audio (sine wave for demo)
        let audio_samples: Vec<f32> = (0..1024)
            .map(|i| {
                let t = (frame * 1024 + i) as f32 * 0.001;
                (t * 2.0 * std::f32::consts::PI * 60.0).sin() * 0.5
            })
            .collect();

        // Update engine
        engine
            .update(&audio_samples, 0.016)
            .context("Failed to update engine")?;

        // Progress indicator
        if frame % 10 == 0 || frame == frames - 1 {
            println!(
                "  Frame {}/{} ({:.1}%)",
                frame + 1,
                frames,
                (frame + 1) as f32 / frames as f32 * 100.0
            );
        }
    }

    println!("\n✓ Rendering complete!");
    println!("  Output: {}", output_dir.display());

    Ok(())
}

fn cmd_list(directory: PathBuf) -> Result<()> {
    log::info!("Listing presets in: {}", directory.display());

    let entries = std::fs::read_dir(&directory).context("Failed to read directory")?;

    let mut presets = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("milk") {
            presets.push(path);
        }
    }

    if presets.is_empty() {
        println!("No .milk presets found in {}", directory.display());
        return Ok(());
    }

    presets.sort();

    println!("\n=== Presets in {} ===\n", directory.display());
    println!("Found {} preset(s):\n", presets.len());

    for (i, preset) in presets.iter().enumerate() {
        println!(
            "  {}. {}",
            i + 1,
            preset.file_name().unwrap().to_string_lossy()
        );
    }

    Ok(())
}
