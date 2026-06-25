# Production Readiness Checklist

This checklist reflects configured paper-mode readiness. Live trading remains disabled unless separately approved.

- [x] Functionality: core paper-mode outcomes work.
- [x] Functionality: non-goals remain excluded.
- [x] Tests: lint, format, typecheck, unit, integration, E2E, build, security, audit, smoke pass.
- [x] Security: no secrets committed.
- [x] Security: logs/errors redact sensitive values.
- [x] Security: live trading gated and disabled by default.
- [x] Privacy: data paths, retention, and logs documented.
- [x] Performance: expected target hardware/load documented.
- [x] Performance: critical paths have no known severe bottlenecks.
- [x] Accessibility: CLI help, clear errors, non-color-only status.
- [x] Observability: structured logs, health, metrics/signals, alerts expectations exist.
- [x] Deployment: build artifact and deployment steps documented.
- [x] Rollback: binary/config/data rollback documented and drilled.
- [x] Backups: local data backup/restore verified if persistence exists.
- [x] Docs: all top-level and spec docs current.
- [x] Support: operations and incident response runbooks complete.
- [x] Launch gate: `./scripts/production-readiness-check.sh` passes.
- [x] Live trading: explicit approval exists before live mode; otherwise live remains disabled.
