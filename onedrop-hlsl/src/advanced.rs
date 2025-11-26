//! Advanced HLSL to WGSL Translation
//!
//! Handles complex HLSL features like control flow, advanced functions, and intrinsics.

use regex::Regex;
use crate::{Result, TranslationError};

/// Advanced HLSL translator with support for complex features
pub struct AdvancedTranslator {
    /// Track variable declarations
    variables: Vec<String>,
    /// Track function declarations
    functions: Vec<String>,
}

impl AdvancedTranslator {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            functions: Vec::new(),
        }
    }
    
    /// Translate complex HLSL shader to WGSL
    pub fn translate(&mut self, hlsl: &str) -> Result<String> {
        let mut wgsl = hlsl.to_string();
        
        // 1. Control flow
        wgsl = self.translate_control_flow(&wgsl)?;
        
        // 2. Advanced functions
        wgsl = self.translate_advanced_functions(&wgsl)?;
        
        // 3. Intrinsics
        wgsl = self.translate_intrinsics(&wgsl)?;
        
        // 4. Swizzling
        wgsl = self.translate_swizzling(&wgsl)?;
        
        // 5. Struct definitions
        wgsl = self.translate_structs(&wgsl)?;
        
        // 6. Shader entry points
        wgsl = self.translate_entry_points(&wgsl)?;
        
        Ok(wgsl)
    }
    
    /// Translate control flow (if, for, while)
    fn translate_control_flow(&self, code: &str) -> Result<String> {
        let mut result = code.to_string();
        
        // HLSL [unroll] → WGSL @unroll
        result = result.replace("[unroll]", "@unroll");
        result = result.replace("[loop]", "");
        
        // HLSL [branch] → remove (WGSL doesn't have equivalent)
        result = result.replace("[branch]", "");
        result = result.replace("[flatten]", "");
        
        Ok(result)
    }
    
    /// Translate advanced functions
    fn translate_advanced_functions(&self, code: &str) -> Result<String> {
        let mut result = code.to_string();
        
        // ddx → dpdx
        result = result.replace("ddx(", "dpdx(");
        
        // ddy → dpdy
        result = result.replace("ddy(", "dpdy(");
        
        // atan2 → atan
        result = result.replace("atan2(", "atan(");
        
        // rsqrt → inverseSqrt
        result = result.replace("rsqrt(", "inverseSqrt(");
        
        // mad(a, b, c) → fma(a, b, c)
        let re = Regex::new(r"mad\(([^,]+),\s*([^,]+),\s*([^)]+)\)").unwrap();
        result = re.replace_all(&result, "fma($1, $2, $3)").to_string();
        
        // rcp(x) → 1.0 / x
        let re = Regex::new(r"rcp\(([^)]+)\)").unwrap();
        result = re.replace_all(&result, "(1.0 / $1)").to_string();
        
        // sincos(x, s, c) → s = sin(x); c = cos(x);
        let re = Regex::new(r"sincos\(([^,]+),\s*([^,]+),\s*([^)]+)\)").unwrap();
        result = re.replace_all(&result, "$2 = sin($1); $3 = cos($1)").to_string();
        
        Ok(result)
    }
    
    /// Translate intrinsic functions
    fn translate_intrinsics(&self, code: &str) -> Result<String> {
        let mut result = code.to_string();
        
        // clip(x) → if (x < 0.0) { discard; }
        let re = Regex::new(r"clip\(([^)]+)\)").unwrap();
        result = re.replace_all(&result, "if ($1 < 0.0) { discard; }").to_string();
        
        // all(x) → all(x) (same in WGSL)
        // any(x) → any(x) (same in WGSL)
        
        // step(edge, x) → step(edge, x) (same)
        // smoothstep(a, b, x) → smoothstep(a, b, x) (same)
        
        // reflect(i, n) → reflect(i, n) (same)
        // refract(i, n, eta) → refract(i, n, eta) (same)
        
        Ok(result)
    }
    
    /// Translate swizzling
    fn translate_swizzling(&self, code: &str) -> Result<String> {
        // HLSL and WGSL have same swizzling syntax
        // .xyzw, .rgba work the same
        Ok(code.to_string())
    }
    
    /// Translate struct definitions
    fn translate_structs(&self, code: &str) -> Result<String> {
        let mut result = code.to_string();
        
        // struct Name { ... }; → struct Name { ... }
        // Same syntax, but remove semantics
        let re = Regex::new(r"struct\s+(\w+)\s*\{([^}]+)\}").unwrap();
        
        for cap in re.captures_iter(code) {
            let struct_name = &cap[1];
            let struct_body = &cap[2];
            
            // Remove semantics from struct members
            let re_semantic = Regex::new(r":\s*[A-Z_][A-Z0-9_]*").unwrap();
            let clean_body = re_semantic.replace_all(struct_body, "");
            
            let new_struct = format!("struct {} {{{}}}", struct_name, clean_body);
            result = result.replace(&cap[0], &new_struct);
        }
        
        Ok(result)
    }
    
    /// Translate shader entry points
    fn translate_entry_points(&self, code: &str) -> Result<String> {
        let mut result = code.to_string();
        
        // VS_OUTPUT VSMain(...) → @vertex fn vs_main(...) -> VSOutput
        let re = Regex::new(r"(\w+)\s+VSMain\s*\(([^)]*)\)").unwrap();
        if let Some(cap) = re.captures(code) {
            let return_type = &cap[1];
            let params = &cap[2];
            let new_sig = format!("@vertex fn vs_main({}) -> {}", params, return_type);
            result = re.replace(&result, new_sig.as_str()).to_string();
        }
        
        // PS_OUTPUT PSMain(...) → @fragment fn fs_main(...) -> PSOutput
        let re = Regex::new(r"(\w+)\s+PSMain\s*\(([^)]*)\)").unwrap();
        if let Some(cap) = re.captures(code) {
            let return_type = &cap[1];
            let params = &cap[2];
            let new_sig = format!("@fragment fn fs_main({}) -> {}", params, return_type);
            result = re.replace(&result, new_sig.as_str()).to_string();
        }
        
        Ok(result)
    }
}

impl Default for AdvancedTranslator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_control_flow() {
        let mut translator = AdvancedTranslator::new();
        let hlsl = "[unroll] for (int i = 0; i < 4; i++) {}";
        let wgsl = translator.translate(hlsl).unwrap();
        assert!(wgsl.contains("@unroll"));
    }
    
    #[test]
    fn test_advanced_functions() {
        let mut translator = AdvancedTranslator::new();
        let hlsl = "float x = ddx(uv.x);";
        let wgsl = translator.translate(hlsl).unwrap();
        assert!(wgsl.contains("dpdx"));
    }
    
    #[test]
    fn test_mad_to_fma() {
        let mut translator = AdvancedTranslator::new();
        let hlsl = "float result = mad(a, b, c);";
        let wgsl = translator.translate(hlsl).unwrap();
        assert!(wgsl.contains("fma"));
    }
    
    #[test]
    fn test_rcp() {
        let mut translator = AdvancedTranslator::new();
        let hlsl = "float inv = rcp(x);";
        let wgsl = translator.translate(hlsl).unwrap();
        assert!(wgsl.contains("1.0 / x"));
    }
    
    #[test]
    fn test_clip() {
        let mut translator = AdvancedTranslator::new();
        let hlsl = "clip(alpha - 0.5);";
        let wgsl = translator.translate(hlsl).unwrap();
        assert!(wgsl.contains("discard"));
    }
    
    #[test]
    fn test_entry_points() {
        let mut translator = AdvancedTranslator::new();
        let hlsl = "VS_OUTPUT VSMain(VS_INPUT input) { }";
        let wgsl = translator.translate(hlsl).unwrap();
        assert!(wgsl.contains("@vertex"));
        assert!(wgsl.contains("vs_main"));
    }
}
