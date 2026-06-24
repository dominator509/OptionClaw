# SPEC-008 Production Readiness

## Status

Draft baseline for EP-010.

## Owner

Blueprint / release owner.

## Linked Roadmap Phase

Phase 9: Production readiness.

## Linked ExecPlans

EP-010.

## User-Visible Goal

OptionClaw can be released to an operator-controlled production environment in a documented, test-backed, secure, observable, and rollback-capable state.

## Non-Goals

- No claim of trading profitability.
- No live trading unless explicit live approval exists.
- No bypass of provider/legal/compliance obligations.

## Terms

- Production-ready: software quality gate satisfied for configured mode.
- Launch gate: final checklist before production deploy.
- Rollback drill: verified ability to return to previous safe state.

## Required Behavior

- Full verification passes.
- Production-readiness check passes.
- Security review completed.
- Dependency audit reviewed.
- Deployment process documented and tested in staging/paper mode.
- Rollback process documented and drilled.
- Observability signals verified.
- Remaining risks documented.

## Inputs

- Verification command results.
- Security/audit results.
- Deployment target details.
- Config/secrets inventory.
- Runbook/checklist status.

## Outputs

- Production-readiness report.
- Launch checklist.
- Known risks.
- Rollback verification record.

## Error States

- Verification failure.
- Security gate failure.
- Missing rollback artifact.
- Missing config documentation.
- Missing secret handling.
- Live gates incomplete.

## Data Rules

- Production data must be backed up before schema changes.
- No production credentials in tests.
- Data retention documented.

## Security Rules

- No secrets in repository.
- Redaction verified.
- Live mode gated.
- Kill switch verified.

## Accessibility Rules

CLI commands remain readable and operable from terminal.

## Performance Rules

Critical operations have documented expectations and no known severe bottlenecks for target hardware.

## Observability Rules

Health, logs, and operational signals are verified before launch.

## Required Tests

- Full verification.
- Production readiness check.
- Smoke test on deployment target or staging equivalent.
- Rollback drill verification.
- Security redaction checks.

## Acceptance Criteria

- All readiness sections in `PRODUCTION_READINESS.md` are complete.
- `./scripts/production-readiness-check.sh` passes.
- Remaining risks are documented.
- Live trading remains disabled unless separately approved.
