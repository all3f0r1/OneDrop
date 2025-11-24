//! Error types for the onedrop-parser crate.

use std::fmt;

/// Result type alias for onedrop-parser operations.
pub type Result<T> = std::result::Result<T, ParseError>;

/// Errors that can occur during preset parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Invalid preset version number
    InvalidVersion(String),
    
    /// Missing required header
    MissingHeader(String),
    
    /// Invalid parameter value
    InvalidParameter {
        name: String,
        value: String,
        reason: String,
    },
    
    /// Invalid equation syntax
    InvalidEquation {
        line: usize,
        equation: String,
        reason: String,
    },
    
    /// Missing required section
    MissingSection(String),
    
    /// Generic parsing error
    ParseFailed(String),
    
    /// IO error
    IoError(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidVersion(v) => {
                write!(f, "Invalid preset version: {}", v)
            }
            ParseError::MissingHeader(h) => {
                write!(f, "Missing required header: {}", h)
            }
            ParseError::InvalidParameter { name, value, reason } => {
                write!(f, "Invalid parameter '{}' with value '{}': {}", name, value, reason)
            }
            ParseError::InvalidEquation { line, equation, reason } => {
                write!(f, "Invalid equation at line {}: '{}' - {}", line, equation, reason)
            }
            ParseError::MissingSection(s) => {
                write!(f, "Missing required section: {}", s)
            }
            ParseError::ParseFailed(msg) => {
                write!(f, "Parse failed: {}", msg)
            }
            ParseError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IoError(err.to_string())
    }
}
