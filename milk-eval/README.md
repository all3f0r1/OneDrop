# milk-eval

Expression evaluator for Milkdrop per-frame and per-pixel equations.

## Overview

This crate provides a complete expression evaluation engine for Milkdrop visualization presets. It supports all Milkdrop variables, mathematical functions, and equation types.

## Features

- **Complete variable support** - All Milkdrop built-in variables (time, bass, x, y, etc.)
- **User variables** - q1-q64 for preset-defined values
- **Mathematical functions** - sin, cos, sqrt, abs, and more
- **Per-frame equations** - Execute once per frame
- **Per-pixel equations** - Execute for each pixel
- **Fast evaluation** - Optimized for real-time performance

## Usage

### Simple evaluation

```rust
use milk_eval::eval_simple;

let result = eval_simple("2 + 2")?;
assert_eq!(result, 4.0);
```

### With context

```rust
use milk_eval::MilkEvaluator;

let mut eval = MilkEvaluator::new();

// Set time variable
eval.context_mut().set_time(1.5);

// Evaluate expression
let result = eval.eval("sin(time * 3.14159)")?;
```

### Per-frame equations

```rust
use milk_eval::MilkEvaluator;

let mut eval = MilkEvaluator::new();

// Set audio data
eval.context_mut().set_audio(0.5, 0.3, 0.7, 0.4, 0.2, 0.6);

// Evaluate per-frame equations
let equations = vec![
    "wave_r = 0.5 + 0.4 * bass".to_string(),
    "wave_g = 0.3 + 0.2 * mid".to_string(),
    "wave_b = 0.7 + 0.3 * treb".to_string(),
    "zoom = 1.0 + 0.1 * bass_att".to_string(),
];

eval.eval_per_frame(&equations)?;

// Access updated variables
let zoom = eval.context().get_var("zoom").unwrap();
```

### Per-pixel equations

```rust
use milk_eval::MilkEvaluator;

let mut eval = MilkEvaluator::new();
eval.context_mut().set_var("zoom", 1.0);

let equations = vec![
    "zoom = zoom + 0.1 * rad".to_string(),
];

// Evaluate for a specific pixel
eval.eval_per_pixel(0.5, 0.5, 0.5, 0.0, &equations)?;
```

## Supported Variables

### Time Variables
- `time` - Current time in seconds
- `frame` - Current frame number
- `fps` - Frames per second

### Audio Variables
- `bass`, `mid`, `treb` - Current audio levels (0-1)
- `bass_att`, `mid_att`, `treb_att` - Attenuated audio levels

### Geometric Variables (per-pixel)
- `x`, `y` - Pixel coordinates (0-1)
- `rad` - Distance from center
- `ang` - Angle from center

### Motion Parameters
- `zoom` - Zoom factor
- `rot` - Rotation angle
- `cx`, `cy` - Center position
- `dx`, `dy` - Translation
- `sx`, `sy` - Stretch factors
- `warp` - Warp amount

### Wave Parameters
- `wave_r`, `wave_g`, `wave_b`, `wave_a` - Wave colors
- `wave_x`, `wave_y` - Wave position

### User Variables
- `q1` through `q64` - User-defined variables

## Supported Functions

- **Trigonometric**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
- **Exponential**: `exp`, `ln`, `log`, `log2`, `log10`
- **Power**: `pow`, `sqrt`
- **Rounding**: `floor`, `ceil`, `round`
- **Other**: `abs`, `min`, `max`

## Performance

The evaluator is optimized for real-time use:
- Variables are stored efficiently
- Expression parsing is done once
- Minimal allocations during evaluation

For per-pixel equations (executed millions of times per second), consider:
- Keeping expressions simple
- Pre-computing constants in per-frame equations
- Using q variables for shared values

## Testing

```bash
cargo test
cargo test -- --nocapture  # Show output
```

## License

MIT

## Part of OneDrop

This crate is part of the OneDrop project, a pure-Rust reimplementation of the Milkdrop music visualizer.
