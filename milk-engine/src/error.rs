//! Error types for the milk-engine crate.

use std::fmt;

/// Result type alias for milk-engine operations.
pub type Result<T> = std::result::Result<T, EngineError>;

/// Errors that can occur in the engine.
#[derive(Debug)]
pub enum EngineError {
    /// Preset loading failed
    PresetLoadFailed(String),
    
    /// Preset parsing failed
    PresetParseFailed(milk_parser::ParseError),
    
    /// Expression evaluation failed
    EvalFailed(milk_eval::EvalError),
    
    /// Rendering failed
    RenderFailed(milk_renderer::RenderError),
    
    /// No preset loaded
    NoPresetLoaded,
    
    /// Invalid audio data
    InvalidAudioData(String),
    
    /// Generic error
    Other(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::PresetLoadFailed(msg) => {
                write!(f, "Failed to load preset: {}", msg)
            }
            EngineError::PresetParseFailed(err) => {
                write!(f, "Failed to parse preset: {}", err)
            }
            EngineError::EvalFailed(err) => {
                write!(f, "Expression evaluation failed: {}", err)
            }
            EngineError::RenderFailed(err) => {
                write!(f, "Rendering failed: {}", err)
            }
            EngineError::NoPresetLoaded => {
                write!(f, "No preset loaded")
            }
            EngineError::InvalidAudioData(msg) => {
                write!(f, "Invalid audio data: {}", msg)
            }
            EngineError::Other(msg) => {
                write!(f, "Error: {}", msg)
            }
        }
    }
}

impl std::error::Error for EngineError {}

impl From<milk_parser::ParseError> for EngineError {
    fn from(err: milk_parser::ParseError) -> Self {
        EngineError::PresetParseFailed(err)
    }
}

impl From<milk_eval::EvalError> for EngineError {
    fn from(err: milk_eval::EvalError) -> Self {
        EngineError::EvalFailed(err)
    }
}

impl From<milk_renderer::RenderError> for EngineError {
    fn from(err: milk_renderer::RenderError) -> Self {
        EngineError::RenderFailed(err)
    }
}

impl From<std::io::Error> for EngineError {
    fn from(err: std::io::Error) -> Self {
        EngineError::PresetLoadFailed(err.to_string())
    }
}
