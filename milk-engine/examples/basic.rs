//! Basic example of using the Milkdrop engine.

use milk_engine::{EngineConfig, MilkEngine};

fn main() {
    env_logger::init();
    
    println!("Creating Milkdrop engine...");
    
    // Create engine with default configuration
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config))
        .expect("Failed to create engine");
    
    println!("Engine created successfully!");
    println!("Resolution: {}x{}", 
             engine.state().motion.zoom, 
             engine.state().motion.rot);
    
    // Simulate some frames without preset
    println!("\nRendering 10 frames without preset...");
    
    for i in 0..10 {
        // Generate some fake audio data
        let audio_samples: Vec<f32> = (0..1024)
            .map(|j| ((i + j) as f32 * 0.1).sin() * 0.5)
            .collect();
        
        // Update engine (60 FPS = 0.016s per frame)
        engine.update(&audio_samples, 0.016)
            .expect("Failed to update engine");
        
        println!("Frame {}: time={:.3}s", i, engine.state().time);
    }
    
    println!("\nBasic example completed successfully!");
}
