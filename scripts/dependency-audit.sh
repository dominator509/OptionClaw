#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

if [ ! -f Cargo.toml ]; then
  echo "ERROR: Cargo.toml missing. Complete EP-001-foundation.md before running dependency audit." >&2
  exit 1
fi
if ! cargo audit --version >/dev/null 2>&1; then
  echo "ERROR: cargo-audit is required for dependency audit. Install with 'cargo install cargo-audit' or update COMMANDS.md with an approved equivalent after repository discovery." >&2
  exit 1
fi
cargo audit
echo "dependency audit: ok"
