// Advanced waveform shader with multiple rendering modes

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    wave_mode: u32,
    wave_scale: f32,
    wave_alpha: f32,
    wave_smoothing: f32,
    wave_additive: u32,
    wave_dots: u32,
    wave_thick: u32,
    wave_color: vec4<f32>,
}

struct WavePoint {
    position: vec2<f32>,
    value: f32,
    _padding: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> wave_data: array<WavePoint>;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    let num_points = arrayLength(&wave_data);
    let point_idx = input.vertex_index / 6u; // 6 vertices per quad
    let vertex_in_quad = input.vertex_index % 6u;
    
    if (point_idx >= num_points - 1u) {
        // Out of bounds
        output.position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
        output.color = vec4<f32>(0.0);
        output.uv = vec2<f32>(0.0);
        return output;
    }
    
    let point = wave_data[point_idx];
    let next_point = wave_data[point_idx + 1u];
    
    // Calculate positions based on wave mode
    var pos: vec2<f32>;
    var thickness = 0.002;
    
    if (uniforms.wave_thick != 0u) {
        thickness = 0.004;
    }
    
    // Mode 0: Centered waveform
    // Mode 1: Left channel
    // Mode 2: Right channel
    // Mode 3: Spectrum
    
    let x = point.position.x;
    let y = point.value * uniforms.wave_scale;
    
    // Create quad vertices
    var offset: vec2<f32>;
    switch (vertex_in_quad) {
        case 0u: { offset = vec2<f32>(-thickness, -thickness); }
        case 1u: { offset = vec2<f32>(thickness, -thickness); }
        case 2u: { offset = vec2<f32>(thickness, thickness); }
        case 3u: { offset = vec2<f32>(-thickness, -thickness); }
        case 4u: { offset = vec2<f32>(thickness, thickness); }
        default: { offset = vec2<f32>(-thickness, thickness); }
    }
    
    pos = vec2<f32>(x, y) + offset;
    
    // Convert to clip space (-1 to 1)
    let clip_pos = vec2<f32>(
        pos.x * 2.0 - 1.0,
        pos.y * 2.0 - 1.0
    );
    
    output.position = vec4<f32>(clip_pos, 0.0, 1.0);
    
    // Color with alpha
    output.color = vec4<f32>(
        uniforms.wave_color.rgb,
        uniforms.wave_color.a * uniforms.wave_alpha
    );
    
    output.uv = pos;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var color = input.color;
    
    // Apply smoothing/glow effect
    if (uniforms.wave_smoothing > 0.0) {
        let dist = length(input.uv - vec2<f32>(0.5));
        let glow = exp(-dist * uniforms.wave_smoothing * 10.0);
        color.a *= glow;
    }
    
    // Additive blending handled by pipeline
    return color;
}

// Dot rendering mode
@vertex
fn vs_dots(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    let num_points = arrayLength(&wave_data);
    let point_idx = input.vertex_index / 6u;
    let vertex_in_quad = input.vertex_index % 6u;
    
    if (point_idx >= num_points) {
        output.position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
        output.color = vec4<f32>(0.0);
        output.uv = vec2<f32>(0.0);
        return output;
    }
    
    let point = wave_data[point_idx];
    let dot_size = 0.005;
    
    let x = point.position.x;
    let y = point.value * uniforms.wave_scale;
    
    // Create quad for dot
    var offset: vec2<f32>;
    switch (vertex_in_quad) {
        case 0u: { offset = vec2<f32>(-dot_size, -dot_size); }
        case 1u: { offset = vec2<f32>(dot_size, -dot_size); }
        case 2u: { offset = vec2<f32>(dot_size, dot_size); }
        case 3u: { offset = vec2<f32>(-dot_size, -dot_size); }
        case 4u: { offset = vec2<f32>(dot_size, dot_size); }
        default: { offset = vec2<f32>(-dot_size, dot_size); }
    }
    
    let pos = vec2<f32>(x, y) + offset;
    let clip_pos = vec2<f32>(pos.x * 2.0 - 1.0, pos.y * 2.0 - 1.0);
    
    output.position = vec4<f32>(clip_pos, 0.0, 1.0);
    output.color = vec4<f32>(uniforms.wave_color.rgb, uniforms.wave_alpha);
    output.uv = offset / dot_size; // -1 to 1 within dot
    
    return output;
}

@fragment
fn fs_dots(input: VertexOutput) -> @location(0) vec4<f32> {
    // Circular dot shape
    let dist = length(input.uv);
    if (dist > 1.0) {
        discard;
    }
    
    var color = input.color;
    // Smooth edges
    color.a *= 1.0 - smoothstep(0.8, 1.0, dist);
    
    return color;
}
