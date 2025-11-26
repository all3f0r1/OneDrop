# onedrop-codegen

WGSL shader code generation from Milkdrop equations.

## Features

- **Expression Transpilation** - Convert Milkdrop equations to WGSL
- **Variable Mapping** - Map Milkdrop variables to WGSL uniforms
- **Shader Generation** - Generate complete per-pixel shaders
- **Validation** - Validate generated shaders with naga

## Usage

### Transpile Single Equation

```rust
use onedrop_codegen::transpile_equation;

let wgsl = transpile_equation("x = x + 0.01*sin(time)")?;
// Output: "vars.x = vars.x + 0.01*sin(vars.time);"
```

### Generate Complete Shader

```rust
use onedrop_codegen::ShaderGenerator;
use onedrop_parser::parse_preset;

let preset = parse_preset(&content)?;
let generator = ShaderGenerator::new();
let shader = generator.generate_per_pixel_shader(&preset)?;
```

## Variable Mapping

| Milkdrop | WGSL |
|----------|------|
| `x`, `y` | `vars.x`, `vars.y` |
| `rad`, `ang` | `vars.rad`, `vars.ang` |
| `bass`, `mid`, `treb` | `vars.bass`, `vars.mid`, `vars.treb` |
| `time`, `frame` | `vars.time`, `vars.frame` |
| `q1`-`q64` | `vars.q[0]`-`vars.q[63]` |

## Function Mapping

Most math functions are compatible:
- `sin`, `cos`, `tan` → Same in WGSL
- `sqrt`, `abs`, `pow` → Same in WGSL
- `min`, `max`, `clamp` → Same in WGSL

## Example

Input (Milkdrop):
```
per_pixel_1=x = x + 0.01*sin(time + y*10);
per_pixel_2=y = y + 0.01*cos(time + x*10);
```

Output (WGSL):
```wgsl
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var vars = vars;
    vars.x = input.uv.x;
    vars.y = input.uv.y;
    
    vars.x = vars.x + 0.01*sin(vars.time + vars.y*10.0);
    vars.y = vars.y + 0.01*cos(vars.time + vars.x*10.0);
    
    return color;
}
```

## License

MIT
