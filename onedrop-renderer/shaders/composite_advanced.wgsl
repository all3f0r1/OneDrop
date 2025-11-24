// Advanced composite shader with motion effects and post-processing

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
    brighten: u32,
    darken: u32,
    solarize: u32,
    invert: u32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var prev_frame: texture_2d<f32>;
@group(0) @binding(2) var frame_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // Full-screen quad
    let x = f32((vertex_index & 1u) << 1u);
    let y = f32((vertex_index & 2u));
    
    output.position = vec4<f32>(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
    output.uv = vec2<f32>(x, 1.0 - y);
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Center coordinates
    var uv = input.uv - 0.5;
    
    // Apply rotation
    if (uniforms.rot != 0.0) {
        let cos_r = cos(uniforms.rot);
        let sin_r = sin(uniforms.rot);
        let rotated = vec2<f32>(
            uv.x * cos_r - uv.y * sin_r,
            uv.x * sin_r + uv.y * cos_r
        );
        uv = rotated;
    }
    
    // Apply zoom
    if (uniforms.zoom != 1.0) {
        uv = uv / uniforms.zoom;
    }
    
    // Apply stretch
    uv.x *= uniforms.sx;
    uv.y *= uniforms.sy;
    
    // Apply translation
    uv.x += uniforms.dx - (uniforms.cx - 0.5);
    uv.y += uniforms.dy - (uniforms.cy - 0.5);
    
    // Apply warp effect
    if (uniforms.warp != 0.0) {
        let dist = length(uv);
        let warp_amount = uniforms.warp * 0.1;
        uv = uv * (1.0 + warp_amount * sin(dist * 10.0 + uniforms.time));
    }
    
    // Back to 0-1 range
    uv = uv + 0.5;
    
    // Sample previous frame with wrapping
    var color: vec4<f32>;
    if (uv.x >= 0.0 && uv.x <= 1.0 && uv.y >= 0.0 && uv.y <= 1.0) {
        color = textureSample(prev_frame, frame_sampler, uv);
    } else {
        // Wrap or clamp
        let wrapped_uv = fract(uv);
        color = textureSample(prev_frame, frame_sampler, wrapped_uv);
    }
    
    // Apply decay
    color = color * uniforms.decay;
    
    // Apply effects
    if (uniforms.brighten != 0u) {
        color.r = min(color.r * 1.2, 1.0);
        color.g = min(color.g * 1.2, 1.0);
        color.b = min(color.b * 1.2, 1.0);
    }
    
    if (uniforms.darken != 0u) {
        color.r = color.r * 0.8;
        color.g = color.g * 0.8;
        color.b = color.b * 0.8;
    }
    
    if (uniforms.solarize != 0u) {
        color.r = abs(color.r - 0.5) * 2.0;
        color.g = abs(color.g - 0.5) * 2.0;
        color.b = abs(color.b - 0.5) * 2.0;
    }
    
    if (uniforms.invert != 0u) {
        color.r = 1.0 - color.r;
        color.g = 1.0 - color.g;
        color.b = 1.0 - color.b;
    }
    
    return color;
}

// Blur pass for post-processing
@fragment
fn fs_blur(input: VertexOutput) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / uniforms.resolution;
    var color = vec4<f32>(0.0);
    
    // 3x3 Gaussian blur
    let offsets = array<vec2<f32>, 9>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0,  0.0), vec2<f32>(0.0,  0.0), vec2<f32>(1.0,  0.0),
        vec2<f32>(-1.0,  1.0), vec2<f32>(0.0,  1.0), vec2<f32>(1.0,  1.0)
    );
    
    let weights = array<f32, 9>(
        1.0, 2.0, 1.0,
        2.0, 4.0, 2.0,
        1.0, 2.0, 1.0
    );
    
    var total_weight = 0.0;
    for (var i = 0; i < 9; i++) {
        let offset = offsets[i] * texel_size;
        let sample_uv = input.uv + offset;
        color += textureSample(prev_frame, frame_sampler, sample_uv) * weights[i];
        total_weight += weights[i];
    }
    
    return color / total_weight;
}

// Sharpen pass
@fragment
fn fs_sharpen(input: VertexOutput) -> @location(0) vec4<f32> {
    let texel_size = 1.0 / uniforms.resolution;
    
    let center = textureSample(prev_frame, frame_sampler, input.uv);
    let left = textureSample(prev_frame, frame_sampler, input.uv + vec2<f32>(-texel_size.x, 0.0));
    let right = textureSample(prev_frame, frame_sampler, input.uv + vec2<f32>(texel_size.x, 0.0));
    let up = textureSample(prev_frame, frame_sampler, input.uv + vec2<f32>(0.0, -texel_size.y));
    let down = textureSample(prev_frame, frame_sampler, input.uv + vec2<f32>(0.0, texel_size.y));
    
    // Sharpen kernel
    let sharpened = center * 5.0 - (left + right + up + down);
    
    return clamp(sharpened, vec4<f32>(0.0), vec4<f32>(1.0));
}
