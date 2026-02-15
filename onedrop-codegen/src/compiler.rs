//! Shader compiler with naga validation

use crate::error::{CodegenError, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Compiled shader with validated module
#[derive(Clone)]
pub struct CompiledShader {
    /// Original WGSL source
    pub source: String,

    /// Validated naga module
    pub module: Arc<naga::Module>,

    /// Module info for validation
    pub info: Arc<naga::valid::ModuleInfo>,
}

/// Shader compiler with caching
pub struct ShaderCompiler {
    cache: Arc<Mutex<HashMap<String, CompiledShader>>>,
    validator: naga::valid::Validator,
}

impl ShaderCompiler {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            validator: naga::valid::Validator::new(
                naga::valid::ValidationFlags::all(),
                naga::valid::Capabilities::all(),
            ),
        }
    }

    /// Compile and validate a WGSL shader
    pub fn compile(&mut self, source: &str) -> Result<CompiledShader> {
        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(compiled) = cache.get(source) {
                log::debug!("Shader cache hit");
                return Ok(compiled.clone());
            }
        }

        log::debug!("Compiling shader ({} bytes)", source.len());

        // Parse WGSL
        let module = naga::front::wgsl::parse_str(source)
            .map_err(|e| CodegenError::Compilation(format!("WGSL parse error: {:?}", e)))?;

        // Validate
        let info = self
            .validator
            .validate(&module)
            .map_err(|e| CodegenError::Compilation(format!("Validation error: {:?}", e)))?;

        let compiled = CompiledShader {
            source: source.to_string(),
            module: Arc::new(module),
            info: Arc::new(info),
        };

        // Cache it
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(source.to_string(), compiled.clone());
        }

        log::debug!("Shader compiled and cached successfully");

        Ok(compiled)
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.lock().unwrap();
        CacheStats {
            size: cache.len(),
            total_source_bytes: cache.values().map(|s| s.source.len()).sum(),
        }
    }

    /// Clear the cache
    pub fn clear_cache(&mut self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        log::debug!("Shader cache cleared");
    }
}

impl Default for ShaderCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub total_source_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_simple_shader() {
        let mut compiler = ShaderCompiler::new();

        let source = r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

        let result = compiler.compile(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cache_hit() {
        let mut compiler = ShaderCompiler::new();

        let source = r#"
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

        // First compile
        let result1 = compiler.compile(source);
        assert!(result1.is_ok());

        // Second compile (should hit cache)
        let result2 = compiler.compile(source);
        assert!(result2.is_ok());

        let stats = compiler.cache_stats();
        assert_eq!(stats.size, 1);
    }

    #[test]
    fn test_invalid_shader() {
        let mut compiler = ShaderCompiler::new();

        let source = "invalid shader code";

        let result = compiler.compile(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_clear_cache() {
        let mut compiler = ShaderCompiler::new();

        let source = r#"
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

        compiler.compile(source).unwrap();
        assert_eq!(compiler.cache_stats().size, 1);

        compiler.clear_cache();
        assert_eq!(compiler.cache_stats().size, 0);
    }
}
