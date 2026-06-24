# Production Readiness Checklist

- [ ] Functionality: core paper-mode outcomes work.
- [ ] Functionality: non-goals remain excluded.
- [ ] Tests: lint, format, typecheck, unit, integration, E2E, build, security, audit, smoke pass.
- [ ] Security: no secrets committed.
- [ ] Security: logs/errors redact sensitive values.
- [ ] Security: live trading gated and disabled by default.
- [ ] Privacy: data paths, retention, and logs documented.
- [ ] Performance: expected target hardware/load documented.
- [ ] Performance: critical paths have no known severe bottlenecks.
- [ ] Accessibility: CLI help, clear errors, non-color-only status.
- [ ] Observability: structured logs, health, metrics/signals, alerts expectations exist.
- [ ] Deployment: build artifact and deployment steps documented.
- [ ] Rollback: binary/config/data rollback documented and drilled.
- [ ] Backups: local data backup/restore verified if persistence exists.
- [ ] Docs: all top-level and spec docs current.
- [ ] Support: operations and incident response runbooks complete.
- [ ] Launch gate: `./scripts/production-readiness-check.sh` passes.
- [ ] Live trading: explicit approval exists before live mode; otherwise live remains disabled.
