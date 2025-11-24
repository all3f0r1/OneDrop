//! Optimized evaluator with expression caching.

use crate::cache::ExpressionCache;
use crate::context::MilkContext;
use crate::error::{EvalError, Result};
use evalexpr::Node;

/// Optimized evaluator for Milkdrop expressions with caching.
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
            cache: ExpressionCache::default(),
        }
    }
    
    /// Create with custom cache size.
    pub fn with_cache_size(cache_size: usize) -> Self {
        Self {
            context: MilkContext::new(),
            cache: ExpressionCache::new(cache_size),
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
    
    /// Evaluate a single expression (optimized with cache).
    pub fn eval(&mut self, expression: &str) -> Result<f64> {
        // Clean the expression
        let expr = expression.trim().trim_end_matches(';').trim();
        
        if expr.is_empty() {
            return Ok(0.0);
        }
        
        // Get or compile the expression
        let node = self.cache.get_or_compile(expr)
            .map_err(|e| EvalError::EvaluationFailed(e.to_string()))?;
        
        // Evaluate the compiled node
        self.eval_node(&node)
    }
    
    /// Evaluate a compiled node.
    fn eval_node(&mut self, node: &Node) -> Result<f64> {
        match node.eval_with_context_mut(self.context.inner_mut()) {
            Ok(value) => {
                match value {
                    evalexpr::Value::Float(f) => Ok(f),
                    evalexpr::Value::Int(i) => Ok(i as f64),
                    evalexpr::Value::Boolean(b) => Ok(if b { 1.0 } else { 0.0 }),
                    evalexpr::Value::Empty => Ok(0.0),
                    _ => Err(EvalError::InvalidResult(format!("{:?}", value))),
                }
            }
            Err(e) => Err(EvalError::EvaluationFailed(e.to_string())),
        }
    }
    
    /// Evaluate multiple expressions in batch.
    pub fn eval_batch(&mut self, expressions: &[String]) -> Vec<Result<f64>> {
        expressions.iter().map(|expr| self.eval(expr)).collect()
    }
    
    /// Pre-compile expressions for later use.
    pub fn precompile(&mut self, expressions: &[String]) -> Result<()> {
        for expr in expressions {
            let _ = self.cache.get_or_compile(expr)
                .map_err(|e| EvalError::EvaluationFailed(e.to_string()))?;
        }
        Ok(())
    }
    
    /// Reset the evaluator (clear context and cache).
    pub fn reset(&mut self) {
        self.context = MilkContext::new();
        self.cache.clear();
    }
    
    /// Get cache statistics.
    pub fn cache_stats(&self) -> crate::cache::CacheStats {
        self.cache.stats()
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
        
        eval.context_mut().set_var("x", 5.0);
        
        let result = eval.eval("x + 1").unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_cache_performance() {
        let mut eval = OptimizedEvaluator::new();
        
        eval.context_mut().set_var("x", 1.0);
        
        // First evaluation - cache miss
        eval.eval("x + 1").unwrap();
        let stats1 = eval.cache_stats();
        assert_eq!(stats1.misses, 1);
        
        // Second evaluation - cache hit
        eval.eval("x + 1").unwrap();
        let stats2 = eval.cache_stats();
        assert_eq!(stats2.hits, 1);
        assert_eq!(stats2.hit_rate, 0.5);
    }

    #[test]
    fn test_precompile() {
        let mut eval = OptimizedEvaluator::new();
        
        let expressions = vec![
            "x + 1".to_string(),
            "y * 2".to_string(),
            "sin(time)".to_string(),
        ];
        
        eval.precompile(&expressions).unwrap();
        
        let stats = eval.cache_stats();
        assert_eq!(stats.size, 3);
    }

    #[test]
    fn test_batch_eval() {
        let mut eval = OptimizedEvaluator::new();
        
        eval.context_mut().set_var("x", 2.0);
        eval.context_mut().set_var("y", 3.0);
        
        let expressions = vec![
            "x + 1".to_string(),
            "y * 2".to_string(),
            "x + y".to_string(),
        ];
        
        let results = eval.eval_batch(&expressions);
        
        assert_eq!(results[0].as_ref().unwrap(), &3.0);
        assert_eq!(results[1].as_ref().unwrap(), &6.0);
        assert_eq!(results[2].as_ref().unwrap(), &5.0);
    }
}
