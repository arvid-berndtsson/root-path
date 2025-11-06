 # Conventional Commit Checker (Rust)

 Validate commit messages against the Conventional Commits specification.

 ## Usage

 Build and test locally:

 ```bash
 cargo run -- .git/COMMIT_EDITMSG
 ```

 The tool validates the first meaningful line (ignores `#` comments) with the format:

 ```
 <type>(<scope>)!: <subject>
 ```

 - Allowed types: feat, fix, chore, docs, style, refactor, perf, test, build, ci, revert
- Subject default max length: 72 chars
- Trailing period in subject is disallowed by default
- Merge/Revert messages are allowed by default (`--no-allow-merge-commits` to disable)

 Flags:

 - `--extra-types "wip,release"` add custom allowed types
- `--max-subject 0` disable subject length check
- `--no-trailing-period=false` allow trailing period
- `--format json` machine-readable output (`{"ok":true}` or `{ "ok": false, "error": "..." }`)

 ## Pre-commit integration

 This repository includes a `.pre-commit-config.yaml` configured for the `commit-msg` stage using a local hook:

 ```yaml
 repos:
   - repo: local
     hooks:
       - id: conventional-commit-check
         name: Conventional Commit Checker (Rust)
         entry: scripts/conventional_commit_check.sh
         language: system
         stages: [commit-msg]
         pass_filenames: true
         always_run: true
 ```

 Install and enable pre-commit in your repo:

 ```bash
 pre-commit install --hook-type commit-msg
 ```

 For faster runs, build the release binary once so the script uses it:

 ```bash
 cargo build --release
 ```

## Publishing to crates.io

This repository is configured to publish to crates.io using GitHub Actions:

- Pull requests targeting `main` will run `cargo publish --dry-run` to validate the package.
- Pushing a tag named `vX.Y.Z` to the repository will trigger a publish to crates.io.

Setup steps:

1. Create a crates.io API token in your account (`Settings` → `API Tokens`).
2. Add the token to this repository secrets as `CARGO_REGISTRY_TOKEN` (`Settings` → `Secrets and variables` → `Actions`).
3. Bump the version in `Cargo.toml` and create a tag:

   ```bash
   git commit -am "chore(release): vX.Y.Z"
   git tag vX.Y.Z
   git push origin main --tags
   ```

The workflow will build, test, dry-run, and then publish the crate.

 ## License

 MIT

# Conventional Commit Checker

A Rust-based tool to validate git commit messages against the [Conventional Commits](https://www.conventionalcommits.org/) specification. This tool hooks into git's commit-msg hook to automatically validate commit messages.

## Features

- ✅ Validates commit message format according to Conventional Commits specification
- ✅ Supports all standard commit types (feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert)
- ✅ Validates optional scope
- ✅ Detects breaking changes (via `!` or `BREAKING CHANGE:` footer)
- ✅ Checks description length (warns if > 72 characters)
- ✅ Integrates seamlessly with git pre-commit hooks
- ✅ Can read from stdin or file (works with git hooks)

## Installation

### Prerequisites

- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))

### Install the Git Hook

1. Build and install the hook:
   ```bash
   ./install.sh
   ```

   This will:
   - Build the Rust binary in release mode
   - Install the commit-msg hook in your `.git/hooks/` directory
   - Backup any existing commit-msg hook

2. Alternatively, you can manually:
   ```bash
   # Build the project
   cargo build --release
   
   # Copy the hook (adjust path as needed)
   cp git-hooks/commit-msg .git/hooks/commit-msg
   chmod +x .git/hooks/commit-msg
   ```

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
# Read from stdin
echo "feat: add feature" | cargo run --release

# Or from a file
cargo run --release -- --file COMMIT_EDITMSG

# Verbose output
cargo run --release -- --verbose --file COMMIT_EDITMSG
```

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

## Building

```bash
# Debug build
cargo build

# Release build (recommended for hooks)
cargo build --release

# Run tests
cargo test
```

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

MIT OR Apache-2.0

