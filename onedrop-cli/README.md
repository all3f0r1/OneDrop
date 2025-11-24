# onedrop-cli

Command-line interface for OneDrop Milkdrop visualizer.

## Overview

`onedrop-cli` is a command-line tool for working with Milkdrop presets. It allows you to inspect, validate, and render presets from the terminal.

## Installation

```bash
cd onedrop-cli
cargo install --path .
```

Or run directly:

```bash
cargo run -- <command>
```

## Usage

### Show preset information

```bash
onedrop info preset.milk
```

Output:
```
=== Preset Information ===

Version: 201
Warp shader version: 2
Composite shader version: 2

--- Parameters ---
Zoom: 0.99197
Rotation: 0.0
Decay: 0.98
Wave color: R=1.0, G=0.0, B=0.0

--- Equations ---
Per-frame equations: 6
Per-pixel equations: 6
...
```

### Validate a preset

```bash
onedrop validate preset.milk
```

Output:
```
✓ Preset is valid!
  Version: 201
  Per-frame equations: 6
  Per-pixel equations: 6
```

### Render frames

```bash
onedrop render preset.milk --frames 120 --output output/ --width 1920 --height 1080
```

Options:
- `--frames, -f` - Number of frames to render (default: 60)
- `--output, -o` - Output directory (default: output/)
- `--width, -w` - Width in pixels (default: 1280)
- `--height, -H` - Height in pixels (default: 720)

### List presets in directory

```bash
onedrop list /path/to/presets/
```

Output:
```
=== Presets in /path/to/presets/ ===

Found 10 preset(s):

  1. preset1.milk
  2. preset2.milk
  3. preset3.milk
  ...
```

## Commands

| Command | Description |
|---------|-------------|
| `info` | Show detailed information about a preset |
| `validate` | Check if a preset is valid |
| `render` | Render frames from a preset |
| `list` | List all presets in a directory |

## Options

- `-v, --verbose` - Enable verbose logging (applies to all commands)

## Examples

### Inspect a preset

```bash
onedrop info ../test-presets/10.milk
```

### Validate multiple presets

```bash
for preset in presets/*.milk; do
    onedrop validate "$preset"
done
```

### Render HD video frames

```bash
onedrop render preset.milk \
    --frames 1800 \
    --width 1920 \
    --height 1080 \
    --output frames/
```

Then convert to video with ffmpeg:
```bash
ffmpeg -framerate 60 -i frames/frame_%04d.png -c:v libx264 output.mp4
```

### List and validate all presets

```bash
onedrop list presets/
onedrop validate presets/*.milk
```

## Development

Build:
```bash
cargo build
```

Run:
```bash
cargo run -- info preset.milk
```

Test:
```bash
cargo test
```

## License

MIT

## Part of OneDrop

This is the command-line interface for the OneDrop project, a pure-Rust reimplementation of the Milkdrop music visualizer.
