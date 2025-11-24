// Composite shader - combines layers and applies effects

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    decay: f32,
    zoom: f32,
    rot: f32,
    cx: f32,
    cy: f32,
    dx: f32,
    dy: f32,
    sx: f32,
    sy: f32,
    warp: f32,
    _padding: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var prev_texture: texture_2d<f32>;

@group(0) @binding(2)
var texture_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // Full-screen quad
    let x = f32((vertex_index & 1u) << 2u);
    let y = f32((vertex_index & 2u) << 1u);
    
    output.position = vec4<f32>(x - 1.0, 1.0 - y, 0.0, 1.0);
    output.uv = vec2<f32>(x * 0.5, y * 0.5);
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Center coordinates
    var uv = input.uv - vec2<f32>(uniforms.cx, uniforms.cy);
    
    // Apply rotation
    let cos_r = cos(uniforms.rot);
    let sin_r = sin(uniforms.rot);
    uv = vec2<f32>(
        uv.x * cos_r - uv.y * sin_r,
        uv.x * sin_r + uv.y * cos_r
    );
    
    // Apply zoom
    uv = uv / uniforms.zoom;
    
    // Apply stretch
    uv = vec2<f32>(uv.x / uniforms.sx, uv.y / uniforms.sy);
    
    // Apply translation
    uv = uv + vec2<f32>(uniforms.dx, uniforms.dy);
    
    // Back to texture coordinates
    uv = uv + vec2<f32>(uniforms.cx, uniforms.cy);
    
    // Sample previous frame
    var color: vec4<f32>;
    if (uv.x >= 0.0 && uv.x <= 1.0 && uv.y >= 0.0 && uv.y <= 1.0) {
        color = textureSample(prev_texture, texture_sampler, uv);
    } else {
        color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    
    // Apply decay
    color = color * uniforms.decay;
    
    return color;
}
