// Blend shader for double-preset rendering
// Implements 27 blending patterns inspired by MilkDrop3

struct BlendUniforms {
    blend_pattern: u32,
    blend_amount: f32,
    time: f32,
    _padding: f32,
}

@group(0) @binding(0) var texture_a: texture_2d<f32>;
@group(0) @binding(1) var texture_b: texture_2d<f32>;
@group(0) @binding(2) var texture_sampler: sampler;
@group(0) @binding(3) var<uniform> uniforms: BlendUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // Full-screen quad
    let x = f32((vertex_index & 1u) << 1u) - 1.0;
    let y = f32((vertex_index & 2u)) - 1.0;
    
    output.position = vec4<f32>(x, y, 0.0, 1.0);
    output.uv = vec2<f32>((x + 1.0) * 0.5, (1.0 - y) * 0.5);
    
    return output;
}

// Helper functions for blending

fn blend_alpha(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return mix(a, b, t);
}

fn blend_additive(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return a + b * t;
}

fn blend_multiply(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return mix(a, a * b, t);
}

fn blend_screen(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    let screen = vec4<f32>(1.0) - (vec4<f32>(1.0) - a) * (vec4<f32>(1.0) - b);
    return mix(a, screen, t);
}

fn blend_overlay(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    var result: vec4<f32>;
    for (var i = 0; i < 3; i++) {
        if (a[i] < 0.5) {
            result[i] = 2.0 * a[i] * b[i];
        } else {
            result[i] = 1.0 - 2.0 * (1.0 - a[i]) * (1.0 - b[i]);
        }
    }
    result.a = a.a;
    return mix(a, result, t);
}

fn blend_darken(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return mix(a, min(a, b), t);
}

fn blend_lighten(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return mix(a, max(a, b), t);
}

fn blend_color_dodge(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    let dodge = a / (vec4<f32>(1.0) - b + vec4<f32>(0.001));
    return mix(a, dodge, t);
}

fn blend_color_burn(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    let burn = vec4<f32>(1.0) - (vec4<f32>(1.0) - a) / (b + vec4<f32>(0.001));
    return mix(a, burn, t);
}

fn blend_hard_light(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return blend_overlay(b, a, t);
}

fn blend_soft_light(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    let soft = (1.0 - 2.0 * b) * a * a + 2.0 * b * a;
    return mix(a, soft, t);
}

fn blend_difference(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return mix(a, abs(a - b), t);
}

fn blend_exclusion(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    let excl = a + b - 2.0 * a * b;
    return mix(a, excl, t);
}

// Pattern-based blending functions

fn plasma_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let x = uv.x * 10.0;
    let y = uv.y * 10.0;
    return (sin(x + time) + sin(y + time) + sin((x + y) * 0.5 + time)) * 0.333 + 0.5;
}

fn snail_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let center = vec2<f32>(0.5, 0.5);
    let delta = uv - center;
    let angle = atan2(delta.y, delta.x);
    let radius = length(delta);
    return fract(angle / (2.0 * 3.14159) + radius * 5.0 + time * 0.1);
}

fn triangle_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let x = fract(uv.x * 10.0 + time * 0.1);
    let y = fract(uv.y * 10.0);
    return select(x, 1.0 - x, (x + y) > 1.0);
}

fn donuts_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let center = vec2<f32>(0.5, 0.5);
    let dist = length(uv - center);
    return fract(dist * 10.0 + time * 0.1);
}

fn checkerboard_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let x = floor(uv.x * 10.0);
    let y = floor(uv.y * 10.0);
    return select(0.0, 1.0, ((x + y) % 2.0) < 1.0);
}

fn horizontal_stripes_pattern(uv: vec2<f32>, time: f32) -> f32 {
    return fract(uv.y * 10.0 + time * 0.1);
}

fn vertical_stripes_pattern(uv: vec2<f32>, time: f32) -> f32 {
    return fract(uv.x * 10.0 + time * 0.1);
}

fn diagonal_stripes_pattern(uv: vec2<f32>, time: f32) -> f32 {
    return fract((uv.x + uv.y) * 10.0 + time * 0.1);
}

fn radial_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let center = vec2<f32>(0.5, 0.5);
    return length(uv - center) * 2.0;
}

fn angular_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let center = vec2<f32>(0.5, 0.5);
    let delta = uv - center;
    let angle = atan2(delta.y, delta.x);
    return (angle + 3.14159 + time * 0.5) / (2.0 * 3.14159);
}

fn perlin_noise_pattern(uv: vec2<f32>, time: f32) -> f32 {
    // Simplified Perlin noise
    let p = floor(uv * 10.0);
    let f = fract(uv * 10.0);
    let u = f * f * (3.0 - 2.0 * f);
    
    let a = sin(dot(p, vec2<f32>(12.9898, 78.233)) + time) * 43758.5453;
    let b = sin(dot(p + vec2<f32>(1.0, 0.0), vec2<f32>(12.9898, 78.233)) + time) * 43758.5453;
    let c = sin(dot(p + vec2<f32>(0.0, 1.0), vec2<f32>(12.9898, 78.233)) + time) * 43758.5453;
    let d = sin(dot(p + vec2<f32>(1.0, 1.0), vec2<f32>(12.9898, 78.233)) + time) * 43758.5453;
    
    return mix(mix(fract(a), fract(b), u.x), mix(fract(c), fract(d), u.x), u.y);
}

fn voronoi_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let p = floor(uv * 10.0);
    let f = fract(uv * 10.0);
    
    var min_dist = 10.0;
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let point = vec2<f32>(
                fract(sin(dot(p + neighbor, vec2<f32>(12.9898, 78.233)) + time) * 43758.5453),
                fract(sin(dot(p + neighbor, vec2<f32>(39.3468, 11.135)) + time) * 43758.5453)
            );
            let dist = length(neighbor + point - f);
            min_dist = min(min_dist, dist);
        }
    }
    return min_dist;
}

fn wave_pattern(uv: vec2<f32>, time: f32) -> f32 {
    return sin(uv.x * 10.0 + time) * 0.5 + 0.5;
}

fn random_pixel_pattern(uv: vec2<f32>, time: f32) -> f32 {
    let p = floor(uv * 100.0);
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233)) + time) * 43758.5453);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color_a = textureSample(texture_a, texture_sampler, input.uv);
    let color_b = textureSample(texture_b, texture_sampler, input.uv);
    
    var result: vec4<f32>;
    
    // Select blending mode based on pattern
    switch (uniforms.blend_pattern) {
        case 0u: { result = blend_alpha(color_a, color_b, uniforms.blend_amount); }
        case 1u: { result = blend_additive(color_a, color_b, uniforms.blend_amount); }
        case 2u: { result = blend_multiply(color_a, color_b, uniforms.blend_amount); }
        case 3u: { result = blend_screen(color_a, color_b, uniforms.blend_amount); }
        case 4u: { result = blend_overlay(color_a, color_b, uniforms.blend_amount); }
        case 5u: { result = blend_darken(color_a, color_b, uniforms.blend_amount); }
        case 6u: { result = blend_lighten(color_a, color_b, uniforms.blend_amount); }
        case 7u: { result = blend_color_dodge(color_a, color_b, uniforms.blend_amount); }
        case 8u: { result = blend_color_burn(color_a, color_b, uniforms.blend_amount); }
        case 9u: { result = blend_hard_light(color_a, color_b, uniforms.blend_amount); }
        case 10u: { result = blend_soft_light(color_a, color_b, uniforms.blend_amount); }
        case 11u: { result = blend_difference(color_a, color_b, uniforms.blend_amount); }
        case 12u: { result = blend_exclusion(color_a, color_b, uniforms.blend_amount); }
        case 13u: { 
            let t = plasma_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 14u: { 
            let t = snail_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 15u: { 
            let t = triangle_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 16u: { 
            let t = donuts_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 17u: { 
            let t = checkerboard_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 18u: { 
            let t = horizontal_stripes_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 19u: { 
            let t = vertical_stripes_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 20u: { 
            let t = diagonal_stripes_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 21u: { 
            let t = radial_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 22u: { 
            let t = angular_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 23u: { 
            let t = perlin_noise_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 24u: { 
            let t = voronoi_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 25u: { 
            let t = wave_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        case 26u: { 
            let t = random_pixel_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        default: { result = blend_alpha(color_a, color_b, uniforms.blend_amount); }
    }
    
    return result;
}
