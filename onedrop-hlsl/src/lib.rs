//! HLSL to WGSL Translation
//!
//! Simplified HLSL to WGSL translator for Milkdrop shaders.

use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TranslationError {
    #[error("Translation error: {0}")]
    Translation(String),
    
    #[error("Unsupported HLSL feature: {0}")]
    Unsupported(String),
}

pub type Result<T> = std::result::Result<T, TranslationError>;

/// Translate HLSL shader code to WGSL
pub fn translate_shader(hlsl: &str) -> Result<String> {
    let mut wgsl = hlsl.to_string();
    
    // Type replacements
    wgsl = replace_types(&wgsl);
    
    // Function replacements
    wgsl = replace_functions(&wgsl);
    
    // Texture sampling
    wgsl = replace_texture_sampling(&wgsl);
    
    // Semantic replacements
    wgsl = replace_semantics(&wgsl);
    
    Ok(wgsl)
}

fn replace_types(code: &str) -> String {
    let mut result = code.to_string();
    
    // Vector types
    result = result.replace("float4", "vec4<f32>");
    result = result.replace("float3", "vec3<f32>");
    result = result.replace("float2", "vec2<f32>");
    result = result.replace("float", "f32");
    
    // Matrix types
    result = result.replace("float4x4", "mat4x4<f32>");
    result = result.replace("float3x3", "mat3x3<f32>");
    
    result
}

fn replace_functions(code: &str) -> String {
    let mut result = code.to_string();
    
    // lerp → mix
    result = result.replace("lerp(", "mix(");
    
    // saturate → clamp
    let re = Regex::new(r"saturate\(([^)]+)\)").unwrap();
    result = re.replace_all(&result, "clamp($1, 0.0, 1.0)").to_string();
    
    // frac → fract
    result = result.replace("frac(", "fract(");
    
    // mul(matrix, vector) → matrix * vector
    let re = Regex::new(r"mul\(([^,]+),\s*([^)]+)\)").unwrap();
    result = re.replace_all(&result, "$1 * $2").to_string();
    
    result
}

fn replace_texture_sampling(code: &str) -> String {
    let mut result = code.to_string();
    
    // tex2D(sampler, uv) → textureSample(texture, sampler, uv)
    let re = Regex::new(r"tex2D\(([^,]+),\s*([^)]+)\)").unwrap();
    result = re.replace_all(&result, "textureSample(texture_$1, sampler_$1, $2)").to_string();
    
    result
}

fn replace_semantics(code: &str) -> String {
    let mut result = code.to_string();
    
    // Remove HLSL semantics (: POSITION, : COLOR, etc.)
    let re = Regex::new(r":\s*[A-Z_][A-Z0-9_]*").unwrap();
    result = re.replace_all(&result, "").to_string();
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_replacement() {
        let hlsl = "float4 color = float4(1.0, 0.0, 0.0, 1.0);";
        let wgsl = translate_shader(hlsl).unwrap();
        assert!(wgsl.contains("vec4<f32>"));
    }
    
    #[test]
    fn test_function_replacement() {
        let hlsl = "color = lerp(a, b, t);";
        let wgsl = translate_shader(hlsl).unwrap();
        assert!(wgsl.contains("mix"));
    }
    
    #[test]
    fn test_saturate_replacement() {
        let hlsl = "color = saturate(color);";
        let wgsl = translate_shader(hlsl).unwrap();
        assert!(wgsl.contains("clamp"));
    }
    
    #[test]
    fn test_texture_sampling() {
        let hlsl = "color = tex2D(sampler0, uv);";
        let wgsl = translate_shader(hlsl).unwrap();
        assert!(wgsl.contains("textureSample"));
    }
}
