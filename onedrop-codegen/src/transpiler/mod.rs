//! Expression to WGSL transpiler
//!
//! Converts Milkdrop expressions to WGSL shader code.

mod expression;
mod variable;

pub use expression::ExpressionTranspiler;
pub use variable::VariableMapper;

use crate::error::{CodegenError, Result};

/// Transpile a Milkdrop equation to WGSL
pub fn transpile_equation(equation: &str) -> Result<String> {
    let transpiler = ExpressionTranspiler::new();
    transpiler.transpile(equation)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_assignment() {
        let result = transpile_equation("x = 0.5").unwrap();
        assert!(result.contains("x = 0.5"));
    }
    
    #[test]
    fn test_math_expression() {
        let result = transpile_equation("x = x + 0.01*sin(time)").unwrap();
        assert!(result.contains("sin"));
        assert!(result.contains("time"));
    }
}
