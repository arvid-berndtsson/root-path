 #!/bin/sh
 set -euo pipefail

 COMMIT_MSG_FILE=${1:-}

 if [ -z "${COMMIT_MSG_FILE}" ]; then
   echo "conventional commit check: missing commit message file path" >&2
   exit 1
 fi

 # Prefer running the compiled binary if available, otherwise use cargo run.
 if command -v target/release/conv-commit-check >/dev/null 2>&1; then
   target/release/conv-commit-check "$COMMIT_MSG_FILE"
 elif command -v cargo >/dev/null 2>&1; then
   cargo run --quiet -- "$COMMIT_MSG_FILE"
 else
   echo "conventional commit check: cargo not found and no local binary built" >&2
   exit 1
 fi


