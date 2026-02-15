# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in OneDrop, please report it by:

1. **Do not open a public issue** - Security vulnerabilities should be reported privately
2. Email the maintainer at security@onedrop.example.com (replace with actual email)
3. Include the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if available)

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Fix Development**: Depends on severity
- **Disclosure**: After fix is released

## Security Considerations

### Input Validation

OneDrop processes `.milk` preset files from external sources. The following security measures are in place:

- **File Size Limits**: Maximum 10MB per preset file
- **Expression Length Limits**: Maximum 100KB per expression
- **No Arbitrary Code Execution**: Expression evaluation uses a sandboxed evaluator

### Memory Safety

OneDrop is written in Rust, which provides memory safety guarantees:

- No buffer overflows
- No use-after-free
- No null pointer dereferences
- No data races in safe code

### Attack Surface

The primary attack surface is:

1. **Preset File Parsing** (`onedrop-parser`)
   - Input validation via file size limits
   - Safe string handling

2. **Expression Evaluation** (`onedrop-eval`)
   - Length limits prevent DoS
   - Sandboxed evaluator (evalexpr)
   - No file system or network access

3. **GPU Rendering** (`onedrop-renderer`)
   - Uses wgpu with default limits
   - No user-controlled buffer sizes

### Known Limitations

- No path canonicalization for preset files (potential directory traversal)
- No checksum verification for preset files
- No signature verification for downloaded presets

## Security Best Practices for Users

1. Only load preset files from trusted sources
2. Keep your Rust toolchain updated
3. Run with minimal privileges
4. Review preset files before loading if from untrusted sources

## Dependency Security

We monitor dependencies for known vulnerabilities using `cargo audit`. Dependencies are kept up to date with security patches.

## Disclosure Policy

When a security vulnerability is fixed:

1. A GitHub Security Advisory will be published
2. The fix will be included in the next release
3. CVE will be requested if appropriate
4. Credits will be given to the reporter (unless they wish to remain anonymous)
