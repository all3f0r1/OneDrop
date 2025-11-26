//! Preset management and transitions.

use crate::error::{EngineError, Result};
use onedrop_parser::MilkPreset;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};

/// Preset manager handling loading and transitions.
pub struct PresetManager {
    /// Queue of presets to load
    preset_queue: VecDeque<PathBuf>,
    
    /// Current preset index
    current_index: usize,
    
    /// Transition state
    transition: TransitionState,
}

/// Transition state between presets.
#[derive(Debug, Clone)]
pub enum TransitionState {
    /// No transition in progress
    None,
    
    /// Transitioning between presets
    Transitioning {
        /// Progress (0.0 to 1.0)
        progress: f32,
        
        /// Duration in seconds
        duration: f32,
        
        /// Elapsed time
        elapsed: f32,
    },
}

impl PresetManager {
    /// Create a new preset manager.
    pub fn new() -> Self {
        Self {
            preset_queue: VecDeque::new(),
            current_index: 0,
            transition: TransitionState::None,
        }
    }
    
    /// Add a preset to the queue.
    pub fn add_preset<P: AsRef<Path>>(&mut self, path: P) {
        self.preset_queue.push_back(path.as_ref().to_path_buf());
    }
    
    /// Add multiple presets to the queue.
    pub fn add_presets<P: AsRef<Path>>(&mut self, paths: &[P]) {
        for path in paths {
            self.add_preset(path);
        }
    }
    
    /// Get the next preset path.
    pub fn next_preset(&mut self) -> Option<&Path> {
        if self.preset_queue.is_empty() {
            return None;
        }
        
        self.current_index = (self.current_index + 1) % self.preset_queue.len();
        self.preset_queue.get(self.current_index).map(|p| p.as_path())
    }
    
    /// Get the previous preset path.
    pub fn prev_preset(&mut self) -> Option<&Path> {
        if self.preset_queue.is_empty() {
            return None;
        }
        
        if self.current_index == 0 {
            self.current_index = self.preset_queue.len() - 1;
        } else {
            self.current_index -= 1;
        }
        
        self.preset_queue.get(self.current_index).map(|p| p.as_path())
    }
    
    /// Get the current preset path.
    pub fn current_preset(&self) -> Option<&Path> {
        self.preset_queue.get(self.current_index).map(|p| p.as_path())
    }
    
    /// Get a random preset path.
    pub fn random_preset(&mut self) -> Option<&Path> {
        if self.preset_queue.is_empty() {
            return None;
        }
        
        // Use system time for randomness
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;
        
        self.current_index = seed % self.preset_queue.len();
        self.preset_queue.get(self.current_index).map(|p| p.as_path())
    }
    
    /// Start a transition to the next preset.
    pub fn start_transition(&mut self, duration: f32) {
        self.transition = TransitionState::Transitioning {
            progress: 0.0,
            duration,
            elapsed: 0.0,
        };
    }
    
    /// Update transition state.
    pub fn update_transition(&mut self, delta_time: f32) -> bool {
        match &mut self.transition {
            TransitionState::Transitioning { progress, duration, elapsed } => {
                *elapsed += delta_time;
                *progress = (*elapsed / *duration).min(1.0);
                
                if *progress >= 1.0 {
                    self.transition = TransitionState::None;
                    true // Transition complete
                } else {
                    false
                }
            }
            TransitionState::None => true,
        }
    }
    
    /// Get transition progress (0.0 to 1.0).
    pub fn transition_progress(&self) -> f32 {
        match &self.transition {
            TransitionState::Transitioning { progress, .. } => *progress,
            TransitionState::None => 1.0,
        }
    }
    
    /// Check if transitioning.
    pub fn is_transitioning(&self) -> bool {
        matches!(self.transition, TransitionState::Transitioning { .. })
    }
    
    /// Clear all presets.
    pub fn clear(&mut self) {
        self.preset_queue.clear();
        self.current_index = 0;
        self.transition = TransitionState::None;
    }
    
    /// Get number of presets in queue.
    pub fn preset_count(&self) -> usize {
        self.preset_queue.len()
    }
    
    /// Shuffle presets.
    pub fn shuffle(&mut self) {
        use std::collections::HashSet;
        use std::time::{SystemTime, UNIX_EPOCH};
        
        if self.preset_queue.len() <= 1 {
            return;
        }
        
        // Simple shuffle using system time as seed
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let mut new_queue = VecDeque::new();
        let mut indices: Vec<usize> = (0..self.preset_queue.len()).collect();
        
        // Fisher-Yates shuffle
        for i in (1..indices.len()).rev() {
            let j = (seed + i) % (i + 1);
            indices.swap(i, j);
        }
        
        for idx in indices {
            if let Some(preset) = self.preset_queue.get(idx) {
                new_queue.push_back(preset.clone());
            }
        }
        
        self.preset_queue = new_queue;
        self.current_index = 0;
    }
}

impl Default for PresetManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_manager_creation() {
        let manager = PresetManager::new();
        assert_eq!(manager.preset_count(), 0);
    }

    #[test]
    fn test_add_presets() {
        let mut manager = PresetManager::new();
        
        manager.add_preset("preset1.milk");
        manager.add_preset("preset2.milk");
        manager.add_preset("preset3.milk");
        
        assert_eq!(manager.preset_count(), 3);
    }

    #[test]
    fn test_navigation() {
        let mut manager = PresetManager::new();
        
        manager.add_preset("preset1.milk");
        manager.add_preset("preset2.milk");
        manager.add_preset("preset3.milk");
        
        // Current should be first
        assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset1.milk");
        
        // Next
        manager.next_preset();
        assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset2.milk");
        
        // Next again
        manager.next_preset();
        assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset3.milk");
        
        // Wrap around
        manager.next_preset();
        assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset1.milk");
        
        // Previous
        manager.prev_preset();
        assert_eq!(manager.current_preset().unwrap().to_str().unwrap(), "preset3.milk");
    }

    #[test]
    fn test_transition() {
        let mut manager = PresetManager::new();
        
        manager.start_transition(1.0);
        assert!(manager.is_transitioning());
        assert_eq!(manager.transition_progress(), 0.0);
        
        // Update halfway
        manager.update_transition(0.5);
        assert!(manager.is_transitioning());
        assert!((manager.transition_progress() - 0.5).abs() < 0.01);
        
        // Complete transition
        manager.update_transition(0.5);
        assert!(!manager.is_transitioning());
        assert_eq!(manager.transition_progress(), 1.0);
    }

    #[test]
    fn test_shuffle() {
        let mut manager = PresetManager::new();
        
        for i in 0..10 {
            manager.add_preset(format!("preset{}.milk", i));
        }
        
        let original_first = manager.current_preset().unwrap().to_path_buf();
        
        manager.shuffle();
        
        // After shuffle, should still have same count
        assert_eq!(manager.preset_count(), 10);
        
        // First preset might be different (not guaranteed, but likely)
        // Just check that we can still navigate
        assert!(manager.current_preset().is_some());
    }
}
