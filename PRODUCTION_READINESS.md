# OptionClaw Production Readiness

## Definition of Production Readiness

OptionClaw is production-ready when it is functionally complete for its configured mode, test-backed, secure, observable, deployable, rollback-capable, documented, and operationally supportable. Production readiness does not mean profitability is proven and does not authorize live trading by itself. Live trading also requires explicit operator approval, provider credentials, compliance review, and all live-mode gates.

## Functional Readiness

- Core user outcomes work in paper mode.
- Required behavior from specs is implemented.
- Non-goals remain excluded.
- Risk gates reject invalid or unsafe order intents.
- LLM output cannot bypass deterministic checks.
- Paper execution lifecycle is auditable.
- Live execution remains disabled until all gates pass.

## Test Readiness

- `./scripts/lint.sh` passes.
- `./scripts/format-check.sh` passes.
- `./scripts/typecheck.sh` passes.
- `./scripts/test-unit.sh` passes.
- `./scripts/test-integration.sh` passes.
- `./scripts/test-e2e.sh` passes.
- `./scripts/build.sh` passes.
- `./scripts/security-check.sh` passes.
- `./scripts/dependency-audit.sh` passes or documented equivalent is approved.
- `./scripts/smoke-test.sh` passes.
- Regression tests cover critical failures.

## Security Readiness

- No secrets committed.
- Sensitive logs are redacted.
- Dependency audit reviewed.
- Input validation exists at trust boundaries.
- Local secret storage is encrypted.
- Kill switch works.
- Live mode fails closed when any required gate is missing.
- Provider integrations use sandbox/fixtures before live.

## Privacy Readiness

- Data retention documented.
- Local data paths documented.
- Production logs treated as sensitive.
- User data export/deletion requirements addressed if a future multi-user mode is added.

## Performance Readiness

- Expected load documented.
- Critical-path latency expectations documented.
- Obvious bottlenecks identified.
- Performance smoke or benchmark checks exist for risk evaluation and paper run-once if needed.

## Accessibility Readiness

CLI accessibility requirements:

- Commands support `--help`.
- Errors are clear and actionable.
- Required status is not color-only.
- Output works in keyboard-only terminal workflows.

## Observability Readiness

- Structured logs exist.
- Errors are logged without secrets.
- Health checks exist.
- Metrics or operational signals exist.
- Alert expectations documented.
- Runbooks link signals to actions.

## Deployment Readiness

- Build artifact documented.
- Environment variables documented.
- Deployment process documented.
- Release checklist exists.
- Post-deploy smoke test exists.
- Production deployment STOP conditions documented.

## Rollback Readiness

- Rollback triggers documented.
- Binary/config rollback documented.
- Data rollback rules documented.
- Rollback verification documented.
- Rollback drill completed before live production.

## Data Readiness

- Local schema versions documented.
- Backup/restore documented.
- Migrations are dry-run capable where applicable.
- Corrupt state handling tested.
- Audit retention documented.

## Documentation Readiness

- `PROJECT_BRIEF.md`, `ARCHITECTURE.md`, `COMMANDS.md`, `TESTING.md`, `SECURITY.md`, `ENVIRONMENT.md`, `DEPLOYMENT.md`, `OPERATIONS.md`, `OBSERVABILITY.md`, `RELEASE.md`, and `ROLLBACK.md` are current.
- Relevant specs and ExecPlans are updated.
- Decisions and assumptions are current.

## Support Readiness

- Incident response checklist exists.
- Common failure modes documented.
- Escalation rules documented.
- Maintenance procedure documented.

## Final Launch Gate

Before production launch:

1. Run `./scripts/verify.sh`.
2. Run `./scripts/production-readiness-check.sh`.
3. Review `git diff --name-only`.
4. Confirm no secrets or production data in diff.
5. Confirm rollback artifact exists.
6. Confirm live trading is disabled unless separately approved.
7. Record remaining risks.

## Checklist

- [ ] Functional readiness complete.
- [ ] Test readiness complete.
- [ ] Security readiness complete.
- [ ] Privacy readiness complete.
- [ ] Performance readiness complete.
- [ ] Accessibility readiness complete.
- [ ] Observability readiness complete.
- [ ] Deployment readiness complete.
- [ ] Rollback readiness complete.
- [ ] Data readiness complete.
- [ ] Documentation readiness complete.
- [ ] Support readiness complete.
- [ ] Final launch gate complete.
