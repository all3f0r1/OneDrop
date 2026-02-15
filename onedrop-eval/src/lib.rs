//! # onedrop-eval
//!
//! Expression evaluator for Milkdrop per-frame and per-pixel equations.
//!
//! This crate provides functionality to evaluate mathematical expressions
//! used in Milkdrop presets, with support for all Milkdrop variables and functions.

pub mod cache;
pub mod context;
pub mod error;
pub mod evaluator;
pub mod evaluator_optimized;
pub mod math_functions;

pub use cache::{CacheStats, ExpressionCache};
pub use context::MilkContext;
pub use error::{EvalError, Result};
pub use evaluator::MilkEvaluator;
pub use evaluator_optimized::OptimizedEvaluator;
pub use math_functions::{list_math_functions, register_math_functions};

/// Evaluate a simple expression with default context.
///
/// # Examples
///
/// ```
/// use onedrop_eval::eval_simple;
///
/// let result = eval_simple("2 + 2").unwrap();
/// assert_eq!(result, 4.0);
/// ```
pub fn eval_simple(expression: &str) -> Result<f64> {
    let mut evaluator = MilkEvaluator::new();
    evaluator.eval(expression)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple() {
        let result = eval_simple("10 * 5").unwrap();
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_eval_with_math() {
        let result = eval_simple("sin(0) + cos(0)").unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }
}
