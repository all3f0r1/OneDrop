//! OneDrop Code Generation
//!
//! Generate WGSL shaders from Milkdrop presets.

pub mod compiler;
pub mod error;
pub mod generator;
pub mod transpiler;

pub use compiler::{CacheStats, CompiledShader, ShaderCompiler};
pub use error::{CodegenError, Result};
pub use generator::ShaderGenerator;
pub use transpiler::{ExpressionTranspiler, VariableMapper, transpile_equation};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpile_simple() {
        let result = transpile_equation("x = 0.5").unwrap();
        assert!(result.contains("vars.x"));
    }
}
