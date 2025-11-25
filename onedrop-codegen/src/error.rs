//! Error types for shader code generation

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("Transpilation error: {0}")]
    Transpilation(String),
    
    #[error("Optimization error: {0}")]
    Optimization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Unsupported expression: {0}")]
    UnsupportedExpression(String),
    
    #[error("Invalid variable: {0}")]
    InvalidVariable(String),
    
    #[error("Shader compilation error: {0}")]
    Compilation(String),
    
    #[error("Naga error: {0}")]
    Naga(#[from] naga::front::wgsl::ParseError),
}

pub type Result<T> = std::result::Result<T, CodegenError>;
