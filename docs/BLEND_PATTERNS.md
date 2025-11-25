# OneDrop Blend Patterns Guide

Complete reference for all 27 blending patterns available in OneDrop v0.4.0.

## Overview

OneDrop supports 27 different blending patterns for double-preset visualization, inspired by MilkDrop3. Each pattern creates unique visual effects by combining two presets in different ways.

## Pattern Categories

### Basic Blends (0-4)

#### 0. Alpha
**Description**: Simple transparency blending  
**Formula**: `mix(A, B, blend_amount)`  
**Best for**: Smooth transitions between presets  
**Visual**: Gradual fade from preset A to preset B

#### 1. Additive
**Description**: Add colors together  
**Formula**: `A + B * blend_amount`  
**Best for**: Bright, glowing effects  
**Visual**: Colors become brighter, can create bloom

#### 2. Multiply
**Description**: Multiply color values  
**Formula**: `mix(A, A * B, blend_amount)`  
**Best for**: Darkening, shadow effects  
**Visual**: Colors become darker, preserves dark areas

#### 3. Screen
**Description**: Inverse multiply  
**Formula**: `mix(A, 1 - (1-A) * (1-B), blend_amount)`  
**Best for**: Lightening, highlight effects  
**Visual**: Colors become lighter, preserves light areas

#### 4. Overlay
**Description**: Combination of multiply and screen  
**Formula**: Conditional based on pixel brightness  
**Best for**: Contrast enhancement  
**Visual**: Increases contrast, vibrant colors

### Advanced Blends (5-10)

#### 5. Darken
**Description**: Keep darker pixels from both presets  
**Formula**: `mix(A, min(A, B), blend_amount)`  
**Best for**: Shadow preservation  
**Visual**: Only darker colors show through

#### 6. Lighten
**Description**: Keep lighter pixels from both presets  
**Formula**: `mix(A, max(A, B), blend_amount)`  
**Best for**: Highlight preservation  
**Visual**: Only lighter colors show through

#### 7. Color Dodge
**Description**: Brighten preset A based on preset B  
**Formula**: `mix(A, A / (1 - B), blend_amount)`  
**Best for**: Extreme brightening, glow effects  
**Visual**: Very bright, can create blown-out areas

#### 8. Color Burn
**Description**: Darken preset A based on preset B  
**Formula**: `mix(A, 1 - (1-A) / B, blend_amount)`  
**Best for**: Extreme darkening, shadow effects  
**Visual**: Very dark, can create deep shadows

#### 9. Hard Light
**Description**: Strong contrast blend  
**Formula**: Overlay with presets swapped  
**Best for**: Dramatic effects  
**Visual**: High contrast, bold colors

#### 10. Soft Light
**Description**: Gentle contrast blend  
**Formula**: Softened version of hard light  
**Best for**: Subtle enhancement  
**Visual**: Gentle contrast increase

### Difference Blends (11-12)

#### 11. Difference
**Description**: Absolute difference between colors  
**Formula**: `mix(A, abs(A - B), blend_amount)`  
**Best for**: Psychedelic effects  
**Visual**: Inverted colors, high contrast

#### 12. Exclusion
**Description**: Soft difference  
**Formula**: `mix(A, A + B - 2*A*B, blend_amount)`  
**Best for**: Subtle color inversion  
**Visual**: Softer than difference, pastel-like

### Pattern Blends (13-17)

#### 13. Plasma ⭐
**Description**: Animated organic plasma pattern  
**Formula**: `sin(x) + sin(y) + sin((x+y)/2) + time`  
**Best for**: Organic, flowing transitions  
**Visual**: Smooth waves, cellular appearance  
**Animation**: Flows continuously

#### 14. Snail
**Description**: Spiral pattern from center  
**Formula**: Based on angle and radius from center  
**Best for**: Rotating transitions  
**Visual**: Spiral arms radiating outward  
**Animation**: Rotates slowly

#### 15. Triangle
**Description**: Triangular tessellation  
**Formula**: Geometric triangle calculation  
**Best for**: Sharp, geometric transitions  
**Visual**: Triangular tiles  
**Animation**: Tiles shift

#### 16. Donuts
**Description**: Concentric circular rings  
**Formula**: Based on distance from center  
**Best for**: Radial transitions  
**Visual**: Circular rings expanding outward  
**Animation**: Rings pulse

#### 17. Checkerboard
**Description**: Classic checkerboard pattern  
**Formula**: `(floor(x) + floor(y)) % 2`  
**Best for**: Retro, pixelated effects  
**Visual**: Alternating squares  
**Animation**: Squares flip

### Stripe Blends (18-20)

#### 18. Horizontal Stripes
**Description**: Horizontal bands  
**Formula**: Based on Y coordinate  
**Best for**: Scanline effects  
**Visual**: Horizontal lines across screen  
**Animation**: Lines scroll vertically

#### 19. Vertical Stripes
**Description**: Vertical bands  
**Formula**: Based on X coordinate  
**Best for**: Curtain effects  
**Visual**: Vertical lines across screen  
**Animation**: Lines scroll horizontally

#### 20. Diagonal Stripes
**Description**: Diagonal bands  
**Formula**: Based on X + Y coordinate  
**Best for**: Dynamic transitions  
**Visual**: Diagonal lines at 45°  
**Animation**: Lines scroll diagonally

### Geometric Blends (21-22)

#### 21. Radial
**Description**: Radial gradient from center  
**Formula**: Based on distance from center  
**Best for**: Spotlight effects  
**Visual**: Circular gradient  
**Animation**: Gradient pulses

#### 22. Angular
**Description**: Rotating angular pattern  
**Formula**: Based on angle from center  
**Best for**: Spinning transitions  
**Visual**: Pie-slice pattern  
**Animation**: Rotates continuously

### Noise Blends (23-24)

#### 23. Perlin Noise
**Description**: Organic noise pattern  
**Formula**: Simplified Perlin noise algorithm  
**Best for**: Natural, organic transitions  
**Visual**: Cloud-like, flowing shapes  
**Animation**: Flows smoothly

#### 24. Voronoi
**Description**: Cellular pattern  
**Formula**: Distance to nearest point  
**Best for**: Crystal, cellular effects  
**Visual**: Irregular cells  
**Animation**: Cells shift

### Dynamic Blends (25-26)

#### 25. Wave
**Description**: Sine wave pattern  
**Formula**: `sin(x * frequency + time)`  
**Best for**: Rhythmic transitions  
**Visual**: Horizontal waves  
**Animation**: Waves flow

#### 26. Random Pixel
**Description**: Random pixel-by-pixel blend  
**Formula**: Random noise per pixel  
**Best for**: Glitch effects  
**Visual**: Random noise  
**Animation**: Constantly changing

## Usage Examples

### Basic Usage

```rust
use onedrop_parser::{DoublePreset, BlendPattern};

let double = DoublePreset::new(preset_a, preset_b)
    .with_pattern(BlendPattern::Plasma)
    .with_blend_amount(0.5)
    .with_animation(1.0);
```

### Rendering

```rust
use onedrop_renderer::BlendRenderer;

blend_renderer.render(
    &texture_a_view,
    &texture_b_view,
    &output_view,
    BlendPattern::Plasma as u32,  // Pattern index
    0.5,                           // Blend amount (0.0-1.0)
    time,                          // Time for animation
)?;
```

### Creating .od2 Files

```ini
[DoublePreset]
BlendPattern=13          # 13 = Plasma
BlendAmount=0.5          # 50% blend
AnimateBlend=1           # Enable animation
AnimationSpeed=1.0       # Normal speed

[PresetA]
MILKDROP_PRESET_VERSION=201
...

[PresetB]
MILKDROP_PRESET_VERSION=201
...
```

## Pattern Selection Guide

### For Smooth Transitions
- **Alpha** (0) - Classic fade
- **Plasma** (13) - Organic flow
- **Perlin Noise** (23) - Natural blend

### For Dramatic Effects
- **Color Dodge** (7) - Extreme brightness
- **Color Burn** (8) - Extreme darkness
- **Difference** (11) - Color inversion

### For Geometric Effects
- **Triangle** (15) - Sharp edges
- **Checkerboard** (17) - Retro style
- **Voronoi** (24) - Cellular pattern

### For Rhythmic Effects
- **Wave** (25) - Flowing rhythm
- **Horizontal Stripes** (18) - Scanlines
- **Angular** (22) - Spinning

## Performance Notes

- All patterns run at **60 FPS** at 1920x1080
- GPU-accelerated via WGSL shaders
- **< 2ms** per frame overhead
- No CPU-side processing required

## Animation Speed

- `0.0` - Static (no animation)
- `0.5` - Slow animation
- `1.0` - Normal speed (default)
- `2.0` - Fast animation
- `5.0` - Very fast animation

## Blend Amount

- `0.0` - 100% Preset A
- `0.25` - 75% A, 25% B
- `0.5` - 50/50 blend (default)
- `0.75` - 25% A, 75% B
- `1.0` - 100% Preset B

## Tips and Tricks

### Creating Smooth Transitions
1. Start with **Alpha** (0) at `blend_amount = 0.0`
2. Gradually increase to `1.0` over time
3. Switch to preset B completely

### Creating Psychedelic Effects
1. Use **Plasma** (13) or **Perlin Noise** (23)
2. Set `animation_speed = 2.0` or higher
3. Use complementary color presets

### Creating Glitch Effects
1. Use **Random Pixel** (26)
2. Rapidly change `blend_amount`
3. Combine with **Difference** (11)

### Creating Rhythmic Effects
1. Use **Wave** (25) or **Horizontal Stripes** (18)
2. Sync `animation_speed` to BPM
3. Modulate `blend_amount` with bass

## Technical Implementation

All patterns are implemented in `onedrop-renderer/shaders/blend.wgsl`:

```wgsl
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color_a = textureSample(texture_a, texture_sampler, input.uv);
    let color_b = textureSample(texture_b, texture_sampler, input.uv);
    
    var result: vec4<f32>;
    
    switch (uniforms.blend_pattern) {
        case 0u: { result = blend_alpha(color_a, color_b, uniforms.blend_amount); }
        case 13u: { 
            let t = plasma_pattern(input.uv, uniforms.time);
            result = mix(color_a, color_b, t * uniforms.blend_amount);
        }
        // ... 25 more patterns
    }
    
    return result;
}
```

## Comparison with MilkDrop3

| Feature | MilkDrop3 | OneDrop |
|---------|-----------|---------|
| Pattern count | 27 | 27 ✅ |
| GPU rendering | ❌ DX9 | ✅ wgpu |
| Animation | ✅ | ✅ |
| Cross-platform | ❌ Windows | ✅ All |
| Performance | 60 FPS | 60 FPS ✅ |

## Future Enhancements

### Planned for v0.5.0
- Custom pattern creation
- Pattern morphing
- Audio-reactive patterns
- User-defined blend functions

### Planned for v1.0.0
- Pattern presets library
- Visual pattern editor
- Real-time pattern preview
- Community pattern sharing

## License

MIT - See LICENSE file for details

---

*OneDrop v0.4.0 - Pure Rust Milkdrop Visualizer*
