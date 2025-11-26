//! Expression transpiler implementation

use crate::error::{CodegenError, Result};
use super::VariableMapper;

pub struct ExpressionTranspiler {
    variable_mapper: VariableMapper,
}

impl ExpressionTranspiler {
    pub fn new() -> Self {
        Self {
            variable_mapper: VariableMapper::new(),
        }
    }
    
    /// Transpile a Milkdrop equation to WGSL
    pub fn transpile(&self, equation: &str) -> Result<String> {
        // Remove whitespace
        let equation = equation.trim();
        
        if equation.is_empty() {
            return Ok(String::new());
        }
        
        // Parse assignment (e.g., "x = expression")
        if let Some((lhs, rhs)) = equation.split_once('=') {
            let lhs = self.transpile_variable(lhs.trim())?;
            let rhs = self.transpile_expression(rhs.trim())?;
            Ok(format!("{} = {};", lhs, rhs))
        } else {
            // Just an expression
            let expr = self.transpile_expression(equation)?;
            Ok(format!("{};", expr))
        }
    }
    
    /// Transpile a variable name
    fn transpile_variable(&self, var: &str) -> Result<String> {
        self.variable_mapper.map_variable(var)
    }
    
    /// Transpile an expression
    fn transpile_expression(&self, expr: &str) -> Result<String> {
        let mut result = expr.to_string();
        
        // Replace Milkdrop functions with WGSL equivalents
        result = self.replace_functions(&result);
        
        // Replace variables
        result = self.replace_variables(&result)?;
        
        Ok(result)
    }
    
    /// Replace function names
    fn replace_functions(&self, expr: &str) -> String {
        let mut result = expr.to_string();
        
        // Math functions (mostly compatible)
        // sin, cos, tan, sqrt, abs, etc. are the same in WGSL
        
        // Special replacements
        result = result.replace("pow(", "pow(");  // Same
        result = result.replace("atan2(", "atan2(");  // Same
        result = result.replace("min(", "min(");  // Same
        result = result.replace("max(", "max(");  // Same
        result = result.replace("clamp(", "clamp(");  // Same
        
        result
    }
    
    /// Replace variable names
    fn replace_variables(&self, expr: &str) -> Result<String> {
        let mut result = expr.to_string();
        
        // Common variables
        let vars = [
            ("time", "vars.time"),
            ("frame", "vars.frame"),
            ("bass", "vars.bass"),
            ("mid", "vars.mid"),
            ("treb", "vars.treb"),
            ("x", "vars.x"),
            ("y", "vars.y"),
            ("rad", "vars.rad"),
            ("ang", "vars.ang"),
        ];
        
        for (from, to) in &vars {
            // Only replace whole words
            result = Self::replace_word(&result, from, to);
        }
        
        // Replace q variables (q1-q64)
        for i in 1..=64 {
            let from = format!("q{}", i);
            let to = format!("vars.q[{}]", i - 1);
            result = Self::replace_word(&result, &from, &to);
        }
        
        Ok(result)
    }
    
    /// Replace a whole word (not part of another word)
    fn replace_word(text: &str, from: &str, to: &str) -> String {
        let mut result = String::new();
        let mut chars = text.chars().peekable();
        let from_chars: Vec<char> = from.chars().collect();
        
        while let Some(&ch) = chars.peek() {
            // Try to match the word
            let mut matched = true;
            let mut temp_chars = chars.clone();
            
            // Check if we're at the start of a word
            if result.is_empty() || !result.chars().last().unwrap().is_alphanumeric() {
                for &fc in &from_chars {
                    if let Some(&tc) = temp_chars.peek() {
                        if tc == fc {
                            temp_chars.next();
                        } else {
                            matched = false;
                            break;
                        }
                    } else {
                        matched = false;
                        break;
                    }
                }
                
                // Check if we're at the end of a word
                if matched {
                    if let Some(&next_ch) = temp_chars.peek() {
                        if next_ch.is_alphanumeric() || next_ch == '_' {
                            matched = false;
                        }
                    }
                }
                
                if matched {
                    result.push_str(to);
                    chars = temp_chars;
                    continue;
                }
            }
            
            result.push(chars.next().unwrap());
        }
        
        result
    }
}

impl Default for ExpressionTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_assignment() {
        let transpiler = ExpressionTranspiler::new();
        let result = transpiler.transpile("x = 0.5").unwrap();
        assert_eq!(result, "vars.x = 0.5;");
    }
    
    #[test]
    fn test_math_expression() {
        let transpiler = ExpressionTranspiler::new();
        let result = transpiler.transpile("x = x + 0.01*sin(time)").unwrap();
        assert!(result.contains("vars.x"));
        assert!(result.contains("sin(vars.time)"));
    }
    
    #[test]
    fn test_q_variable() {
        let transpiler = ExpressionTranspiler::new();
        let result = transpiler.transpile("x = q1 + q2").unwrap();
        assert!(result.contains("vars.q[0]"));
        assert!(result.contains("vars.q[1]"));
    }
    
    #[test]
    fn test_replace_word() {
        let result = ExpressionTranspiler::replace_word("x + x2 + x", "x", "vars.x");
        assert_eq!(result, "vars.x + x2 + vars.x");
    }
}
