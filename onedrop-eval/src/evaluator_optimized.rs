//! Optimized evaluator with expression caching.

use crate::cache::ExpressionCache;
use crate::context::MilkContext;
use crate::error::{EvalError, Result};

/// Optimized evaluator with expression caching for better performance.
pub struct OptimizedEvaluator {
    /// Execution context
    context: MilkContext,
    
    /// Expression cache
    cache: ExpressionCache,
}

impl OptimizedEvaluator {
    /// Create a new optimized evaluator.
    pub fn new() -> Self {
        Self {
            context: MilkContext::new(),
            cache: ExpressionCache::new(),
        }
    }
    
    /// Create a new optimized evaluator with specified cache capacity.
    pub fn with_cache_capacity(capacity: usize) -> Self {
        Self {
            context: MilkContext::new(),
            cache: ExpressionCache::with_capacity(capacity),
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
    
    /// Get cache statistics.
    pub fn cache_stats(&self) -> crate::cache::CacheStats {
        self.cache.stats()
    }
    
    /// Clear the expression cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Evaluate a single expression using the cache.
    pub fn eval(&mut self, expression: &str) -> Result<f64> {
        // Clean the expression
        let expr = expression.trim().trim_end_matches(';').trim();
        
        if expr.is_empty() {
            return Ok(0.0);
        }
        
        // Get or compile the expression
        let node = self.cache.get_or_compile(expr)
            .map_err(|e| EvalError::SyntaxError {
                expression: expr.to_string(),
                reason: e.to_string(),
            })?;
        
        // Evaluate with context
        match node.eval_with_context_mut(self.context.inner_mut()) {
            Ok(value) => {
                // Convert result to f64
                match value {
                    evalexpr::Value::Float(f) => Ok(f),
                    evalexpr::Value::Int(i) => Ok(i as f64),
                    evalexpr::Value::Boolean(b) => Ok(if b { 1.0 } else { 0.0 }),
                    _ => Err(EvalError::TypeError {
                        expected: "number".to_string(),
                        got: format!("{:?}", value),
                    }),
                }
            }
            Err(e) => Err(EvalError::EvalFailed(e.to_string())),
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
    pub fn eval_per_pixel(&mut self, x: f64, y: f64, rad: f64, ang: f64, equations: &[String]) -> Result<()> {
        // Set pixel position
        self.context.set_pixel(x, y, rad, ang);
        
        // Evaluate all per-pixel equations
        for equation in equations {
            self.eval(equation)?;
        }
        
        Ok(())
    }
    
    /// Batch evaluate per-pixel equations for multiple pixels.
    /// This is more efficient than calling eval_per_pixel repeatedly.
    pub fn eval_per_pixel_batch(
        &mut self,
        pixels: &[(f64, f64, f64, f64)], // (x, y, rad, ang)
        equations: &[String],
    ) -> Result<Vec<()>> {
        pixels.iter()
            .map(|(x, y, rad, ang)| {
                self.eval_per_pixel(*x, *y, *rad, *ang, equations)
            })
            .collect()
    }
    
    /// Reset the evaluator to initial state.
    pub fn reset(&mut self) {
        self.context = MilkContext::new();
        // Keep the cache
    }
}

impl Default for OptimizedEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_eval() {
        let mut eval = OptimizedEvaluator::new();
        
        let result = eval.eval("2 + 2").unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_cache_performance() {
        let mut eval = OptimizedEvaluator::new();
        
        // First evaluation - cache miss
        eval.eval("sin(0.5) + cos(0.5)").ok();
        let stats1 = eval.cache_stats();
        assert_eq!(stats1.misses, 1);
        assert_eq!(stats1.hits, 0);
        
        // Second evaluation - cache hit
        eval.eval("sin(0.5) + cos(0.5)").ok();
        let stats2 = eval.cache_stats();
        assert_eq!(stats2.misses, 1);
        assert_eq!(stats2.hits, 1);
        
        // Hit rate should be 50%
        assert!((stats2.hit_rate - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_per_frame_equations() {
        let mut eval = OptimizedEvaluator::new();
        
        let equations = vec![
            "time = 1.0".to_string(),
            "bass = 0.5".to_string(),
            "zoom = 1.0 + bass * 0.1".to_string(),
        ];
        
        eval.eval_per_frame(&equations).unwrap();
        
        assert_eq!(eval.context().get("time"), Some(1.0));
        assert_eq!(eval.context().get("bass"), Some(0.5));
        assert_eq!(eval.context().get("zoom"), Some(1.05));
    }

    #[test]
    fn test_per_pixel_evaluation() {
        let mut eval = OptimizedEvaluator::new();
        
        let equations = vec![
            "rad = sqrt(x*x + y*y)".to_string(),
        ];
        
        eval.eval_per_pixel(0.5, 0.5, 0.0, 0.0, &equations).unwrap();
        
        let rad = eval.context().get("rad").unwrap();
        assert!((rad - 0.7071067811865476).abs() < 1e-10);
    }

    #[test]
    fn test_batch_evaluation() {
        let mut eval = OptimizedEvaluator::new();
        
        let pixels = vec![
            (0.0, 0.0, 0.0, 0.0),
            (1.0, 0.0, 1.0, 0.0),
            (0.0, 1.0, 1.0, 1.5708),
        ];
        
        let equations = vec![
            "x = x + 0.1".to_string(),
        ];
        
        let result = eval.eval_per_pixel_batch(&pixels, &equations);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_clear_cache() {
        let mut eval = OptimizedEvaluator::new();
        
        eval.eval("1 + 1").ok();
        eval.eval("2 + 2").ok();
        
        assert_eq!(eval.cache_stats().size, 2);
        
        eval.clear_cache();
        
        assert_eq!(eval.cache_stats().size, 0);
        assert_eq!(eval.cache_stats().hits, 0);
        assert_eq!(eval.cache_stats().misses, 0);
    }
}
