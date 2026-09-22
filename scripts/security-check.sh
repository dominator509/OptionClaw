#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

if [ ! -f Cargo.toml ]; then
  echo "ERROR: Cargo.toml missing. Complete EP-001-foundation.md before running security check." >&2
  exit 1
fi
if [ -d .git ]; then
  if git ls-files .env 2>/dev/null | grep -q .; then
    echo "ERROR: .env is tracked by git." >&2
    exit 1
  fi
fi
scan_paths=""
for d in src tests fixtures config .github; do
  if [ -d "$d" ]; then
    scan_paths="$scan_paths $d"
  fi
done
if [ -n "$scan_paths" ]; then
  if grep -R -n -E -- '-----BEGIN (RSA |EC |OPENSSH |PRIVATE )?KEY-----|seed phrase|mnemonic phrase|api_secret *= *"[^f][^"]+"|api_key *= *"sk-[A-Za-z0-9]' $scan_paths >/tmp/optionclaw_secret_scan.txt 2>/dev/null; then
    cat /tmp/optionclaw_secret_scan.txt >&2
    echo "ERROR: Potential secret material found in repository-controlled files." >&2
    exit 1
  fi
fi
cargo check --all-targets --all-features >/dev/null
echo "security check: ok"
