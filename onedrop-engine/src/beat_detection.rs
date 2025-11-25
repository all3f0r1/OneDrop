//! Beat detection for automatic preset changing.

use std::time::{Duration, Instant};

/// Beat detection mode (inspired by MilkDrop3).
#[derive(Debug, Clone, PartialEq)]
pub enum BeatDetectionMode {
    /// No beat detection
    Off,
    
    /// HardCut1: Load new preset if bass > 1.5 with minimum delay of 0.2s
    HardCut1,
    
    /// HardCut2: Load new preset if treb > 2.9 with minimum delay of 0.5s
    HardCut2,
    
    /// HardCut3: Load new preset if treb > 2.9 with minimum delay of 1s
    HardCut3,
    
    /// HardCut4: Load new preset if treb > 2.9 with minimum delay of 3s,
    /// or immediately if treb > 8
    HardCut4,
    
    /// HardCut5: Load new preset if treb > 2.9 with minimum delay of 5s
    HardCut5,
    
    /// HardCut6: Load new preset if bass > 1.5,
    /// and load special preset if bass > 4.90
    HardCut6 { special_preset: String },
}

impl BeatDetectionMode {
    /// Get the mode name.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::HardCut1 => "HardCut1",
            Self::HardCut2 => "HardCut2",
            Self::HardCut3 => "HardCut3",
            Self::HardCut4 => "HardCut4",
            Self::HardCut5 => "HardCut5",
            Self::HardCut6 { .. } => "HardCut6",
        }
    }
    
    /// Get the next mode (for cycling through modes).
    pub fn next(&self) -> Self {
        match self {
            Self::Off => Self::HardCut1,
            Self::HardCut1 => Self::HardCut2,
            Self::HardCut2 => Self::HardCut3,
            Self::HardCut3 => Self::HardCut4,
            Self::HardCut4 => Self::HardCut5,
            Self::HardCut5 => Self::HardCut6 {
                special_preset: "Bass/WHITE.milk".to_string(),
            },
            Self::HardCut6 { .. } => Self::Off,
        }
    }
}

/// Type of preset change to trigger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetChange {
    /// Load a random preset
    Random,
    
    /// Load a specific preset
    Specific(String),
}

/// Beat detector for automatic preset changing.
#[derive(Debug, Clone)]
pub struct BeatDetector {
    /// Current detection mode
    mode: BeatDetectionMode,
    
    /// Last time a preset change was triggered
    last_trigger: Option<Instant>,
    
    /// Enable/disable detection
    enabled: bool,
}

impl BeatDetector {
    /// Create a new beat detector.
    pub fn new() -> Self {
        Self {
            mode: BeatDetectionMode::Off,
            last_trigger: None,
            enabled: false,
        }
    }
    
    /// Create a beat detector with specific mode.
    pub fn with_mode(mode: BeatDetectionMode) -> Self {
        let enabled = mode != BeatDetectionMode::Off;
        Self {
            mode,
            last_trigger: None,
            enabled,
        }
    }
    
    /// Set the detection mode.
    pub fn set_mode(&mut self, mode: BeatDetectionMode) {
        self.enabled = mode != BeatDetectionMode::Off;
        self.mode = mode;
    }
    
    /// Get the current mode.
    pub fn mode(&self) -> &BeatDetectionMode {
        &self.mode
    }
    
    /// Cycle to the next mode.
    pub fn next_mode(&mut self) {
        self.mode = self.mode.next();
        self.enabled = self.mode != BeatDetectionMode::Off;
    }
    
    /// Enable detection.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable detection.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Check if detection is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Check if a preset change should be triggered based on audio levels.
    pub fn should_change_preset(&mut self, bass: f32, mid: f32, treb: f32) -> Option<PresetChange> {
        if !self.enabled || self.mode == BeatDetectionMode::Off {
            return None;
        }
        
        let now = Instant::now();
        
        // Check if minimum delay has passed
        let can_trigger = match self.last_trigger {
            None => true,
            Some(last) => {
                let min_delay = self.get_min_delay();
                now.duration_since(last) >= min_delay
            }
        };
        
        // Check conditions based on mode
        let change = match self.mode {
            BeatDetectionMode::Off => None,
            
            BeatDetectionMode::HardCut1 => {
                if bass > 1.5 && can_trigger {
                    Some(PresetChange::Random)
                } else {
                    None
                }
            }
            
            BeatDetectionMode::HardCut2 => {
                if treb > 2.9 && can_trigger {
                    Some(PresetChange::Random)
                } else {
                    None
                }
            }
            
            BeatDetectionMode::HardCut3 => {
                if treb > 2.9 && can_trigger {
                    Some(PresetChange::Random)
                } else {
                    None
                }
            }
            
            BeatDetectionMode::HardCut4 => {
                if treb > 8.0 {
                    // Immediate trigger on very high treble
                    Some(PresetChange::Random)
                } else if treb > 2.9 && can_trigger {
                    Some(PresetChange::Random)
                } else {
                    None
                }
            }
            
            BeatDetectionMode::HardCut5 => {
                if treb > 2.9 && can_trigger {
                    Some(PresetChange::Random)
                } else {
                    None
                }
            }
            
            BeatDetectionMode::HardCut6 { ref special_preset } => {
                if bass > 4.90 {
                    // Load special preset on very high bass
                    Some(PresetChange::Specific(special_preset.clone()))
                } else if bass > 1.5 && can_trigger {
                    Some(PresetChange::Random)
                } else {
                    None
                }
            }
        };
        
        // Update last trigger time if change was triggered
        if change.is_some() {
            self.last_trigger = Some(now);
        }
        
        change
    }
    
    /// Get the minimum delay for the current mode.
    fn get_min_delay(&self) -> Duration {
        match self.mode {
            BeatDetectionMode::Off => Duration::from_secs(0),
            BeatDetectionMode::HardCut1 => Duration::from_millis(200),
            BeatDetectionMode::HardCut2 => Duration::from_millis(500),
            BeatDetectionMode::HardCut3 => Duration::from_secs(1),
            BeatDetectionMode::HardCut4 => Duration::from_secs(3),
            BeatDetectionMode::HardCut5 => Duration::from_secs(5),
            BeatDetectionMode::HardCut6 { .. } => Duration::from_millis(200),
        }
    }
}

impl Default for BeatDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_beat_detector_off() {
        let mut detector = BeatDetector::new();
        
        assert_eq!(*detector.mode(), BeatDetectionMode::Off);
        assert!(!detector.is_enabled());
        
        let change = detector.should_change_preset(2.0, 1.0, 3.0);
        assert_eq!(change, None);
    }

    #[test]
    fn test_hardcut1_bass_threshold() {
        let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut1);
        
        // Below threshold
        let change = detector.should_change_preset(1.0, 0.5, 0.5);
        assert_eq!(change, None);
        
        // Above threshold
        let change = detector.should_change_preset(2.0, 0.5, 0.5);
        assert_eq!(change, Some(PresetChange::Random));
    }

    #[test]
    fn test_hardcut2_treb_threshold() {
        let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut2);
        
        // Below threshold
        let change = detector.should_change_preset(0.5, 0.5, 2.0);
        assert_eq!(change, None);
        
        // Above threshold
        let change = detector.should_change_preset(0.5, 0.5, 3.5);
        assert_eq!(change, Some(PresetChange::Random));
    }

    #[test]
    fn test_hardcut4_immediate_trigger() {
        let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut4);
        
        // Trigger once
        let change = detector.should_change_preset(0.5, 0.5, 3.0);
        assert_eq!(change, Some(PresetChange::Random));
        
        // Should not trigger again immediately (min delay not passed)
        let change = detector.should_change_preset(0.5, 0.5, 3.0);
        assert_eq!(change, None);
        
        // But should trigger immediately on very high treble
        let change = detector.should_change_preset(0.5, 0.5, 9.0);
        assert_eq!(change, Some(PresetChange::Random));
    }

    #[test]
    fn test_hardcut6_special_preset() {
        let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut6 {
            special_preset: "Bass/WHITE.milk".to_string(),
        });
        
        // Normal bass
        let change = detector.should_change_preset(2.0, 0.5, 0.5);
        assert_eq!(change, Some(PresetChange::Random));
        
        // Very high bass - should load special preset
        let change = detector.should_change_preset(5.0, 0.5, 0.5);
        assert_eq!(change, Some(PresetChange::Specific("Bass/WHITE.milk".to_string())));
    }

    #[test]
    fn test_min_delay() {
        let mut detector = BeatDetector::with_mode(BeatDetectionMode::HardCut1);
        
        // First trigger
        let change = detector.should_change_preset(2.0, 0.5, 0.5);
        assert_eq!(change, Some(PresetChange::Random));
        
        // Immediate second trigger - should be blocked
        let change = detector.should_change_preset(2.0, 0.5, 0.5);
        assert_eq!(change, None);
        
        // Wait for min delay
        thread::sleep(Duration::from_millis(250));
        
        // Should trigger again
        let change = detector.should_change_preset(2.0, 0.5, 0.5);
        assert_eq!(change, Some(PresetChange::Random));
    }

    #[test]
    fn test_mode_cycling() {
        let mut detector = BeatDetector::new();
        
        assert_eq!(*detector.mode(), BeatDetectionMode::Off);
        
        detector.next_mode();
        assert_eq!(*detector.mode(), BeatDetectionMode::HardCut1);
        
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
        
        detector.next_mode();
        assert_eq!(*detector.mode(), BeatDetectionMode::Off);
    }
}
