# cc-check

A cross-platform Rust-based tool to validate git commit messages against the [Conventional Commits](https://www.conventionalcommits.org/) specification. This tool hooks into git's commit-msg hook to automatically validate commit messages.

**Works on Windows, Linux, and macOS** - The same installation and usage works across all platforms.

## Features

- ✅ Validates commit message format according to Conventional Commits specification
- ✅ Supports all standard commit types (feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
- ✅ Validates optional scope
- ✅ Detects breaking changes (via `!` or `BREAKING CHANGE:` footer)
- ✅ Checks description length (warns if > 72 characters)
- ✅ Integrates seamlessly with git pre-commit hooks
- ✅ Can read from stdin or file (works with git hooks)
- ✅ Configurable via command-line flags

## Installation

### Prerequisites

- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))
- Git installed

### Cross-Platform Support

`cc-check` works on **Windows**, **Linux**, and **macOS**. The installation process is the same on all platforms.

### Install the Git Hook

The easiest way to install the git hook is using the built-in install command:

```bash
# Build and install (recommended)
cargo run --release -- install

# Or if already built
cargo build --release
./target/release/cc-check install
```

This will:
- Build the Rust binary in release mode (unless `--no-build` is used)
- Install a cross-platform commit-msg hook in your `.git/hooks/` directory
- Backup any existing commit-msg hook
- Work on Windows, Linux, and macOS

**Note:** On Windows, the hook uses a `.bat` file if needed, but Git Bash (included with Git for Windows) can also run `.sh` hooks.

## Usage

### As a Git Hook (Automatic)

Once installed, the hook will automatically validate commit messages when you run `git commit`. Invalid messages will be rejected.

```bash
# ❌ This will be rejected
git commit -m "invalid commit message"

# ✅ This will pass
git commit -m "feat: add new feature"
```

### As a Standalone Tool

You can also use the checker directly:

```bash
# Check a commit message file
cc-check check .git/COMMIT_EDITMSG

# Or use backward-compatible syntax (file path as first argument)
cc-check .git/COMMIT_EDITMSG

# With JSON output
cc-check check --format json .git/COMMIT_EDITMSG

# With custom types
cc-check check --extra-types "wip,release" .git/COMMIT_EDITMSG
```

### Command-Line Flags

Use `cc-check check --help` to see all available flags:

- `--extra-types "wip,release"` - Add custom allowed types
- `--max-subject 0` - Disable subject length check
- `--no-trailing-period` - Disallow trailing period (default: true)
- `--format json` - Machine-readable output (`{"ok":true}` or `{ "ok": false, "error": "..." }`)
- `--allow-merge-commits` - Allow merge/revert message validation (default: true)

### Commit Message Format

The tool validates the following format:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `build`: Changes that affect the build system or external dependencies
- `ci`: Changes to CI configuration files and scripts
- `chore`: Other changes that don't modify src or test files
- `revert`: Reverts a previous commit

**Validation Rules:**
- Subject default max length: 72 chars
- Trailing period in subject is disallowed by default
- Merge/Revert messages are allowed by default

**Examples:**

```bash
# Simple commit
feat: add user authentication

# With scope
fix(api): correct error handling in endpoint

# Breaking change
feat!: change API structure

# With body
feat: add new feature

This feature adds support for user authentication
with JWT tokens.

# With breaking change footer
feat: add new API

BREAKING CHANGE: The old API endpoint is deprecated
```

## Pre-commit Integration

This repository includes a `.pre-commit-config.yaml` with hooks that run the same checks as CI:

1. **Rust Format Check** - Validates code formatting with `cargo fmt --all -- --check`
2. **Rust Clippy** - Runs linter with `cargo clippy --all-targets --all-features -- -D warnings`
3. **Conventional Commit Check** - Validates commit message format

Install and enable pre-commit hooks:

```bash
# Install all hooks (pre-commit and commit-msg)
pre-commit install
pre-commit install --hook-type commit-msg
```

The hooks will automatically:
- Check Rust code formatting before commits
- Run clippy linter to catch common issues
- Validate commit messages follow conventional commit format

**Note:** For best performance, build the release binaries:
```bash
cargo build --release
```

This ensures the hooks run quickly without needing to compile on each commit. The first run may be slower as it compiles the binaries, but subsequent runs will use the cached binaries.

If formatting fails, run `cargo fmt --all` to auto-format your code, then re-stage and commit.

## Building

```bash
# Debug build
cargo build

# Release build (recommended for hooks)
cargo build --release

# Run tests
cargo test
```

## Publishing

This project is configured for multi-language distribution. See [PUBLISHING.md](PUBLISHING.md) for complete publishing instructions.

### Quick Start

1. **Set up GitHub Secrets**:
   - `CARGO_REGISTRY_TOKEN` - for crates.io
   - `NPM_TOKEN` - for npm
   - `PYPI_API_TOKEN` - for PyPI

2. **Create a release**:
   ```bash
   # Update versions in Cargo.toml, package.json, pyproject.toml
   git commit -am "chore(release): v0.1.0"
   git tag v0.1.0
   git push origin main --tags
   ```

3. **GitHub Actions will automatically**:
   - Build binaries for Windows, Linux, macOS (x86_64 and ARM64)
   - Create GitHub Release with all binaries
   - Publish to crates.io (Rust)
   - Publish to npm (Node.js)
   - Publish to PyPI (Python)

See [PUBLISHING.md](PUBLISHING.md) for detailed instructions.

## Using from Different Language Ecosystems

`cc-check` can be used from any language ecosystem that can execute binaries:

### Rust

```toml
# Cargo.toml
[dependencies]
cc-check = "0.1.0"  # When published to crates.io
```

Or use as a binary:
```bash
cargo install cc-check
cc-check check .git/COMMIT_EDITMSG
```

### Node.js / npm

Install as a binary dependency:
```bash
npm install --save-dev cc-check
npx cc-check check .git/COMMIT_EDITMSG
```

Or use in package.json scripts:
```json
{
  "scripts": {
    "commit-msg": "cc-check check"
  }
}
```

### Python

Install via pip (when available):
```bash
pip install cc-check
cc-check check .git/COMMIT_EDITMSG
```

Or use as a pre-commit hook:
```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: cc-check
        name: cc-check
        entry: cc-check check
        language: system
        stages: [commit-msg]
```

### Go

Use as an external tool:
```go
package main

import (
    "os/exec"
)

func main() {
    cmd := exec.Command("cc-check", "check", ".git/COMMIT_EDITMSG")
    // ... handle output
}
```

### Deno

```typescript
const process = Deno.run({
  cmd: ["cc-check", "check", ".git/COMMIT_EDITMSG"],
});
await process.status();
```

**Note:** For multi-language distribution, you'll need to:
1. Build binaries for each platform (Windows, Linux, macOS)
2. Package them appropriately for each ecosystem
3. Publish to respective package registries (npm, PyPI, crates.io, etc.)

The core Rust implementation ensures consistent behavior across all platforms and language bindings.

## Uninstallation

To remove the hook:

```bash
rm .git/hooks/commit-msg
```

If you backed up a previous hook, you can restore it:

```bash
mv .git/hooks/commit-msg.backup .git/hooks/commit-msg
```

## License

MIT
