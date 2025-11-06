#!/bin/bash
#
# Installation script for cc-check git hook
#

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GIT_DIR="$(git rev-parse --git-dir 2>/dev/null || echo "")"

if [ -z "$GIT_DIR" ]; then
    echo "Error: Not in a git repository"
    exit 1
fi

HOOKS_DIR="$GIT_DIR/hooks"
COMMIT_MSG_HOOK="$HOOKS_DIR/commit-msg"
HOOK_SCRIPT="$SCRIPT_DIR/git-hooks/commit-msg"

# Build the Rust project
echo "Building cc-check..."
cargo build --release

# Create hooks directory if it doesn't exist
mkdir -p "$HOOKS_DIR"

# Backup existing commit-msg hook if it exists
if [ -f "$COMMIT_MSG_HOOK" ]; then
    echo "Backing up existing commit-msg hook to commit-msg.backup"
    cp "$COMMIT_MSG_HOOK" "$COMMIT_MSG_HOOK.backup"
fi

# Install the hook
echo "Installing commit-msg hook..."
cp "$HOOK_SCRIPT" "$COMMIT_MSG_HOOK"
chmod +x "$COMMIT_MSG_HOOK"

echo "✓ Commit-msg hook installed successfully!"
echo ""
echo "The hook will now validate all commit messages against the conventional commit format."
echo ""
echo "To test it, try committing with:"
echo "  git commit -m \"invalid commit\"        # Will fail"
echo "  git commit -m \"test: valid commit\"    # Will pass"

