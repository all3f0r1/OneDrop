//! Error types for the onedrop-eval crate.

use std::fmt;

/// Result type alias for onedrop-eval operations.
pub type Result<T> = std::result::Result<T, EvalError>;

/// Errors that can occur during expression evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    /// Syntax error in expression
    SyntaxError {
        expression: String,
        reason: String,
    },
    
    /// Undefined variable
    UndefinedVariable(String),
    
    /// Undefined function
    UndefinedFunction(String),
    
    /// Type mismatch
    TypeError {
        expected: String,
        got: String,
    },
    
    /// Division by zero
    DivisionByZero,
    
    /// Generic evaluation error
    EvalFailed(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::SyntaxError { expression, reason } => {
                write!(f, "Syntax error in '{}': {}", expression, reason)
            }
            EvalError::UndefinedVariable(var) => {
                write!(f, "Undefined variable: {}", var)
            }
            EvalError::UndefinedFunction(func) => {
                write!(f, "Undefined function: {}", func)
            }
            EvalError::TypeError { expected, got } => {
                write!(f, "Type error: expected {}, got {}", expected, got)
            }
            EvalError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            EvalError::EvalFailed(msg) => {
                write!(f, "Evaluation failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for EvalError {}

impl From<evalexpr::EvalexprError> for EvalError {
    fn from(err: evalexpr::EvalexprError) -> Self {
        EvalError::EvalFailed(err.to_string())
    }
}
