#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

if [ ! -f Cargo.toml ]; then
  echo "ERROR: Cargo.toml missing. Complete EP-001-foundation.md before running E2E tests." >&2
  exit 1
fi
found=0
for f in tests/e2e*.rs tests/acceptance*.rs; do
  if [ -f "$f" ]; then
    found=1
    name=$(basename "$f" .rs)
    cargo test --test "$name" --all-features
  fi
done
if [ "$found" -eq 0 ]; then
  echo "ERROR: No E2E or acceptance tests found. Complete EP-001/EP-005 test harness before running E2E tests." >&2
  exit 1
fi
echo "e2e tests: ok"
