---
name: release-notes
description: Generate release notes from git commits since last tag
disable-model-invocation: true
arguments:
  - name: version
    description: "Version to release (e.g., 1.0.0). If not provided, uses next patch version."
    required: false
    default: ""
---

# Release Notes Skill

Generate release notes for OneDrop from git history.

## Usage

```
/release-notes           # Generate notes for next patch version
/release-notes 1.1.0     # Generate notes for specific version
```

## Steps

### 1. Get Current Version
```bash
git describe --tags --abbrev=0 2>/dev/null || echo "v0.2.0"
```

### 2. Get Commits Since Last Tag
```bash
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.2.0")
git log $LAST_TAG..HEAD --oneline --no-merges
```

### 3. Get Commit Details by Category
```bash
# Features
git log $LAST_TAG..HEAD --oneline --no-merges --grep="feat:" --grep="feature:" --grep="add:"

# Fixes
git log $LAST_TAG..HEAD --oneline --no-merges --grep="fix:" --grep="bugfix:" --grep="fix"

# Refactors
git log $LAST_TAG..HEAD --oneline --no-merges --grep="refactor:" --grep="refactor"

# Docs
git log $LAST_TAG..HEAD --oneline --no-merges --grep="docs:" --grep="doc:"
```

### 4. Update Version in Cargo.toml Files
Update version in all workspace member Cargo.toml files:
- Root Cargo.toml: `[workspace.package]` version
- Each crate: `[package]` version

### 5. Generate CHANGELOG Entry
Create `CHANGELOG_v{VERSION}.md` with:
- Summary of changes
- Breaking changes (if any)
- New features
- Bug fixes
- Internal changes

## Output Template

```markdown
# OneDrop v{VERSION}

Released: {DATE}

## Summary
{Brief summary of release}

## Breaking Changes
- {Any breaking changes}

## New Features
- {New features from feat: commits}

## Bug Fixes
- {Fixes from fix: commits}

## Internal
- {Refactors, dependencies, etc.}

## Compatibility
- Preset compatibility: {percentage}%
- Rust version: 1.70+

## Contributors
{List of contributors from commits}
```

## Notes

- Current roadmap versions: v0.7.0 (done), v0.8.0 (done), v0.9.0 (done), v1.0.0 (next)
- Workspace version is defined in root Cargo.toml `[workspace.package]`
- All 8 crates share the same version
