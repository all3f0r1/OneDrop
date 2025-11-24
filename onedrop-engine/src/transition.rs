//! Preset transition system with blending.

use std::time::{Duration, Instant};

/// Transition mode between presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionMode {
    /// Instant cut (no transition)
    Cut,
    /// Linear fade
    Fade,
    /// Smooth ease-in-out
    EaseInOut,
    /// Crossfade with overlap
    Crossfade,
}

/// Transition state.
#[derive(Debug, Clone)]
pub struct Transition {
    /// Transition mode
    mode: TransitionMode,
    
    /// Transition duration
    duration: Duration,
    
    /// Start time
    start_time: Instant,
    
    /// Current progress (0.0 to 1.0)
    progress: f32,
    
    /// Is transition active
    active: bool,
}

impl Transition {
    /// Create a new transition.
    pub fn new(mode: TransitionMode, duration: Duration) -> Self {
        Self {
            mode,
            duration,
            start_time: Instant::now(),
            progress: 0.0,
            active: false,
        }
    }
    
    /// Start the transition.
    pub fn start(&mut self) {
        self.start_time = Instant::now();
        self.progress = 0.0;
        self.active = true;
    }
    
    /// Update transition progress.
    pub fn update(&mut self) {
        if !self.active {
            return;
        }
        
        let elapsed = self.start_time.elapsed();
        let t = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        
        if t >= 1.0 {
            self.progress = 1.0;
            self.active = false;
        } else {
            self.progress = match self.mode {
                TransitionMode::Cut => 1.0,
                TransitionMode::Fade => t,
                TransitionMode::EaseInOut => Self::ease_in_out(t),
                TransitionMode::Crossfade => t,
            };
        }
    }
    
    /// Get current progress (0.0 to 1.0).
    pub fn progress(&self) -> f32 {
        self.progress
    }
    
    /// Check if transition is complete.
    pub fn is_complete(&self) -> bool {
        !self.active
    }
    
    /// Get blend factor for old preset (1.0 to 0.0).
    pub fn old_blend(&self) -> f32 {
        1.0 - self.progress
    }
    
    /// Get blend factor for new preset (0.0 to 1.0).
    pub fn new_blend(&self) -> f32 {
        self.progress
    }
    
    /// Ease-in-out function (smooth S-curve).
    fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - 2.0 * (1.0 - t) * (1.0 - t)
        }
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new(TransitionMode::Fade, Duration::from_secs(2))
    }
}

/// Preset transition manager.
pub struct TransitionManager {
    /// Current transition
    transition: Option<Transition>,
    
    /// Default transition mode
    default_mode: TransitionMode,
    
    /// Default transition duration
    default_duration: Duration,
}

impl TransitionManager {
    /// Create a new transition manager.
    pub fn new(mode: TransitionMode, duration: Duration) -> Self {
        Self {
            transition: None,
            default_mode: mode,
            default_duration: duration,
        }
    }
    
    /// Start a new transition.
    pub fn start_transition(&mut self) {
        let mut transition = Transition::new(self.default_mode, self.default_duration);
        transition.start();
        self.transition = Some(transition);
    }
    
    /// Start a transition with custom parameters.
    pub fn start_custom_transition(&mut self, mode: TransitionMode, duration: Duration) {
        let mut transition = Transition::new(mode, duration);
        transition.start();
        self.transition = Some(transition);
    }
    
    /// Update the current transition.
    pub fn update(&mut self) {
        if let Some(ref mut transition) = self.transition {
            transition.update();
            
            if transition.is_complete() {
                self.transition = None;
            }
        }
    }
    
    /// Check if a transition is active.
    pub fn is_transitioning(&self) -> bool {
        self.transition.is_some()
    }
    
    /// Get current transition progress.
    pub fn progress(&self) -> f32 {
        self.transition.as_ref()
            .map(|t| t.progress())
            .unwrap_or(1.0)
    }
    
    /// Get blend factors (old, new).
    pub fn blend_factors(&self) -> (f32, f32) {
        if let Some(ref transition) = self.transition {
            (transition.old_blend(), transition.new_blend())
        } else {
            (0.0, 1.0) // Fully on new preset
        }
    }
}

impl Default for TransitionManager {
    fn default() -> Self {
        Self::new(TransitionMode::Fade, Duration::from_secs(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_transition_progress() {
        let mut transition = Transition::new(TransitionMode::Fade, Duration::from_millis(100));
        transition.start();
        
        assert_eq!(transition.progress(), 0.0);
        assert!(!transition.is_complete());
        
        thread::sleep(Duration::from_millis(50));
        transition.update();
        
        assert!(transition.progress() > 0.0 && transition.progress() < 1.0);
        
        thread::sleep(Duration::from_millis(60));
        transition.update();
        
        assert_eq!(transition.progress(), 1.0);
        assert!(transition.is_complete());
    }

    #[test]
    fn test_ease_in_out() {
        assert_eq!(Transition::ease_in_out(0.0), 0.0);
        assert_eq!(Transition::ease_in_out(1.0), 1.0);
        
        let mid = Transition::ease_in_out(0.5);
        assert!(mid > 0.4 && mid < 0.6);
    }

    #[test]
    fn test_blend_factors() {
        let mut transition = Transition::new(TransitionMode::Fade, Duration::from_secs(1));
        transition.start();
        
        // At start
        assert_eq!(transition.old_blend(), 1.0);
        assert_eq!(transition.new_blend(), 0.0);
        
        // Simulate progress
        transition.progress = 0.5;
        assert_eq!(transition.old_blend(), 0.5);
        assert_eq!(transition.new_blend(), 0.5);
    }

    #[test]
    fn test_transition_manager() {
        let mut manager = TransitionManager::default();
        
        assert!(!manager.is_transitioning());
        
        manager.start_transition();
        assert!(manager.is_transitioning());
        
        thread::sleep(Duration::from_millis(100));
        manager.update();
        
        assert!(manager.progress() > 0.0);
    }
}
