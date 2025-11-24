//! FFT-based audio analysis for advanced frequency detection.

use std::f32::consts::PI;

/// FFT analyzer for audio frequency analysis.
pub struct FFTAnalyzer {
    /// FFT size (power of 2)
    fft_size: usize,
    
    /// Window function (Hann window)
    window: Vec<f32>,
    
    /// FFT buffer (real and imaginary parts)
    fft_buffer: Vec<f32>,
    
    /// Frequency bins
    bins: Vec<f32>,
    
    /// Sample rate
    sample_rate: f32,
}

impl FFTAnalyzer {
    /// Create a new FFT analyzer.
    pub fn new(fft_size: usize, sample_rate: f32) -> Self {
        assert!(fft_size.is_power_of_two(), "FFT size must be power of 2");
        
        // Create Hann window
        let window: Vec<f32> = (0..fft_size)
            .map(|i| {
                let x = i as f32 / fft_size as f32;
                0.5 * (1.0 - (2.0 * PI * x).cos())
            })
            .collect();
        
        Self {
            fft_size,
            window,
            fft_buffer: vec![0.0; fft_size * 2], // Real + imaginary
            bins: vec![0.0; fft_size / 2],
            sample_rate,
        }
    }
    
    /// Analyze audio samples and return frequency bins.
    pub fn analyze(&mut self, samples: &[f32]) -> &[f32] {
        // Ensure we have enough samples
        let num_samples = samples.len().min(self.fft_size);
        
        // Apply window and copy to FFT buffer
        for i in 0..num_samples {
            self.fft_buffer[i * 2] = samples[i] * self.window[i]; // Real
            self.fft_buffer[i * 2 + 1] = 0.0; // Imaginary
        }
        
        // Zero-pad if necessary
        for i in num_samples..self.fft_size {
            self.fft_buffer[i * 2] = 0.0;
            self.fft_buffer[i * 2 + 1] = 0.0;
        }
        
        // Perform FFT (simple implementation)
        self.fft_inplace();
        
        // Calculate magnitude spectrum
        for i in 0..self.bins.len() {
            let real = self.fft_buffer[i * 2];
            let imag = self.fft_buffer[i * 2 + 1];
            self.bins[i] = (real * real + imag * imag).sqrt() / self.fft_size as f32;
        }
        
        &self.bins
    }
    
    /// Get bass level (20-250 Hz).
    pub fn get_bass(&self) -> f32 {
        self.get_frequency_range(20.0, 250.0)
    }
    
    /// Get mid level (250-2000 Hz).
    pub fn get_mid(&self) -> f32 {
        self.get_frequency_range(250.0, 2000.0)
    }
    
    /// Get treble level (2000-20000 Hz).
    pub fn get_treble(&self) -> f32 {
        self.get_frequency_range(2000.0, 20000.0)
    }
    
    /// Get energy in a frequency range.
    fn get_frequency_range(&self, min_freq: f32, max_freq: f32) -> f32 {
        let bin_width = self.sample_rate / self.fft_size as f32;
        let min_bin = (min_freq / bin_width) as usize;
        let max_bin = ((max_freq / bin_width) as usize).min(self.bins.len());
        
        if min_bin >= max_bin {
            return 0.0;
        }
        
        let sum: f32 = self.bins[min_bin..max_bin].iter().sum();
        sum / (max_bin - min_bin) as f32
    }
    
    /// Simple in-place FFT (Cooley-Tukey algorithm).
    fn fft_inplace(&mut self) {
        let n = self.fft_size;
        
        // Bit-reversal permutation
        let mut j = 0;
        for i in 0..n {
            if i < j {
                self.fft_buffer.swap(i * 2, j * 2);
                self.fft_buffer.swap(i * 2 + 1, j * 2 + 1);
            }
            
            let mut m = n / 2;
            while m >= 1 && j >= m {
                j -= m;
                m /= 2;
            }
            j += m;
        }
        
        // FFT computation
        let mut len = 2;
        while len <= n {
            let angle = -2.0 * PI / len as f32;
            let wlen_real = angle.cos();
            let wlen_imag = angle.sin();
            
            let mut i = 0;
            while i < n {
                let mut w_real = 1.0;
                let mut w_imag = 0.0;
                
                for j in 0..len / 2 {
                    let u_idx = (i + j) * 2;
                    let v_idx = (i + j + len / 2) * 2;
                    
                    let u_real = self.fft_buffer[u_idx];
                    let u_imag = self.fft_buffer[u_idx + 1];
                    let v_real = self.fft_buffer[v_idx];
                    let v_imag = self.fft_buffer[v_idx + 1];
                    
                    let t_real = w_real * v_real - w_imag * v_imag;
                    let t_imag = w_real * v_imag + w_imag * v_real;
                    
                    self.fft_buffer[u_idx] = u_real + t_real;
                    self.fft_buffer[u_idx + 1] = u_imag + t_imag;
                    self.fft_buffer[v_idx] = u_real - t_real;
                    self.fft_buffer[v_idx + 1] = u_imag - t_imag;
                    
                    let w_temp = w_real;
                    w_real = w_real * wlen_real - w_imag * wlen_imag;
                    w_imag = w_temp * wlen_imag + w_imag * wlen_real;
                }
                
                i += len;
            }
            
            len *= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_analyzer() {
        let mut analyzer = FFTAnalyzer::new(256, 44100.0);
        
        // Generate a simple sine wave at 440 Hz (A4)
        let samples: Vec<f32> = (0..256)
            .map(|i| {
                let t = i as f32 / 44100.0;
                (2.0 * PI * 440.0 * t).sin()
            })
            .collect();
        
        let bins = analyzer.analyze(&samples);
        
        // Check that we got some output
        assert!(bins.len() > 0);
        
        // The peak should be around 440 Hz
        let peak_bin = bins.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        
        let peak_freq = peak_bin as f32 * 44100.0 / 256.0;
        
        // Should be close to 440 Hz (within 200 Hz tolerance)
        assert!((peak_freq - 440.0).abs() < 200.0);
    }

    #[test]
    fn test_frequency_ranges() {
        let mut analyzer = FFTAnalyzer::new(512, 44100.0);
        
        // Generate white noise
        let samples: Vec<f32> = (0..512)
            .map(|i| (i as f32 * 0.1).sin())
            .collect();
        
        analyzer.analyze(&samples);
        
        let bass = analyzer.get_bass();
        let mid = analyzer.get_mid();
        let treble = analyzer.get_treble();
        
        // All should be non-negative
        assert!(bass >= 0.0);
        assert!(mid >= 0.0);
        assert!(treble >= 0.0);
    }
}
