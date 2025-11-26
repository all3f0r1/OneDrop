//! Shader generator
//!
//! Generates complete WGSL shaders from Milkdrop presets.

use crate::error::{CodegenError, Result};
use crate::transpiler::ExpressionTranspiler;
use onedrop_parser::MilkPreset;

pub struct ShaderGenerator {
    transpiler: ExpressionTranspiler,
}

impl ShaderGenerator {
    pub fn new() -> Self {
        Self {
            transpiler: ExpressionTranspiler::new(),
        }
    }
    
    /// Generate a per-pixel shader from equations
    pub fn generate_per_pixel_shader(&self, preset: &MilkPreset) -> Result<String> {
        let mut shader = String::new();
        
        // Add shader header
        shader.push_str(&self.generate_header());
        
        // Add variable struct
        shader.push_str(&self.generate_variable_struct());
        
        // Add uniforms
        shader.push_str(&self.generate_uniforms());
        
        // Add vertex shader
        shader.push_str(&self.generate_vertex_shader());
        
        // Add fragment shader with per-pixel equations
        shader.push_str(&self.generate_fragment_shader(preset)?);
        
        Ok(shader)
    }
    
    fn generate_header(&self) -> String {
        "// Auto-generated WGSL shader from Milkdrop preset\n\n".to_string()
    }
    
    fn generate_variable_struct(&self) -> String {
        r#"struct PixelVars {
    // Coordinates
    x: f32,
    y: f32,
    rad: f32,
    ang: f32,
    
    // Audio
    bass: f32,
    mid: f32,
    treb: f32,
    bass_att: f32,
    mid_att: f32,
    treb_att: f32,
    
    // Time
    time: f32,
    frame: f32,
    fps: f32,
    
    // Padding for alignment
    _padding: f32,
    
    // Custom variables (vec4 for proper alignment)
    q: array<vec4<f32>, 16>,  // 64 floats as 16 vec4s
}

"#.to_string()
    }
    
    fn generate_uniforms(&self) -> String {
        r#"@group(0) @binding(0)
var<uniform> vars: PixelVars;

@group(0) @binding(1)
var texture_sampler: sampler;

@group(0) @binding(2)
var input_texture: texture_2d<f32>;

"#.to_string()
    }
    
    fn generate_vertex_shader(&self) -> String {
        r#"struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.uv = input.uv;
    return output;
}

"#.to_string()
    }
    
    fn generate_fragment_shader(&self, preset: &MilkPreset) -> Result<String> {
        let mut shader = String::new();
        
        shader.push_str("@fragment\n");
        shader.push_str("fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {\n");
        shader.push_str("    // Sample input texture\n");
        shader.push_str("    var color = textureSample(input_texture, texture_sampler, input.uv);\n\n");
        
        // Add per-pixel equations
        if !preset.per_pixel_equations.is_empty() {
            shader.push_str("    // Per-pixel equations\n");
            shader.push_str("    var vars = vars;\n");  // Local copy
            shader.push_str("    vars.x = input.uv.x;\n");
            shader.push_str("    vars.y = input.uv.y;\n");
            shader.push_str("    vars.rad = length(input.uv - vec2<f32>(0.5, 0.5));\n");
            shader.push_str("    vars.ang = atan2(input.uv.y - 0.5, input.uv.x - 0.5);\n\n");
            
            for equation in &preset.per_pixel_equations {
                let wgsl = self.transpiler.transpile(equation)?;
                shader.push_str("    ");
                shader.push_str(&wgsl);
                shader.push_str("\n");
            }
            
            shader.push_str("\n");
        }
        
        shader.push_str("    return color;\n");
        shader.push_str("}\n");
        
        Ok(shader)
    }
}

impl Default for ShaderGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_empty_shader() {
        let generator = ShaderGenerator::new();
        let preset = MilkPreset::default();
        let shader = generator.generate_per_pixel_shader(&preset).unwrap();
        
        assert!(shader.contains("@vertex"));
        assert!(shader.contains("@fragment"));
        assert!(shader.contains("PixelVars"));
    }
    
    #[test]
    fn test_generate_shader_with_equations() {
        let generator = ShaderGenerator::new();
        let mut preset = MilkPreset::default();
        preset.per_pixel_equations.push("x = x + 0.01".to_string());
        
        let shader = generator.generate_per_pixel_shader(&preset).unwrap();
        
        assert!(shader.contains("vars.x"));
        assert!(shader.contains("0.01"));
    }
}
