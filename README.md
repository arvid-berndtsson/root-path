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

`cc-check` works on **Windows**, **Linux**, and **macOS**. Install it using your existing package manager, then set up git hooks for automatic validation.

### Why Multiple Package Managers?

We publish to npm, crates.io, and PyPI so you can use **your existing package manager** without adding dependencies to your project. A Node.js project doesn't need Rust installed, and a Python project doesn't need npm.

### 📦 Install with Your Package Manager

**Node.js/npm projects:**
```bash
npm install --save-dev @arvid-berndtsson/cc-check
```

**Rust/Cargo projects:**
```bash
cargo install cc-check
```

**Python projects:**
```bash
pip install cc-check
```

### 🔧 Set Up Git Hooks (Recommended)

After installing, set up git hooks for automatic validation on every commit:

**Quick setup (direct git hook):**
```bash
cc-check install
```

This will:
- Install a cross-platform commit-msg hook in your `.git/hooks/` directory
- Backup any existing commit-msg hook
- Work on Windows, Linux, and macOS

**Or integrate with your existing workflow:**
- **npm/Node.js projects**: See [Husky integration](#option-1-husky-npmnodejs-projects---recommended) below
- **Python projects**: See [pre-commit integration](#option-2-pre-commit-framework-pythonmulti-language) below

### 🔧 Integration Options

#### Option 1: Husky (npm/Node.js projects - Recommended)

Most JavaScript/TypeScript projects use [husky](https://typicode.github.io/husky/) for git hooks:

```bash
# Install husky and cc-check
npm install --save-dev @arvid-berndtsson/cc-check husky

# Initialize husky (if not already done)
npx husky init

# Add commit-msg hook
echo "npx cc-check check \$1" > .husky/commit-msg
```

Or add to your `package.json`:
```json
{
  "scripts": {
    "prepare": "husky install"
  }
}
```

#### Option 2: pre-commit Framework (Python/Multi-language)

The [pre-commit](https://pre-commit.com/) framework works with any language:

```bash
# Install
pip install cc-check pre-commit

# Add to .pre-commit-config.yaml
cat >> .pre-commit-config.yaml << 'EOF'
repos:
  - repo: local
    hooks:
      - id: cc-check
        name: cc-check
        entry: cc-check check
        language: system
        stages: [commit-msg]
        pass_filenames: true
EOF

# Install hooks
pre-commit install --hook-type commit-msg
```

#### Option 3: Direct Git Hook (Universal)

Works with any project type:

```bash
cc-check install
```

This will:
- Install a cross-platform commit-msg hook in your `.git/hooks/` directory
- Backup any existing commit-msg hook
- Work on Windows, Linux, and macOS

**Note:** On Windows, the hook uses a `.bat` file if needed, but Git Bash (included with Git for Windows) can also run `.sh` hooks.

### Build from Source

If you want to build from source (requires Rust and Cargo):

```bash
# Clone the repository
git clone https://github.com/arvid-berndtsson/cc-check.git
cd cc-check

# Build and install
cargo build --release
./target/release/cc-check install
```

## Usage

### As a Git Hook (Recommended)

Once you've set up git hooks (via `cc-check install` or integration with husky/pre-commit), the hook will **automatically validate commit messages** when you run `git commit`. Invalid messages will be rejected.

```bash
# ❌ This will be rejected
git commit -m "invalid commit message"

# ✅ This will pass
git commit -m "feat: add new feature"
```

### As a CLI Tool (Manual Validation)

You can also use `cc-check` directly to validate commit messages manually:

```bash
# Check a commit message file
cc-check check .git/COMMIT_EDITMSG

# Or use backward-compatible syntax (file path as first argument)
cc-check .git/COMMIT_EDITMSG

# Check from stdin
echo "feat: add feature" | cc-check check

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

## Native Workflow Integration

`cc-check` integrates seamlessly into your existing development workflow:

### Node.js / TypeScript Projects

**With Husky (Recommended):**
```bash
npm install --save-dev @arvid-berndtsson/cc-check husky
npx husky init
echo "npx cc-check check \$1" > .husky/commit-msg
```

**With npm scripts:**
```json
{
  "scripts": {
    "prepare": "husky install",
    "commit-msg": "cc-check check"
  },
  "devDependencies": {
    "@arvid-berndtsson/cc-check": "^0.1.0",
    "husky": "^8.0.0"
  }
}
```

**Direct git hook (no husky needed):**
```bash
npm install --save-dev @arvid-berndtsson/cc-check
npx cc-check install
```

### Python Projects

**With pre-commit (Recommended):**
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
        pass_filenames: true
```

```bash
pip install cc-check pre-commit
pre-commit install --hook-type commit-msg
```

**Direct git hook (no pre-commit needed):**
```bash
pip install cc-check
cc-check install
```

### Rust Projects

**Install:**
```bash
cargo install cc-check
```

**Set up git hook:**
```bash
cc-check install  # Sets up git hook automatically
```

**As a dev dependency (alternative):**
```toml
# Cargo.toml
[dev-dependencies]
cc-check = "0.1.0"
```

Then set up the hook:
```bash
cargo run --bin cc-check -- install
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
