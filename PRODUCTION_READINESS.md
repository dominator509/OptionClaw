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
- EP-011 internal live software approval can pass only with fresh ROI evidence and mocked/sandbox-validated Alpaca gates; it does not authorize real orders by itself.

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
- Local secret storage is fail-closed and rejects plaintext secret files.
- Kill switch works.
- Live mode fails closed when any required gate is missing.
- Provider integrations use sandbox/fixtures before live.
- Alpaca live credentials are env-only and live submit fails closed without explicit enablement, risk caps, kill switch, fresh approval artifact, and options level 2 capability.

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

- Build artifact documented as `target/release/optionclaw`.
- Production paper config example documented at `config/production.example.toml`.
- Manual deploy path documented in `deploy/README.md`.
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
- Release artifact and config restore paths documented.

## Data Readiness

- Local schema versions documented.
- Backup/restore documented.
- Migrations are dry-run capable where applicable.
- Corrupt state handling tested.
- Audit retention documented.

## Documentation Readiness

- `PROJECT_BRIEF.md`, `ARCHITECTURE.md`, `COMMANDS.md`, `TESTING.md`, `SECURITY.md`, `ENVIRONMENT.md`, `DEPLOYMENT.md`, `OPERATIONS.md`, `OBSERVABILITY.md`, `RELEASE.md`, `ROLLBACK.md`, and `deploy/README.md` are current.
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

## EP-011 Live Software Approval Gate

Internal live software approval requires:

- `cargo fmt`, clippy, typecheck, unit, integration, contract, E2E, security, dependency, smoke, and readiness checks passing.
- ROI evidence with annualized net ROI at least 25%, forward-paper ROI at least 8%, profit factor at least 1.35, max drawdown no greater than 20%, at least 200 backtest trades, at least 30 forward-paper trades, and zero risk-gate bypasses.
- Approval artifact age no greater than seven days with matching strategy/risk config hash.
- Alpaca account/options capability confirmed at runtime.
- Operator-supplied env-only Alpaca credentials and `OPTIONCLAW_ENABLE_LIVE_TRADING=true`.
- `--confirm-live` on submit.

This gate is internal software approval only. Broker approval, legal/regulatory suitability, and future ROI remain external risks.

## Checklist

This checklist reflects configured paper-mode readiness. Live trading remains disabled unless separately approved.

- [x] Functional readiness complete.
- [x] Test readiness complete.
- [x] Security readiness complete.
- [x] Privacy readiness complete.
- [x] Performance readiness complete.
- [x] Accessibility readiness complete.
- [x] Observability readiness complete.
- [x] Deployment readiness complete.
- [x] Rollback readiness complete.
- [x] Data readiness complete.
- [x] Documentation readiness complete.
- [x] Support readiness complete.
- [x] Final launch gate complete.
