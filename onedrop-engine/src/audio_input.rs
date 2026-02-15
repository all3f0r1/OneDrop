//! Real-time audio input capture using cpal.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Audio input errors.
#[derive(Debug, Error)]
pub enum AudioInputError {
    #[error("No audio input device available")]
    NoDevice,
    
    #[error("Failed to get default input config: {0}")]
    ConfigError(#[from] cpal::DefaultStreamConfigError),
    
    #[error("Failed to build audio stream: {0}")]
    BuildStreamError(#[from] cpal::BuildStreamError),
    
    #[error("Failed to play audio stream: {0}")]
    PlayStreamError(#[from] cpal::PlayStreamError),
}

pub type Result<T> = std::result::Result<T, AudioInputError>;

/// Real-time audio input capture.
pub struct AudioInput {
    /// Audio host
    _host: Host,
    
    /// Input device
    _device: Device,
    
    /// Input stream
    _stream: Stream,
    
    /// Shared buffer for audio samples
    buffer: Arc<Mutex<Vec<f32>>>,
    
    /// Sample rate
    sample_rate: u32,
}

impl AudioInput {
    /// Create a new audio input capture.
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        
        // Get default input device
        let device = host
            .default_input_device()
            .ok_or(AudioInputError::NoDevice)?;
        
        log::info!("Using audio input device: {}", device.name().unwrap_or_else(|_| "Unknown".to_string()));
        
        // Get default input config
        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0;
        
        log::info!("Audio input config: {} Hz, {} channels", 
                   sample_rate, 
                   config.channels());
        
        // Create shared buffer
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = buffer.clone();
        
        // Build input stream
        let stream_config: StreamConfig = config.into();
        let stream = device.build_input_stream(
            &stream_config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Copy audio data to buffer (handle mutex poisoning gracefully)
                if let Ok(mut buf) = buffer_clone.lock() {
                    buf.clear();
                    buf.extend_from_slice(data);
                }
            },
            |err| {
                log::error!("Audio input stream error: {}", err);
            },
            None,
        )?;
        
        // Start the stream
        stream.play()?;
        
        log::info!("Audio input stream started");
        
        Ok(Self {
            _host: host,
            _device: device,
            _stream: stream,
            buffer,
            sample_rate,
        })
    }
    
    /// Get the latest audio samples.
    /// Returns a copy of the current buffer.
    pub fn get_samples(&self) -> Vec<f32> {
        self.buffer.lock()
            .map(|buf| buf.clone())
            .unwrap_or_default()
    }
    
    /// Get the sample rate.
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    
    /// Get a fixed number of samples for processing.
    /// If not enough samples are available, returns zeros.
    pub fn get_fixed_samples(&self, count: usize) -> Vec<f32> {
        self.buffer.lock()
            .map(|buf| {
                if buf.len() >= count {
                    buf[..count].to_vec()
                } else {
                    // Pad with zeros if not enough samples
                    let mut result = buf.clone();
                    result.resize(count, 0.0);
                    result
                }
            })
            .unwrap_or_else(|_| vec![0.0; count])
    }
}

/// Audio input with FFT analysis for bass/mid/treb extraction.
pub struct AudioAnalysisInput {
    /// Audio input
    input: AudioInput,
    
    /// FFT planner
    fft: Arc<dyn rustfft::Fft<f32>>,
    
    /// FFT buffer size
    fft_size: usize,
}

impl AudioAnalysisInput {
    /// Create a new audio analysis input.
    /// 
    /// # Arguments
    /// * `fft_size` - Size of FFT window (power of 2, e.g., 2048)
    pub fn new(fft_size: usize) -> Result<Self> {
        let input = AudioInput::new()?;
        
        // Create FFT planner
        let mut planner = rustfft::FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);
        
        log::info!("Audio analysis initialized with FFT size {}", fft_size);
        
        Ok(Self {
            input,
            fft,
            fft_size,
        })
    }
    
    /// Analyze audio and extract bass, mid, treb levels.
    /// Returns (bass, mid, treb) in range [0.0, 1.0].
    pub fn analyze(&self) -> (f32, f32, f32) {
        use rustfft::num_complex::Complex;
        
        // Get samples
        let samples = self.input.get_fixed_samples(self.fft_size);
        
        // Convert to complex
        let mut buffer: Vec<Complex<f32>> = samples
            .iter()
            .map(|&s| Complex::new(s, 0.0))
            .collect();
        
        // Apply Hann window
        for (i, sample) in buffer.iter_mut().enumerate() {
            let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / self.fft_size as f32).cos());
            *sample *= window;
        }
        
        // Perform FFT
        self.fft.process(&mut buffer);
        
        // Calculate magnitude spectrum
        let magnitudes: Vec<f32> = buffer
            .iter()
            .take(self.fft_size / 2) // Only use first half (Nyquist)
            .map(|c| c.norm())
            .collect();
        
        // Extract bass, mid, treb
        // Frequency bins: bin_freq = sample_rate * bin_index / fft_size
        let sample_rate = self.input.sample_rate() as f32;
        let bin_to_freq = |bin: usize| sample_rate * bin as f32 / self.fft_size as f32;

        // Bass: 20-250 Hz
        // Mid: 250-2000 Hz
        // Treb: 2000-20000 Hz
        let bass_end = (250.0 * self.fft_size as f32 / sample_rate) as usize;
        let mid_end = (2000.0 * self.fft_size as f32 / sample_rate) as usize;

        // Bounds checking to prevent panics
        let bass_end = bass_end.max(1).min(magnitudes.len());
        let mid_end = mid_end.max(bass_end).min(magnitudes.len());

        let bass: f32 = if bass_end > 1 {
            magnitudes[1..bass_end].iter().sum::<f32>() / (bass_end - 1) as f32
        } else {
            0.0
        };
        let mid: f32 = if mid_end > bass_end {
            magnitudes[bass_end..mid_end].iter().sum::<f32>() / (mid_end - bass_end) as f32
        } else {
            0.0
        };
        let treb: f32 = if magnitudes.len() > mid_end {
            magnitudes[mid_end..].iter().sum::<f32>() / (magnitudes.len() - mid_end) as f32
        } else {
            0.0
        };
        
        // Normalize to [0, 1] range (approximate)
        let normalize = |x: f32| (x * 10.0).min(1.0);
        
        (normalize(bass), normalize(mid), normalize(treb))
    }
    
    /// Get the sample rate.
    pub fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }
    
    /// Get raw samples for further processing.
    pub fn get_samples(&self) -> Vec<f32> {
        self.input.get_samples()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore] // Requires audio device
    fn test_audio_input_creation() {
        let input = AudioInput::new();
        assert!(input.is_ok(), "Failed to create audio input");
    }
    
    #[test]
    #[ignore] // Requires audio device
    fn test_audio_analysis_creation() {
        let input = AudioAnalysisInput::new(2048);
        assert!(input.is_ok(), "Failed to create audio analysis input");
    }
    
    #[test]
    #[ignore] // Requires audio device
    fn test_audio_capture() {
        let input = AudioInput::new().unwrap();
        
        // Wait a bit for samples
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        let samples = input.get_samples();
        assert!(!samples.is_empty(), "Should have captured some samples");
    }
    
    #[test]
    #[ignore] // Requires audio device
    fn test_audio_analysis() {
        let input = AudioAnalysisInput::new(2048).unwrap();
        
        // Wait a bit for samples
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        let (bass, mid, treb) = input.analyze();
        
        // Should be in valid range
        assert!(bass >= 0.0 && bass <= 1.0, "Bass out of range: {}", bass);
        assert!(mid >= 0.0 && mid <= 1.0, "Mid out of range: {}", mid);
        assert!(treb >= 0.0 && treb <= 1.0, "Treb out of range: {}", treb);
        
        println!("Audio levels - Bass: {:.3}, Mid: {:.3}, Treb: {:.3}", bass, mid, treb);
    }
}
