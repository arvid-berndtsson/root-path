# Publishing Guide

This document explains how to publish `cc-check` to multiple package registries for cross-platform, multi-language distribution.

## Overview

`cc-check` is published to:
1. **crates.io** - For Rust users (`cargo install cc-check`)
2. **npm** - For Node.js/JavaScript users (`npm install @arvid-berndtsson/cc-check`)
3. **PyPI** - For Python users (`pip install cc-check`)
4. **GitHub Releases** - Pre-built binaries for all platforms

## Prerequisites

### 1. GitHub Secrets

Add these secrets to your GitHub repository (`Settings` → `Secrets and variables` → `Actions`):

- `CARGO_REGISTRY_TOKEN` - crates.io API token
- `NPM_TOKEN` - npm access token (for publishing)
- `PYPI_API_TOKEN` - PyPI API token

### 2. Package Registry Accounts

- **crates.io**: Create account at https://crates.io, generate API token
- **npm**: Create account at https://www.npmjs.com, generate access token
- **PyPI**: Create account at https://pypi.org, generate API token

## Publishing Process

### Step 1: Update Version

Update the version in:
- `Cargo.toml` (for crates.io)
- `package.json` (for npm)
- `pyproject.toml` (for PyPI)

Use semantic versioning (e.g., `0.1.0`, `0.2.0`, `1.0.0`).

### Step 2: Create Release Commit

```bash
# Update versions in all files
# Then commit and tag
git add Cargo.toml package.json pyproject.toml
git commit -m "chore(release): v0.1.0"
git tag v0.1.0
git push origin main --tags
```

### Step 3: Automated Publishing

When you push a tag starting with `v`, GitHub Actions will:

1. **Build binaries** for all platforms:
   - Linux (x86_64, ARM64)
   - macOS (x86_64, ARM64)
   - Windows (x86_64)

2. **Create GitHub Release** with all binaries

3. **Publish to registries**:
   - crates.io (Rust)
   - npm (Node.js)
   - PyPI (Python)

## Manual Publishing (Alternative)

If you prefer to publish manually:

### crates.io

```bash
cargo login <your-token>
cargo publish
```

### npm

```bash
npm login
npm publish
```

### PyPI

```bash
python -m pip install build twine
python -m build
twine upload dist/*
```

## Package-Specific Notes

### npm Package

The npm package includes:
- Pre-built binaries in `bin/` directory
- Post-install script to download correct binary for platform
- Platform detection for Windows/Linux/macOS

**Note**: You'll need to implement the postinstall script to download binaries from GitHub Releases.

### PyPI Package

The PyPI package uses:
- `pyproject.toml` for modern Python packaging
- Platform-specific wheels with embedded binaries
- Support for Python 3.8+

**Note**: You'll need to create platform-specific wheels with embedded binaries.

### crates.io

The Rust package is straightforward:
- Just publish the crate
- Users can install with `cargo install cc-check`
- Binary is automatically available in `~/.cargo/bin/`

## Cross-Compilation

The GitHub Actions workflow builds for multiple targets:

```bash
# Linux
x86_64-unknown-linux-gnu
aarch64-unknown-linux-gnu

# macOS
x86_64-apple-darwin
aarch64-apple-darwin

# Windows
x86_64-pc-windows-msvc
```

## Testing Before Publishing

1. **Test locally**:
   ```bash
   cargo test
   cargo build --release
   ```

2. **Test dry-run publish**:
   ```bash
   cargo publish --dry-run
   npm pack  # Creates tarball without publishing
   python -m build  # Builds without uploading
   ```

3. **Test on CI**: Push to a branch and verify CI passes

## Version Management

Follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

Update all version numbers in:
- `Cargo.toml`
- `package.json`
- `pyproject.toml`

## Troubleshooting

### crates.io

- **Error**: "crate already exists" - Version already published, bump version
- **Error**: "invalid token" - Check `CARGO_REGISTRY_TOKEN` secret

### npm

- **Error**: "package name taken" - Choose different name or use scoped package (`@your-org/cc-check`)
- **Error**: "unauthorized" - Check `NPM_TOKEN` secret

### PyPI

- **Error**: "File already exists" - Version already published, bump version
- **Error**: "invalid credentials" - Check `PYPI_API_TOKEN` secret

## Next Steps

1. Set up GitHub Secrets
2. Create accounts on all registries
3. Test the workflow with a pre-release version (e.g., `0.1.0-alpha.1`)
4. Publish first stable release

