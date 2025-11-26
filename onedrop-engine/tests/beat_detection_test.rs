//! Comprehensive tests for beat detection functionality.

use onedrop_engine::{BeatDetectionMode, BeatDetector, PresetChange};
use std::thread;
use std::time::Duration;

#[test]
fn test_beat_detector_default_state() {
    let detector = BeatDetector::new();
    
    assert_eq!(*detector.mode(), BeatDetectionMode::Off);
    assert!(!detector.is_enabled());
}

#[test]
fn test_hardcut1_bass_detection() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut1);
    
    // Test below threshold - should not trigger
    let change = detector.should_change_preset(1.0, 0.5, 0.5);
    assert_eq!(change, None, "Should not trigger below bass threshold 1.5");
    
    // Test at threshold - should trigger
    let change = detector.should_change_preset(1.6, 0.5, 0.5);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger at bass > 1.5");
    
    // Test immediate retrigger - should be blocked by min delay
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert_eq!(change, None, "Should be blocked by 0.2s min delay");
    
    // Wait for min delay (200ms)
    thread::sleep(Duration::from_millis(250));
    
    // Test after delay - should trigger again
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger after min delay");
}

#[test]
fn test_hardcut2_treb_detection() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut2);
    
    // Test below threshold
    let change = detector.should_change_preset(0.5, 0.5, 2.0);
    assert_eq!(change, None, "Should not trigger below treb threshold 2.9");
    
    // Test above threshold
    let change = detector.should_change_preset(0.5, 0.5, 3.0);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger at treb > 2.9");
    
    // Test min delay (500ms)
    let change = detector.should_change_preset(0.5, 0.5, 3.5);
    assert_eq!(change, None, "Should be blocked by 0.5s min delay");
    
    thread::sleep(Duration::from_millis(550));
    
    let change = detector.should_change_preset(0.5, 0.5, 3.5);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger after 0.5s delay");
}

#[test]
fn test_hardcut3_longer_delay() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut3);
    
    // First trigger
    let change = detector.should_change_preset(0.5, 0.5, 3.0);
    assert_eq!(change, Some(PresetChange::Random), "First trigger should work");
    
    // Immediate retrigger - blocked
    let change = detector.should_change_preset(0.5, 0.5, 3.5);
    assert_eq!(change, None, "Should be blocked by 1s min delay");
    
    // Wait 1 second
    thread::sleep(Duration::from_millis(1050));
    
    // Should trigger again
    let change = detector.should_change_preset(0.5, 0.5, 3.5);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger after 1s delay");
}

#[test]
fn test_hardcut4_dual_threshold() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut4);
    
    // Test normal threshold (treb > 2.9)
    let change = detector.should_change_preset(0.5, 0.5, 3.0);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger at treb > 2.9");
    
    // Test immediate retrigger with normal threshold - blocked by delay
    let change = detector.should_change_preset(0.5, 0.5, 3.5);
    assert_eq!(change, None, "Should be blocked by 3s min delay");
    
    // Test immediate trigger with very high treble (treb > 8)
    // This should bypass the min delay
    let change = detector.should_change_preset(0.5, 0.5, 9.0);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger immediately at treb > 8");
    
    // Another immediate high treble trigger
    let change = detector.should_change_preset(0.5, 0.5, 10.0);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger again at treb > 8");
}

#[test]
fn test_hardcut5_long_delay() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut5);
    
    // First trigger
    let change = detector.should_change_preset(0.5, 0.5, 3.0);
    assert_eq!(change, Some(PresetChange::Random), "First trigger should work");
    
    // Immediate retrigger - blocked
    let change = detector.should_change_preset(0.5, 0.5, 4.0);
    assert_eq!(change, None, "Should be blocked by 5s min delay");
    
    // Even after 3 seconds, still blocked
    thread::sleep(Duration::from_millis(3000));
    let change = detector.should_change_preset(0.5, 0.5, 4.0);
    assert_eq!(change, None, "Should still be blocked after 3s");
    
    // After 5 seconds, should trigger
    thread::sleep(Duration::from_millis(2100));
    let change = detector.should_change_preset(0.5, 0.5, 4.0);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger after 5s delay");
}

#[test]
fn test_hardcut6_special_preset() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut6 {
        special_preset: "Bass/WHITE.milk".to_string(),
    });
    
    // Test normal bass (1.5 < bass < 4.90)
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert_eq!(change, Some(PresetChange::Random), "Should load random preset at bass > 1.5");
    
    // Test very high bass (bass > 4.90) - should load special preset
    let change = detector.should_change_preset(5.0, 0.5, 0.5);
    assert_eq!(
        change,
        Some(PresetChange::Specific("Bass/WHITE.milk".to_string())),
        "Should load special preset at bass > 4.90"
    );
    
    // Test below threshold
    let change = detector.should_change_preset(1.0, 0.5, 0.5);
    assert_eq!(change, None, "Should not trigger below bass threshold 1.5");
}

#[test]
fn test_mode_cycling() {
    let mut detector = BeatDetector::new();
    
    // Start at Off
    assert_eq!(*detector.mode(), BeatDetectionMode::Off);
    assert!(!detector.is_enabled());
    
    // Cycle through all modes
    detector.next_mode();
    assert_eq!(*detector.mode(), BeatDetectionMode::HardCut1);
    assert!(detector.is_enabled());
    
    detector.next_mode();
    assert_eq!(*detector.mode(), BeatDetectionMode::HardCut2);
    
    detector.next_mode();
    assert_eq!(*detector.mode(), BeatDetectionMode::HardCut3);
    
    detector.next_mode();
    assert_eq!(*detector.mode(), BeatDetectionMode::HardCut4);
    
    detector.next_mode();
    assert_eq!(*detector.mode(), BeatDetectionMode::HardCut5);
    
    detector.next_mode();
    assert!(matches!(detector.mode(), BeatDetectionMode::HardCut6 { .. }));
    
    // Cycle back to Off
    detector.next_mode();
    assert_eq!(*detector.mode(), BeatDetectionMode::Off);
    assert!(!detector.is_enabled());
}

#[test]
fn test_enable_disable() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut1);
    
    assert!(detector.is_enabled());
    
    // Disable
    detector.disable();
    assert!(!detector.is_enabled());
    
    // Should not trigger when disabled
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert_eq!(change, None, "Should not trigger when disabled");
    
    // Enable
    detector.enable();
    assert!(detector.is_enabled());
    
    // Should trigger when enabled
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert_eq!(change, Some(PresetChange::Random), "Should trigger when enabled");
}

#[test]
fn test_set_mode() {
    let mut detector = BeatDetector::new();
    
    // Set to HardCut3
    detector.set_mode(BeatDetectionMode::HardCut3);
    assert_eq!(*detector.mode(), BeatDetectionMode::HardCut3);
    assert!(detector.is_enabled());
    
    // Set to Off
    detector.set_mode(BeatDetectionMode::Off);
    assert_eq!(*detector.mode(), BeatDetectionMode::Off);
    assert!(!detector.is_enabled());
}

#[test]
fn test_mode_names() {
    assert_eq!(BeatDetectionMode::Off.name(), "Off");
    assert_eq!(BeatDetectionMode::HardCut1.name(), "HardCut1");
    assert_eq!(BeatDetectionMode::HardCut2.name(), "HardCut2");
    assert_eq!(BeatDetectionMode::HardCut3.name(), "HardCut3");
    assert_eq!(BeatDetectionMode::HardCut4.name(), "HardCut4");
    assert_eq!(BeatDetectionMode::HardCut5.name(), "HardCut5");
    assert_eq!(
        BeatDetectionMode::HardCut6 {
            special_preset: "test.milk".to_string()
        }
        .name(),
        "HardCut6"
    );
}

#[test]
fn test_realistic_audio_scenario() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut1);
    
    // Simulate a realistic audio sequence
    let audio_sequence = vec![
        (0.5, 0.3, 0.4),  // Quiet
        (0.8, 0.5, 0.6),  // Building up
        (1.2, 0.7, 0.8),  // Getting louder
        (1.8, 0.9, 1.0),  // Beat! Should trigger
        (1.5, 0.8, 0.9),  // After beat
        (1.0, 0.6, 0.7),  // Quieter
    ];
    
    let mut triggered_count = 0;
    
    for (bass, mid, treb) in audio_sequence {
        if let Some(_) = detector.should_change_preset(bass, mid, treb) {
            triggered_count += 1;
        }
        thread::sleep(Duration::from_millis(50)); // Simulate frame time
    }
    
    // Should trigger once at the beat
    assert_eq!(triggered_count, 1, "Should trigger exactly once at the beat");
}

#[test]
fn test_multiple_beats_with_delays() {
    let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut1);
    
    // First beat
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert!(change.is_some(), "First beat should trigger");
    
    // Wait 100ms - still blocked
    thread::sleep(Duration::from_millis(100));
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert!(change.is_none(), "Should be blocked after 100ms");
    
    // Wait another 150ms (total 250ms) - should trigger
    thread::sleep(Duration::from_millis(150));
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert!(change.is_some(), "Should trigger after 250ms");
    
    // Wait 250ms again
    thread::sleep(Duration::from_millis(250));
    let change = detector.should_change_preset(2.0, 0.5, 0.5);
    assert!(change.is_some(), "Should trigger again after another 250ms");
}

#[test]
fn test_preset_change_types() {
    // Test Random
    let random = PresetChange::Random;
    assert_eq!(random, PresetChange::Random);
    
    // Test Specific
    let specific = PresetChange::Specific("test.milk".to_string());
    assert_eq!(specific, PresetChange::Specific("test.milk".to_string()));
    
    // Test inequality
    assert_ne!(random, specific);
}
