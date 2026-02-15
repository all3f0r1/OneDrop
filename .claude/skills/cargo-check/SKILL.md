---
name: cargo-check
description: Run cargo validation (check, clippy, test) across the OneDrop workspace
arguments:
  - name: mode
    description: "check | clippy | test | all"
    required: false
    default: "clippy"
  - name: crate
    description: "Specific crate to check (e.g., onedrop-engine). Default: all crates"
    required: false
    default: ""
---

# Cargo Check Skill

Run cargo validation commands across the OneDrop Rust workspace.

## Usage

```
/cargo-check              # Run clippy on all crates (default)
/cargo-check check        # Run cargo check on all crates
/cargo-check test         # Run tests on all crates
/cargo-check all          # Run check, clippy, and test
/cargo-check clippy onedrop-engine  # Run clippy on specific crate
```

## Commands by Mode

### check
```bash
cargo check --all --all-features
```

### clippy (default)
```bash
cargo clippy --all --all-targets --all-features -- -D warnings
```

### test
```bash
cargo test --all --all-features
```

### all
```bash
cargo check --all --all-features && \
cargo clippy --all --all-targets --all-features -- -D warnings && \
cargo test --all --all-features
```

## Specific Crate

When a crate name is provided, replace `--all` with `-p <crate>`:
```bash
cargo clippy -p onedrop-engine --all-targets -- -D warnings
```

## Notes

- The workspace has 8 crates: onedrop-hlsl, onedrop-codegen, onedrop-parser, onedrop-eval, onedrop-renderer, onedrop-engine, onedrop-cli, onedrop-gui
- Some crates have optional features (e.g., `audio-input` for onedrop-engine)
- Tests include integration tests in `tests/` directories
