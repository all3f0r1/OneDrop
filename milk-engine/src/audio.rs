//! Audio processing and analysis.

use milk_renderer::AudioLevels;

/// Audio analyzer for extracting frequency bands.
pub struct AudioAnalyzer {
    /// Sample rate
    sample_rate: f32,
    
    /// Bass attenuated value
    bass_att: f32,
    
    /// Mid attenuated value
    mid_att: f32,
    
    /// Treble attenuated value
    treb_att: f32,
    
    /// Attenuation factor (0-1)
    attenuation: f32,
}

impl AudioAnalyzer {
    /// Create a new audio analyzer.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            bass_att: 0.0,
            mid_att: 0.0,
            treb_att: 0.0,
            attenuation: 0.8, // Default attenuation
        }
    }
    
    /// Analyze audio samples and extract frequency bands.
    pub fn analyze(&mut self, samples: &[f32]) -> AudioLevels {
        // Simple frequency band extraction
        // In a real implementation, this would use FFT
        
        let bass = Self::extract_band(samples, 0, samples.len() / 4);
        let mid = Self::extract_band(samples, samples.len() / 4, samples.len() / 2);
        let treb = Self::extract_band(samples, samples.len() / 2, samples.len());
        
        // Update attenuated values with smoothing
        self.bass_att = self.bass_att * self.attenuation + bass * (1.0 - self.attenuation);
        self.mid_att = self.mid_att * self.attenuation + mid * (1.0 - self.attenuation);
        self.treb_att = self.treb_att * self.attenuation + treb * (1.0 - self.attenuation);
        
        AudioLevels {
            bass,
            mid,
            treb,
            bass_att: self.bass_att,
            mid_att: self.mid_att,
            treb_att: self.treb_att,
        }
    }
    
    /// Extract a frequency band from samples.
    fn extract_band(samples: &[f32], start: usize, end: usize) -> f32 {
        if samples.is_empty() || start >= end {
            return 0.0;
        }
        
        let range = &samples[start.min(samples.len())..end.min(samples.len())];
        if range.is_empty() {
            return 0.0;
        }
        
        // Calculate RMS (Root Mean Square)
        let sum_squares: f32 = range.iter().map(|&x| x * x).sum();
        let rms = (sum_squares / range.len() as f32).sqrt();
        
        // Normalize to 0-1 range (assuming input is -1 to 1)
        rms.min(1.0)
    }
    
    /// Set attenuation factor.
    pub fn set_attenuation(&mut self, attenuation: f32) {
        self.attenuation = attenuation.clamp(0.0, 1.0);
    }
    
    /// Reset attenuated values.
    pub fn reset(&mut self) {
        self.bass_att = 0.0;
        self.mid_att = 0.0;
        self.treb_att = 0.0;
    }
}

impl Default for AudioAnalyzer {
    fn default() -> Self {
        Self::new(44100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_audio_analyzer_creation() {
        let analyzer = AudioAnalyzer::new(44100.0);
        assert_eq!(analyzer.sample_rate, 44100.0);
    }

    #[test]
    fn test_analyze_silence() {
        let mut analyzer = AudioAnalyzer::new(44100.0);
        let samples = vec![0.0; 1024];
        
        let levels = analyzer.analyze(&samples);
        
        assert_relative_eq!(levels.bass, 0.0, epsilon = 0.01);
        assert_relative_eq!(levels.mid, 0.0, epsilon = 0.01);
        assert_relative_eq!(levels.treb, 0.0, epsilon = 0.01);
    }

    #[test]
    fn test_analyze_signal() {
        let mut analyzer = AudioAnalyzer::new(44100.0);
        
        // Generate a simple sine wave
        let samples: Vec<f32> = (0..1024)
            .map(|i| (i as f32 * 0.1).sin() * 0.5)
            .collect();
        
        let levels = analyzer.analyze(&samples);
        
        // Should have some signal
        assert!(levels.bass > 0.0);
        assert!(levels.mid > 0.0);
        assert!(levels.treb > 0.0);
    }

    #[test]
    fn test_attenuation() {
        let mut analyzer = AudioAnalyzer::new(44100.0);
        analyzer.set_attenuation(0.5);
        
        let samples: Vec<f32> = (0..1024)
            .map(|i| (i as f32 * 0.1).sin())
            .collect();
        
        // First analysis
        let levels1 = analyzer.analyze(&samples);
        
        // Second analysis (should be attenuated)
        let levels2 = analyzer.analyze(&samples);
        
        // Attenuated values should be smoothed
        assert!(levels2.bass_att > 0.0);
    }
}
