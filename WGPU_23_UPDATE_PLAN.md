# OneDrop wgpu 23 Update Plan

**Date**: November 25, 2025  
**Goal**: Update OneDrop from wgpu 22.1 to wgpu 23.0 for OneAmp integration

---

## 📊 Current State

### Crates Using wgpu

| Crate | Current Version | Target Version |
|-------|----------------|----------------|
| `onedrop-engine` | 22.1 | 23.0 |
| `onedrop-renderer` | 22.1 | 23.0 |
| `onedrop-gui` | 22.1 | 23.0 |

### Dependency Tree

```
onedrop-engine
  └── onedrop-renderer (uses wgpu)
      └── wgpu 22.1

onedrop-gui
  └── wgpu 22.1
  └── onedrop-engine
```

---

## 🔧 Changes Required

### 1. Update Cargo.toml Files

**Files to modify**:
- `onedrop-engine/Cargo.toml`
- `onedrop-renderer/Cargo.toml`
- `onedrop-gui/Cargo.toml`

**Change**:
```toml
# Before
wgpu = "22.1"

# After
wgpu = "23.0"
```

### 2. API Changes in wgpu 23

Based on wgpu changelog, main breaking changes:

#### a) Texture Creation
```rust
// wgpu 22
let texture = device.create_texture(&wgpu::TextureDescriptor {
    // ...
});

// wgpu 23 (mostly compatible)
// No major changes expected
```

#### b) Surface Configuration
```rust
// wgpu 22
surface.configure(&device, &config);

// wgpu 23
surface.configure(&device, &config);
// Same API
```

#### c) RenderPass
```rust
// wgpu 22
let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // ...
});

// wgpu 23
// Same API, minor internal changes
```

---

## 📝 Step-by-Step Plan

### Phase 1: Update Dependencies ✅

1. Update `onedrop-renderer/Cargo.toml`
2. Update `onedrop-engine/Cargo.toml`
3. Update `onedrop-gui/Cargo.toml`

### Phase 2: Fix Compilation Errors

1. Run `cargo check` in onedrop-renderer
2. Fix any API incompatibilities
3. Run `cargo check` in onedrop-engine
4. Fix any API incompatibilities
5. Run `cargo check` in onedrop-gui (if needed)

### Phase 3: Test

1. Run `cargo test` in onedrop-renderer
2. Run `cargo test` in onedrop-engine
3. Test with OneAmp integration

### Phase 4: Document & Commit

1. Update CHANGELOG
2. Commit changes
3. Push to repository

---

## ⚠️ Potential Issues

### Issue 1: Surface API Changes

**Risk**: Medium  
**Impact**: Rendering may fail

**Mitigation**: Check wgpu 23 changelog for Surface API changes

### Issue 2: Shader Compatibility

**Risk**: Low  
**Impact**: Shaders may need updates

**Mitigation**: wgpu 23 uses same WGSL version

### Issue 3: Texture Format Changes

**Risk**: Low  
**Impact**: Texture creation may fail

**Mitigation**: Check TextureFormat enum changes

---

## 🧪 Testing Strategy

### Unit Tests

```bash
cd onedrop-renderer
cargo test

cd ../onedrop-engine
cargo test
```

### Integration Test

```bash
cd ../oneamp
cargo build --release
./target/release/oneamp

# Test:
# 1. Enable Milkdrop
# 2. Verify visual rendering appears
# 3. Navigate presets
# 4. Check FPS
```

---

## 📦 Expected Changes

### Cargo.lock

- All wgpu dependencies will update to 23.x
- wgpu-core, wgpu-hal, wgpu-types will update

### Code Changes

**Estimated**: Minimal (0-50 lines)

Most wgpu 23 changes are internal. Public API is mostly stable.

---

## 🎯 Success Criteria

- [ ] OneDrop compiles without errors
- [ ] OneDrop tests pass
- [ ] OneAmp compiles without errors
- [ ] OneAmp displays Milkdrop visualization
- [ ] Preset navigation works
- [ ] FPS is acceptable (30+)

---

## 📚 References

- [wgpu 23.0 Release Notes](https://github.com/gfx-rs/wgpu/releases/tag/v23.0.0)
- [wgpu Migration Guide](https://github.com/gfx-rs/wgpu/wiki/Migration-Guides)

---

**Ready to proceed!** 🚀
