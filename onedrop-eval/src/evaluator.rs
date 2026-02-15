//! Evaluator for Milkdrop expressions.

use crate::context::MilkContext;
use crate::error::{EvalError, Result};
use evalexpr::{Node, eval_with_context_mut};
use regex::Regex;
use std::sync::LazyLock;

// Pre-compiled regex patterns for performance
static VAR_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap());

static ASSIGNMENT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\w+)\s*=\s*(-?\d+)([^\d\.]|$)").unwrap());

static IF_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bif\s*\(").unwrap());

/// Maximum expression length to prevent DoS attacks
const MAX_EXPRESSION_LENGTH: usize = 100_000;

/// Evaluator for Milkdrop expressions.
pub struct MilkEvaluator {
    /// Execution context
    context: MilkContext,

    /// Compiled expressions cache
    compiled_cache: Vec<(String, Node)>,
}

impl MilkEvaluator {
    /// Create a new evaluator.
    pub fn new() -> Self {
        Self {
            context: MilkContext::new(),
            compiled_cache: Vec::new(),
        }
    }

    /// Get a reference to the context.
    pub fn context(&self) -> &MilkContext {
        &self.context
    }

    /// Get a mutable reference to the context.
    pub fn context_mut(&mut self) -> &mut MilkContext {
        &mut self.context
    }

    /// Pre-process expression to handle auto-initialization and type conversion.
    fn preprocess_expression(&mut self, expression: &str) -> String {
        let expr = expression.trim();

        // Extract variable names from the expression using pre-compiled regex
        for cap in VAR_REGEX.captures_iter(expr) {
            let var_name = &cap[1];

            // Skip function names and keywords
            if matches!(
                var_name,
                "sin"
                    | "cos"
                    | "tan"
                    | "sqrt"
                    | "abs"
                    | "pow"
                    | "exp"
                    | "log"
                    | "ln"
                    | "if"
                    | "min"
                    | "max"
                    | "floor"
                    | "ceil"
                    | "round"
                    | "rand"
                    | "above"
                    | "below"
                    | "equal"
                    | "bnot"
                    | "band"
                    | "bor"
                    | "int"
                    | "fmod"
                    | "clamp"
                    | "sinh"
                    | "cosh"
                    | "tanh"
                    | "asin"
                    | "acos"
                    | "atan"
                    | "atan2"
                    | "sqr"
                    | "rad"
                    | "deg"
                    | "fract"
                    | "trunc"
                    | "sign"
            ) {
                continue;
            }

            // Auto-initialize undefined variables to 0
            if self.context.get(var_name).is_none() {
                self.context.set(var_name, 0.0);
            }
        }

        // Convert integer literals to floats in assignments
        // e.g., "zoom = 1" -> "zoom = 1.0"
        let mut result = ASSIGNMENT_REGEX
            .replace_all(expr, "$1 = $2.0$3")
            .to_string();

        // Replace if( or if ( with milkif( to use our custom if function
        // This allows Float conditions (0.0 = false, non-zero = true)
        result = IF_REGEX.replace_all(&result, "milkif(").to_string();

        result
    }

    /// Evaluate a single expression.
    pub fn eval(&mut self, expression: &str) -> Result<f64> {
        // Security check: limit expression length to prevent DoS
        if expression.len() > MAX_EXPRESSION_LENGTH {
            return Err(EvalError::SyntaxError {
                expression: expression.chars().take(100).collect(),
                reason: format!(
                    "Expression too long: {} bytes (max {})",
                    expression.len(),
                    MAX_EXPRESSION_LENGTH
                ),
            });
        }

        // Clean the expression (remove trailing semicolon, trim whitespace)
        let expr = expression.trim().trim_end_matches(';').trim();

        if expr.is_empty() {
            return Ok(0.0);
        }

        // Pre-process to handle auto-initialization and type conversion
        let processed_expr = self.preprocess_expression(expr);

        // Evaluate with context
        match eval_with_context_mut(&processed_expr, self.context.inner_mut()) {
            Ok(value) => {
                // Convert result to f64
                match value {
                    evalexpr::Value::Float(f) => Ok(f),
                    evalexpr::Value::Int(i) => Ok(i as f64),
                    evalexpr::Value::Boolean(b) => Ok(if b { 1.0 } else { 0.0 }),
                    evalexpr::Value::Empty => Ok(0.0), // Assignments return Empty
                    _ => Err(EvalError::TypeError {
                        expected: "number".to_string(),
                        got: format!("{:?}", value),
                    }),
                }
            }
            Err(e) => Err(EvalError::SyntaxError {
                expression: expr.to_string(),
                reason: e.to_string(),
            }),
        }
    }

    /// Evaluate multiple expressions (per-frame equations).
    pub fn eval_per_frame(&mut self, equations: &[String]) -> Result<()> {
        for equation in equations {
            self.eval(equation)?;
        }
        Ok(())
    }

    /// Evaluate per-pixel equations for a single pixel.
    pub fn eval_per_pixel(
        &mut self,
        x: f64,
        y: f64,
        rad: f64,
        ang: f64,
        equations: &[String],
    ) -> Result<()> {
        // Set pixel position
        self.context.set_pixel(x, y, rad, ang);

        // Evaluate all per-pixel equations
        for equation in equations {
            self.eval(equation)?;
        }

        Ok(())
    }

    /// Parse an assignment expression and update context.
    /// Returns the assigned value.
    pub fn eval_assignment(&mut self, expression: &str) -> Result<f64> {
        let result = self.eval(expression)?;

        // Try to extract variable name from assignment
        if let Some((var_name, _)) = expression.split_once('=') {
            let var_name = var_name.trim();
            self.context.set_var(var_name, result);
        }

        Ok(result)
    }

    /// Reset the evaluator to initial state.
    pub fn reset(&mut self) {
        self.context = MilkContext::new();
        self.compiled_cache.clear();
    }
}

impl Default for MilkEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_simple_expression() {
        let mut eval = MilkEvaluator::new();
        let result = eval.eval("2 + 2").unwrap();
        assert_relative_eq!(result, 4.0);
    }

    #[test]
    fn test_math_functions() {
        let mut eval = MilkEvaluator::new();

        let result = eval.eval("sin(0)").unwrap();
        assert_relative_eq!(result, 0.0, epsilon = 1e-10);

        let result = eval.eval("cos(0)").unwrap();
        assert_relative_eq!(result, 1.0, epsilon = 1e-10);

        let result = eval.eval("sqrt(16)").unwrap();
        assert_relative_eq!(result, 4.0);
    }

    #[test]
    fn test_variable_assignment() {
        let mut eval = MilkEvaluator::new();

        eval.eval("zoom = 1.5").unwrap();
        let zoom = eval.context().get_var("zoom").unwrap();
        assert_relative_eq!(zoom, 1.5);
    }

    #[test]
    fn test_variable_usage() {
        let mut eval = MilkEvaluator::new();

        eval.context_mut().set_var("time", 1.0);
        let result = eval.eval("time * 2").unwrap();
        assert_relative_eq!(result, 2.0);
    }

    #[test]
    fn test_complex_expression() {
        let mut eval = MilkEvaluator::new();
        eval.context_mut().set_var("time", 1.0);

        let result = eval.eval("0.5 + 0.4 * sin(time * 2)").unwrap();
        let expected = 0.5 + 0.4 * (1.0_f64 * 2.0).sin();
        assert_relative_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_per_frame_equations() {
        let mut eval = MilkEvaluator::new();

        let equations = vec![
            "wave_r = 0.5".to_string(),
            "wave_g = 0.3".to_string(),
            "wave_b = 0.7".to_string(),
        ];

        eval.eval_per_frame(&equations).unwrap();

        assert_relative_eq!(eval.context().get_var("wave_r").unwrap(), 0.5);
        assert_relative_eq!(eval.context().get_var("wave_g").unwrap(), 0.3);
        assert_relative_eq!(eval.context().get_var("wave_b").unwrap(), 0.7);
    }

    #[test]
    fn test_per_pixel_equations() {
        let mut eval = MilkEvaluator::new();

        let equations = vec!["zoom = zoom + 0.1 * rad".to_string()];

        eval.context_mut().set_var("zoom", 1.0);
        eval.eval_per_pixel(0.5, 0.5, 0.5, 0.0, &equations).unwrap();

        let zoom = eval.context().get_var("zoom").unwrap();
        assert_relative_eq!(zoom, 1.05);
    }

    #[test]
    fn test_q_variables() {
        let mut eval = MilkEvaluator::new();

        eval.eval("q1 = 42").unwrap();
        eval.eval("q2 = q1 * 2").unwrap();

        assert_relative_eq!(eval.context().get_var("q1").unwrap(), 42.0);
        assert_relative_eq!(eval.context().get_var("q2").unwrap(), 84.0);
    }
}
