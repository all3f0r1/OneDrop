# Contributing to OneDrop

Thank you for your interest in contributing to OneDrop! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment. Be kind, constructive, and professional.

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later (2024 edition support required)
- **Git**: For version control
- **A GPU**: With Vulkan, Metal, DX12, or OpenGL support

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/OneDrop.git
   cd OneDrop
   git remote add upstream https://github.com/all3f0r1/OneDrop.git
   ```

## Development Setup

### Build the Project

```bash
# Build all crates
cargo build --all

# Build release (optimized)
cargo build --release --all
```

### Run Tests

```bash
# Run all tests
cargo test --all

# Run tests for a specific crate
cargo test -p onedrop-parser
```

### Run the Application

```bash
# Run GUI
cargo run --release -p onedrop-gui

# Run CLI
cargo run --release -p onedrop-cli -- --help
```

## How to Contribute

### Reporting Bugs

1. Check existing issues to avoid duplicates
2. Use the bug report template
3. Include:
   - Rust version (`rustc --version`)
   - OS and version
   - Steps to reproduce
   - Expected vs actual behavior
   - Logs or screenshots if relevant

### Suggesting Features

1. Check existing issues/discussions
2. Use the feature request template
3. Describe the feature and use case
4. Explain why it would benefit users

### Submitting Code

1. Pick an issue to work on (or create one)
2. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes
4. Add tests
5. Update documentation
6. Submit a pull request

## Pull Request Process

1. **Ensure all checks pass**:
   - `cargo test --all` passes
   - `cargo clippy --all -- -D warnings` passes
   - `cargo fmt --all -- --check` passes

2. **Update documentation**:
   - Update README.md if needed
   - Add inline documentation for new code
   - Update CLAUDE.md for architectural changes

3. **Write good commit messages**:
   ```
   type(scope): brief description

   Longer explanation if needed.

   Fixes #123
   ```

4. **Request review** from maintainers

5. **Address review feedback** promptly

## Coding Standards

### Rust Style

- Follow standard Rust formatting (`cargo fmt`)
- Address all Clippy warnings (`cargo clippy`)
- Use idiomatic Rust patterns

### Code Organization

- Each crate has a single responsibility
- Public APIs should be well-documented
- Error types use `thiserror`
- Internal errors use `anyhow` where appropriate

### Key Principles

1. **No `unwrap()` in production code** - Use `?` or proper error handling
2. **Safety comments required** for any `unsafe` blocks
3. **Documentation** for all public items
4. **Tests** for new functionality

### Error Handling

```rust
// Good: Proper error handling
let value = some_operation()
    .map_err(|e| EngineError::OperationFailed(e.to_string()))?;

// Bad: unwrap in production
let value = some_operation().unwrap();
```

## Testing

### Unit Tests

- Place tests in the same file with `#[cfg(test)]`
- Use `#[test]` for synchronous tests
- Use `tokio::test` for async tests

### Integration Tests

- Place in `tests/` directory
- Test complete workflows

### Test Quality

- Test edge cases
- Test error conditions
- Use descriptive test names:
  ```rust
  #[test]
  fn test_parse_rejects_empty_input() { ... }
  ```

## Documentation

### Inline Documentation

```rust
/// Brief description.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// When this function returns an error
///
/// # Example
///
/// ```
/// let result = my_function(42);
/// ```
pub fn my_function(param: i32) -> Result<i32> { ... }
```

### Crate Documentation

Each crate should have a crate-level doc comment:

```rust
//! Crate description.
//!
//! More details about the crate's purpose and usage.
```

## Project Structure

```
OneDrop/
├── onedrop-parser/    # Preset file parsing
├── onedrop-eval/      # Expression evaluation
├── onedrop-renderer/  # GPU rendering
├── onedrop-engine/    # Main engine
├── onedrop-cli/       # Command-line interface
├── onedrop-gui/       # GUI application
├── onedrop-hlsl/      # HLSL translation
├── onedrop-codegen/   # Code generation
└── CLAUDE.md          # Project context
```

## Questions?

Feel free to open an issue with the "question" label or start a discussion on GitHub.

Thank you for contributing to OneDrop!
