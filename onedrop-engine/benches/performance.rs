//! Performance benchmarks for OneDrop engine.

use onedrop_engine::{EngineConfig, MilkEngine};
use std::time::{Duration, Instant};

/// Benchmark configuration.
struct BenchConfig {
    width: u32,
    height: u32,
    frames: usize,
    with_preset: bool,
}

/// Benchmark results.
struct BenchResult {
    total_time: Duration,
    avg_frame_time: Duration,
    fps: f64,
    min_frame_time: Duration,
    max_frame_time: Duration,
}

impl BenchResult {
    fn print(&self, config: &BenchConfig) {
        println!("\n=== Performance Benchmark ===");
        println!("Resolution: {}x{}", config.width, config.height);
        println!("Frames: {}", config.frames);
        println!("With preset: {}", config.with_preset);
        println!("\nResults:");
        println!("  Total time: {:?}", self.total_time);
        println!("  Avg frame time: {:?}", self.avg_frame_time);
        println!("  FPS: {:.2}", self.fps);
        println!("  Min frame time: {:?}", self.min_frame_time);
        println!("  Max frame time: {:?}", self.max_frame_time);
        println!("============================\n");
    }
}

/// Run a performance benchmark.
fn run_benchmark(config: BenchConfig) -> BenchResult {
    let engine_config = EngineConfig {
        render_config: onedrop_renderer::RenderConfig {
            width: config.width,
            height: config.height,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut engine =
        pollster::block_on(MilkEngine::new(engine_config)).expect("Failed to create engine");

    // Load preset if requested
    if config.with_preset {
        if let Err(e) = engine.load_default_preset() {
            eprintln!("Warning: Failed to load default preset: {}", e);
        }
    }

    // Generate audio samples
    let audio_samples: Vec<f32> = (0..1024).map(|i| (i as f32 * 0.01).sin() * 0.5).collect();

    let delta_time = 1.0 / 60.0; // 60 FPS target

    // Warm up (10 frames)
    for _ in 0..10 {
        let _ = engine.update(&audio_samples, delta_time);
    }

    // Benchmark
    let mut frame_times = Vec::with_capacity(config.frames);
    let start = Instant::now();

    for _ in 0..config.frames {
        let frame_start = Instant::now();
        let _ = engine.update(&audio_samples, delta_time);
        frame_times.push(frame_start.elapsed());
    }

    let total_time = start.elapsed();

    // Calculate statistics
    let avg_frame_time = total_time / config.frames as u32;
    let fps = config.frames as f64 / total_time.as_secs_f64();
    let min_frame_time = *frame_times.iter().min().unwrap();
    let max_frame_time = *frame_times.iter().max().unwrap();

    BenchResult {
        total_time,
        avg_frame_time,
        fps,
        min_frame_time,
        max_frame_time,
    }
}

fn main() {
    env_logger::init();

    println!("OneDrop Performance Benchmarks");
    println!("===============================\n");

    // Benchmark 1: 720p without preset
    let result = run_benchmark(BenchConfig {
        width: 1280,
        height: 720,
        frames: 300, // 5 seconds @ 60 FPS
        with_preset: false,
    });
    result.print(&BenchConfig {
        width: 1280,
        height: 720,
        frames: 300,
        with_preset: false,
    });

    // Benchmark 2: 720p with preset
    let result = run_benchmark(BenchConfig {
        width: 1280,
        height: 720,
        frames: 300,
        with_preset: true,
    });
    result.print(&BenchConfig {
        width: 1280,
        height: 720,
        frames: 300,
        with_preset: true,
    });

    // Benchmark 3: 1080p without preset
    let result = run_benchmark(BenchConfig {
        width: 1920,
        height: 1080,
        frames: 300,
        with_preset: false,
    });
    result.print(&BenchConfig {
        width: 1920,
        height: 1080,
        frames: 300,
        with_preset: false,
    });

    // Benchmark 4: 1080p with preset
    let result = run_benchmark(BenchConfig {
        width: 1920,
        height: 1080,
        frames: 300,
        with_preset: true,
    });
    result.print(&BenchConfig {
        width: 1920,
        height: 1080,
        frames: 300,
        with_preset: true,
    });

    // Benchmark 5: 4K without preset
    let result = run_benchmark(BenchConfig {
        width: 3840,
        height: 2160,
        frames: 300,
        with_preset: false,
    });
    result.print(&BenchConfig {
        width: 3840,
        height: 2160,
        frames: 300,
        with_preset: false,
    });

    println!("All benchmarks completed!");
}
