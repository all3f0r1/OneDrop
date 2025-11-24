//! Error types for the onedrop-renderer crate.

use std::fmt;

/// Result type alias for onedrop-renderer operations.
pub type Result<T> = std::result::Result<T, RenderError>;

/// Errors that can occur during rendering.
#[derive(Debug)]
pub enum RenderError {
    /// GPU device creation failed
    DeviceCreationFailed(String),
    
    /// Shader compilation failed
    ShaderCompilationFailed {
        shader_name: String,
        reason: String,
    },
    
    /// Texture creation failed
    TextureCreationFailed(String),
    
    /// Buffer creation failed
    BufferCreationFailed(String),
    
    /// Rendering failed
    RenderFailed(String),
    
    /// Invalid configuration
    InvalidConfiguration(String),
    
    /// Generic error
    Other(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::DeviceCreationFailed(msg) => {
                write!(f, "GPU device creation failed: {}", msg)
            }
            RenderError::ShaderCompilationFailed { shader_name, reason } => {
                write!(f, "Shader '{}' compilation failed: {}", shader_name, reason)
            }
            RenderError::TextureCreationFailed(msg) => {
                write!(f, "Texture creation failed: {}", msg)
            }
            RenderError::BufferCreationFailed(msg) => {
                write!(f, "Buffer creation failed: {}", msg)
            }
            RenderError::RenderFailed(msg) => {
                write!(f, "Rendering failed: {}", msg)
            }
            RenderError::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
            RenderError::Other(msg) => {
                write!(f, "Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for RenderError {}

impl From<wgpu::RequestDeviceError> for RenderError {
    fn from(err: wgpu::RequestDeviceError) -> Self {
        RenderError::DeviceCreationFailed(err.to_string())
    }
}

impl From<wgpu::SurfaceError> for RenderError {
    fn from(err: wgpu::SurfaceError) -> Self {
        RenderError::RenderFailed(err.to_string())
    }
}
