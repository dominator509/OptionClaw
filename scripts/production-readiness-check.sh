#!/usr/bin/env sh
set -eu
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
cd "$REPO_ROOT"

./scripts/verify.sh
required_docs="PRODUCTION_READINESS.md SECURITY.md TESTING.md OBSERVABILITY.md DEPLOYMENT.md OPERATIONS.md RELEASE.md ROLLBACK.md ENVIRONMENT.md deploy/README.md config/production.example.toml"
for f in $required_docs; do
  if [ ! -f "$f" ]; then
    echo "ERROR: Production readiness document missing: $f" >&2
    exit 1
  fi
done
if [ -d .git ]; then
  git diff --check
fi
echo "production readiness: ok"
