#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

if [ ! -f Cargo.toml ]; then
  echo "ERROR: Cargo.toml missing. Complete EP-001-foundation.md before running build." >&2
  exit 1
fi
if [ -f Cargo.lock ]; then
  cargo build --release --locked
else
  cargo build --release
fi
echo "build: ok"
