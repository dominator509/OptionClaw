#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

if [ ! -f Cargo.toml ]; then
  echo "ERROR: Cargo.toml missing. Complete EP-001-foundation.md before running smoke test." >&2
  exit 1
fi
cargo run -- --help >/tmp/optionclaw_smoke_help.txt
if [ -f config/example.toml ]; then
  cargo run -- check-config --config config/example.toml >/tmp/optionclaw_smoke_config.txt
else
  echo "ERROR: config/example.toml missing. Complete EP-001-foundation.md before running smoke test." >&2
  exit 1
fi
if [ -f config/example.toml ]; then
  cargo run -- health --config config/example.toml >/tmp/optionclaw_smoke_health.txt
  grep -q "health ok" /tmp/optionclaw_smoke_health.txt
  grep -q "secrets_store_ready=" /tmp/optionclaw_smoke_health.txt
  grep -q "providers_ready=" /tmp/optionclaw_smoke_health.txt
else
  echo "ERROR: config/example.toml missing. Complete EP-001-foundation.md before running smoke test." >&2
  exit 1
fi
echo "smoke test: ok"
