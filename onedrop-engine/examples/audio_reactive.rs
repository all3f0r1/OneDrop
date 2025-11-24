//! Audio-reactive visualization example.

use onedrop_engine::{EngineConfig, MilkEngine};
use std::f32::consts::PI;

fn main() {
    env_logger::init();
    
    println!("Creating audio-reactive Milkdrop visualization...");
    
    let config = EngineConfig::default();
    let mut engine = pollster::block_on(MilkEngine::new(config))
        .expect("Failed to create engine");
    
    println!("Engine created!");
    
    // Simulate 120 frames (2 seconds at 60 FPS)
    println!("\nRendering audio-reactive visualization...");
    
    for frame in 0..120 {
        let time = frame as f32 / 60.0;
        
        // Generate audio with varying frequencies to simulate music
        let audio_samples: Vec<f32> = (0..1024)
            .map(|i| {
                let t = (frame * 1024 + i) as f32 * 0.001;
                
                // Bass (low frequency)
                let bass = (t * 2.0 * PI * 60.0).sin() * 0.8;
                
                // Mid (medium frequency)
                let mid = (t * 2.0 * PI * 200.0).sin() * 0.5;
                
                // Treble (high frequency)
                let treb = (t * 2.0 * PI * 1000.0).sin() * 0.3;
                
                (bass + mid + treb) / 3.0
            })
            .collect();
        
        engine.update(&audio_samples, 0.016)
            .expect("Failed to update engine");
        
        let state = engine.state();
        
        if frame % 10 == 0 {
            println!(
                "Frame {:3}: time={:.2}s, bass={:.2}, mid={:.2}, treb={:.2}, zoom={:.3}",
                frame,
                state.time,
                state.audio.bass,
                state.audio.mid,
                state.audio.treb,
                state.motion.zoom
            );
        }
    }
    
    println!("\nAudio-reactive example completed!");
    println!("Final state:");
    let state = engine.state();
    println!("  Time: {:.2}s", state.time);
    println!("  Frame: {}", state.frame);
    println!("  Audio levels: bass={:.2}, mid={:.2}, treb={:.2}", 
             state.audio.bass, state.audio.mid, state.audio.treb);
    println!("  Motion: zoom={:.3}, rot={:.3}", 
             state.motion.zoom, state.motion.rot);
}
