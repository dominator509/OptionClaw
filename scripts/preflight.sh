#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

required_files="AGENTS.md COMMANDS.md .agent/PLANS.md .agent/EXECUTION_RULES.md PROJECT_BRIEF.md ASSUMPTIONS.md ARCHITECTURE.md"
for f in $required_files; do
  if [ ! -f "$f" ]; then
    echo "ERROR: Required file missing: $f" >&2
    exit 1
  fi
done

if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: Cargo is required for the Rust implementation. Install Rust/Cargo or update COMMANDS.md after repository discovery." >&2
  exit 1
fi

if [ ! -f Cargo.toml ]; then
  echo "preflight: note: Cargo.toml not found; EP-001-foundation.md must create the Rust project before build/test commands can pass." >&2
fi

if [ -d .git ]; then
  if git ls-files .env 2>/dev/null | grep -q .; then
    echo "ERROR: .env is tracked by git; remove secrets from version control before continuing." >&2
    exit 1
  fi
fi

for s in install lint format-check typecheck test-unit test-integration test-e2e build security-check dependency-audit smoke-test verify production-readiness-check; do
  if [ ! -f "scripts/$s.sh" ]; then
    echo "ERROR: Required script missing: scripts/$s.sh" >&2
    exit 1
  fi
done

echo "preflight: ok"
