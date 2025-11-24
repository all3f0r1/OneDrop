//! Expression caching for performance optimization.

use evalexpr::Node;
use std::collections::HashMap;

/// Cache for compiled expressions.
pub struct ExpressionCache {
    /// Map from expression string to compiled node
    cache: HashMap<String, Node>,
    
    /// Maximum cache size
    max_size: usize,
    
    /// Hit counter for statistics
    hits: usize,
    
    /// Miss counter for statistics
    misses: usize,
}

impl ExpressionCache {
    /// Create a new expression cache.
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(max_size),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    
    /// Get or compile an expression.
    pub fn get_or_compile(&mut self, expr: &str) -> Result<Node, evalexpr::EvalexprError> {
        if let Some(node) = self.cache.get(expr) {
            self.hits += 1;
            return Ok(node.clone());
        }
        
        self.misses += 1;
        
        // Compile the expression
        let node = evalexpr::build_operator_tree(expr)?;
        
        // Add to cache if not full
        if self.cache.len() < self.max_size {
            self.cache.insert(expr.to_string(), node.clone());
        } else {
            // Cache is full, could implement LRU here
            // For now, just don't cache
            log::debug!("Expression cache full, not caching: {}", expr);
        }
        
        Ok(node)
    }
    
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
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
}

impl Default for ExpressionCache {
    fn default() -> Self {
        Self::new(1000) // Default cache size
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
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
    fn test_cache_basic() {
        let mut cache = ExpressionCache::new(10);
        
        // First access - miss
        let node1 = cache.get_or_compile("x + 1").unwrap();
        assert_eq!(cache.stats().misses, 1);
        assert_eq!(cache.stats().hits, 0);
        
        // Second access - hit
        let node2 = cache.get_or_compile("x + 1").unwrap();
        assert_eq!(cache.stats().misses, 1);
        assert_eq!(cache.stats().hits, 1);
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = ExpressionCache::new(10);
        
        cache.get_or_compile("x + 1").unwrap();
        cache.get_or_compile("x + 1").unwrap();
        cache.get_or_compile("y + 2").unwrap();
        
        let stats = cache.stats();
        assert_eq!(stats.size, 2);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 2);
        assert_eq!(stats.hit_rate, 1.0 / 3.0);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = ExpressionCache::new(10);
        
        cache.get_or_compile("x + 1").unwrap();
        assert_eq!(cache.stats().size, 1);
        
        cache.clear();
        assert_eq!(cache.stats().size, 0);
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 0);
    }
}
