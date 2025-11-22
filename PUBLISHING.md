# Publishing Guide

This document explains how to publish `cc-check` to multiple package registries for cross-platform, multi-language distribution.

## Overview

`cc-check` is published to:
1. **crates.io** - For Rust users (`cargo install cc-check`)
2. **npm** - For Node.js/JavaScript users (`npm install @arvid-berndtsson/cc-check`)
3. **PyPI** - For Python users (`pip install cc-check`)
4. **GitHub Releases** - Pre-built binaries for all platforms

## Prerequisites

### 1. Trusted Publishing Setup (Recommended)

This project uses **Trusted Publishing** for crates.io, npm, and PyPI, which eliminates the need for long-lived API tokens and significantly improves security. **No GitHub secrets are required for publishing!**

#### crates.io Trusted Publishing

1. **Publish your crate manually first** (required before setting up Trusted Publishing):
   ```bash
   cargo login <your-token>
   cargo publish
   ```

2. **Link your GitHub repository**:
   - Go to your crate page on [crates.io](https://crates.io)
   - Navigate to **Settings** → **Repository**
   - Link your GitHub repository: `arvid-berndtsson/cc-check`

3. **Configure Trusted Publisher**:
   - Go to your crate's **Settings** → **Trusted Publishers**
   - Click **Add Trusted Publisher**
   - Configure:
     - **GitHub Organization/User**: `arvid-berndtsson`
     - **Repository Name**: `cc-check`
     - **Workflow Filename**: `release.yml`
     - **Environment Name**: (leave empty unless using GitHub Environments)
   - Click **Add**

4. **Remove `CARGO_REGISTRY_TOKEN` secret** from GitHub (no longer needed)

#### npm Trusted Publishing

1. **Go to npm Trusted Publishers**:
   - Visit: https://www.npmjs.com/settings/arvid-berndtsson/trusted-publishers
   - Or navigate: npm → Your Profile → Access Tokens → Trusted Publishers

2. **Add a new trusted publisher**:
   - Click **Add Trusted Publisher**
   - Configure:
     - **Publisher name**: `github-actions` (or any name you prefer)
     - **Workflow repository**: `arvid-berndtsson/cc-check`
     - **Workflow file**: `.github/workflows/release.yml`
     - **Environment**: (leave empty unless using GitHub Environments)
   - Click **Add**

3. **Verify the trusted publisher is active**:
   - The publisher should show as "Active" in the list
   - If it shows "Pending", wait a few minutes for it to activate

4. **Verify Trusted Publisher is active**:
   - The publisher should show as "Active" in the list
   - If it shows "Pending", wait a few minutes for it to activate
   - Ensure the workflow file path matches exactly: `.github/workflows/release.yml`

5. **Troubleshooting 404 errors**:
   - If you get a 404 error when publishing, verify:
     - Trusted Publisher is "Active" (not "Pending")
     - Workflow file path matches exactly (including `.github/` prefix)
     - Repository name matches exactly: `arvid-berndtsson/cc-check`
     - The workflow has `id-token: write` permission
   - For scoped packages, ensure `--access=public` is included in publish command
   - If package already exists on npm, the 404 might indicate authentication issue

6. **Remove `NPM_TOKEN` secret** from GitHub (no longer needed)

#### PyPI Trusted Publishing

1. **Go to PyPI Trusted Publishers**:
   - Visit: https://pypi.org/manage/account/publishing/
   - Or navigate: PyPI → Your Profile → Account settings → Publishing

2. **Add a new trusted publisher**:
   - Click **Add a new pending publisher**
   - Select the **GitHub** tab
   - Fill in:
     - **PyPI Project Name**: `cc-check`
     - **Owner**: `arvid-berndtsson`
     - **Repository Name**: `cc-check`
     - **Workflow Filename**: `.github/workflows/release.yml`
     - **Environment Name**: (leave empty unless using GitHub Environments)
   - Click **Add**

3. **Remove `PYPI_API_TOKEN` secret** from GitHub (no longer needed)

### 2. Package Registry Accounts

- **crates.io**: Create account at https://crates.io (for initial manual publish)
- **npm**: Create account at https://www.npmjs.com
- **PyPI**: Create account at https://pypi.org

### 3. GitHub Secrets

**No secrets required!** All publishing uses Trusted Publishing with OIDC authentication.

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
# Only needed for initial manual publish before setting up Trusted Publishing
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
- **Error**: "authentication failed" - Verify Trusted Publisher is configured correctly in crate settings
- **Error**: "repository not linked" - Link your GitHub repository in crate settings

### npm

- **Error**: "package name taken" - Choose different name or use scoped package (`@your-org/cc-check`)
- **Error**: "unauthorized" - Verify Trusted Publisher is configured correctly in npm settings
- **Error**: "OTP required" - Trusted Publishing should eliminate this; verify OIDC setup

### PyPI

- **Error**: "File already exists" - Version already published, bump version
- **Error**: "authentication failed" - Verify Trusted Publisher is configured correctly in PyPI settings
- **Error**: "publisher not found" - Ensure the trusted publisher is added and approved in PyPI

## Next Steps

1. Set up Trusted Publishing for crates.io and npm (see Prerequisites above)
2. Add `PYPI_API_TOKEN` secret to GitHub (only secret needed)
3. Create accounts on all registries
4. Test the workflow with a pre-release version (e.g., `0.1.0-alpha.1`)
5. Publish first stable release

