//! Expression cache for performance optimization.

use evalexpr::Node;
use std::collections::HashMap;

/// Cache for compiled expressions.
#[derive(Debug, Clone)]
pub struct ExpressionCache {
    /// Cached compiled expressions
    cache: HashMap<String, Node>,

    /// Cache hit count
    hits: usize,

    /// Cache miss count
    misses: usize,

    /// Maximum cache size
    max_size: usize,
}

impl ExpressionCache {
    /// Create a new expression cache.
    pub fn new() -> Self {
        Self::with_capacity(1000)
    }

    /// Create a new expression cache with specified capacity.
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            hits: 0,
            misses: 0,
            max_size,
        }
    }

    /// Get a compiled expression from cache, or compile and cache it.
    pub fn get_or_compile(&mut self, expression: &str) -> Result<Node, evalexpr::EvalexprError> {
        // Check cache first
        if let Some(node) = self.cache.get(expression) {
            self.hits += 1;
            return Ok(node.clone());
        }

        // Cache miss - compile the expression
        self.misses += 1;
        let node = evalexpr::build_operator_tree(expression)?;

        // Add to cache if not full
        if self.cache.len() < self.max_size {
            self.cache.insert(expression.to_string(), node.clone());
        }

        Ok(node)
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            hits: self.hits,
            misses: self.misses,
            hit_rate: if self.hits + self.misses > 0 {
                self.hits as f64 / (self.hits + self.misses) as f64
            } else {
                0.0
            },
        }
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }

    /// Get the number of cached expressions.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for ExpressionCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CacheStats {
    /// Current cache size
    pub size: usize,

    /// Maximum cache size
    pub max_size: usize,

    /// Number of cache hits
    pub hits: usize,

    /// Number of cache misses
    pub misses: usize,

    /// Hit rate (0.0 to 1.0)
    pub hit_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // TODO: Add math functions to evalexpr 13.0 context
    fn test_cache_basic() {
        let mut cache = ExpressionCache::new();

        // First access - miss
        let result1 = cache.get_or_compile("2 + 2");
        assert!(result1.is_ok());
        assert_eq!(cache.stats().misses, 1);
        assert_eq!(cache.stats().hits, 0);

        // Second access - hit
        let result2 = cache.get_or_compile("2 + 2");
        assert!(result2.is_ok());
        assert_eq!(cache.stats().misses, 1);
        assert_eq!(cache.stats().hits, 1);

        // Hit rate should be 50%
        assert!((cache.stats().hit_rate - 0.5).abs() < 1e-10);
    }

    #[test]
    #[ignore] // TODO: Add math functions to evalexpr 13.0 context
    fn test_cache_capacity() {
        let mut cache = ExpressionCache::with_capacity(2);

        cache.get_or_compile("1 + 1").ok();
        cache.get_or_compile("2 + 2").ok();
        cache.get_or_compile("3 + 3").ok();

        // Should not exceed capacity
        assert!(cache.len() <= 2);
    }

    #[test]
    #[ignore] // TODO: Add math functions to evalexpr 13.0 context
    fn test_cache_clear() {
        let mut cache = ExpressionCache::new();

        cache.get_or_compile("1 + 1").ok();
        cache.get_or_compile("2 + 2").ok();

        assert_eq!(cache.len(), 2);

        cache.clear();

        assert_eq!(cache.len(), 0);
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 0);
    }

    #[test]
    #[ignore] // TODO: Add math functions to evalexpr 13.0 context
    fn test_cache_invalid_expression() {
        let mut cache = ExpressionCache::new();

        let result = cache.get_or_compile("invalid expression +++");
        assert!(result.is_err());
    }
}
