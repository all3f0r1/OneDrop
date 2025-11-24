// Waveform shader for Milkdrop

struct Uniforms {
    resolution: vec2<f32>,
    time: f32,
    wave_scale: f32,
    wave_color: vec4<f32>,
    wave_mode: i32,
    _padding: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) value: f32,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Calculate waveform position
    let x = input.position.x * 2.0 - 1.0; // -1 to 1
    let y = input.value * uniforms.wave_scale;
    
    output.position = vec4<f32>(x, y, 0.0, 1.0);
    output.color = uniforms.wave_color;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
