# EP-010 Production Readiness

## 1. Purpose / Big Picture

Bring OptionClaw to production readiness for its configured mode by completing final verification, security review, performance review, accessibility review, privacy review, backup/restore verification, monitoring verification, deployment dry run, rollback drill, documentation review, and launch checklist.

## 2. Scope

- Full verification.
- Security and dependency review.
- Performance and resource review for local hardware/VPS.
- CLI accessibility review.
- Privacy/data review.
- Backup/restore verification.
- Observability/health verification.
- Deployment dry run in paper/sandbox mode.
- Rollback drill.
- Documentation and release checklist.

## 3. Non-goals

- No guarantee of profit.
- No legal/tax/regulatory approval by coding agent.
- No live trading unless explicit operator approval is provided after all gates.
- No provider-specific live adapter unless already implemented by separate plan.
- No production deployment without permission.

## 4. Context and Orientation

Production readiness means the software can be safely operated in its configured mode. For OptionClaw, live trading is a separate permission gate. If any live credential, legal/compliance issue, or real fund movement is involved, STOP.

## 5. Files to Read First

- `PRODUCTION_READINESS.md`
- `.agent/specs/SPEC-008-production-readiness.md`
- `SECURITY.md`
- `TESTING.md`
- `OBSERVABILITY.md`
- `DEPLOYMENT.md`
- `OPERATIONS.md`
- `RELEASE.md`
- `ROLLBACK.md`
- `ASSUMPTIONS.md`
- `DECISIONS.md`
- All prior ExecPlan Outcomes

## 6. Files to Change

Expected changed files:

- `PRODUCTION_READINESS.md`
- `SECURITY.md`
- `TESTING.md`
- `OBSERVABILITY.md`
- `DEPLOYMENT.md`
- `OPERATIONS.md`
- `RELEASE.md`
- `ROLLBACK.md`
- `ASSUMPTIONS.md`
- `DECISIONS.md`
- `scripts/production-readiness-check.sh`
- `.agent/checklists/production-readiness.md`
- `.agent/execplans/EP-010-production-readiness.md`

Forbidden changes:

- New product features.
- Live trading enablement without explicit approval.
- Real credentials or production data.
- Irreversible migrations.

## 7. Interfaces and Contracts

Production readiness check must run:

```sh
./scripts/verify.sh
./scripts/security-check.sh
./scripts/dependency-audit.sh
./scripts/smoke-test.sh
```

It must also verify required docs exist. Additional target-specific deployment dry-run commands must be documented, not guessed.

## 8. Milestones

### M1: Full verification baseline

- Goal: Establish current validation status.
- Files to read: scripts, command docs.
- Files to change: EP-010 progress/surprises.
- Exact edits expected: Record exact command results and blockers.
- Validation command: `./scripts/verify.sh`
- Expected result: `verify: ok`.
- Recovery instruction: If verify fails, debug narrow failing command with anti-fixation; do not continue to launch gate until resolved or documented as STOP.

### M2: Security, privacy, and dependency review

- Goal: Confirm secret, redaction, live-mode, privacy, and dependency safety.
- Files to read: security docs, env docs, dependency manifests, tests.
- Files to change: `SECURITY.md`, `ASSUMPTIONS.md`, `DECISIONS.md`, EP-010.
- Exact edits expected: Update any remaining risks; verify no secrets; review audit result.
- Validation command: `./scripts/security-check.sh && ./scripts/dependency-audit.sh`
- Expected result: `security check: ok`, `dependency audit: ok`.
- Recovery instruction: If audit tooling missing, STOP production readiness or replace with documented approved equivalent.

### M3: Performance, accessibility, and observability review

- Goal: Confirm operational quality for CLI/local runtime.
- Files to read: observability, operations, CLI tests, performance notes.
- Files to change: docs and EP-010.
- Exact edits expected: Record performance expectations, CLI accessibility status, observability health/signal verification.
- Validation command: `./scripts/smoke-test.sh`
- Expected result: `smoke test: ok`.
- Recovery instruction: If performance is unknown, document expected load and add a simple timing smoke test or mark as production risk.

### M4: Backup/restore, deployment dry run, and rollback drill

- Goal: Verify operational recovery.
- Files to read: persistence tests, deployment docs, rollback docs.
- Files to change: `DEPLOYMENT.md`, `ROLLBACK.md`, `OPERATIONS.md`, EP-010.
- Exact edits expected: Record dry-run/rollback drill results in paper mode; update docs if commands differed.
- Validation command: `./scripts/build.sh && ./scripts/smoke-test.sh`
- Expected result: `build: ok`, `smoke test: ok`.
- Recovery instruction: If a real deployment target is unavailable, document local dry-run and mark target-specific deployment as remaining risk; STOP before production deploy.

### M5: Final launch gate

- Goal: Complete production-readiness checklist and final report.
- Files to read: all readiness docs/checklists.
- Files to change: `PRODUCTION_READINESS.md`, `.agent/checklists/production-readiness.md`, EP-010 outcomes.
- Exact edits expected: Mark checklist status, risks, and mode-specific readiness.
- Validation command: `./scripts/production-readiness-check.sh && git diff --name-only`
- Expected result: `production readiness: ok`; changed files match expected or extras justified.
- Recovery instruction: If live-specific criteria cannot pass due missing credentials/provider approval, mark production-ready for paper mode only and STOP before live.

## 9. Concrete Steps

1. Run preflight.
2. Run full verification.
3. Review security/privacy/dependencies.
4. Review performance/accessibility/observability.
5. Verify backup/restore and rollback drill.
6. Complete production-readiness checklist.
7. Run final production readiness check.
8. Update outcomes and final report.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/verify.sh
./scripts/production-readiness-check.sh
git diff --name-only
```

Acceptance criteria:

- Full verification passes.
- Production readiness check passes for configured mode.
- Security, privacy, performance, accessibility, observability, deployment, rollback, data, docs, and support status are documented.
- Live trading remains disabled unless explicit approval and gates exist.
- Remaining risks are documented.

## 11. Idempotence and Recovery

Readiness review may be rerun. Do not mark criteria complete without evidence. If a criterion fails, fix via a scoped change or record STOP. Do not deploy or enable live mode from this plan without explicit permission.

## 12. Progress

- [ ] M1 - Full verification baseline.
- [ ] M2 - Security, privacy, and dependency review.
- [ ] M3 - Performance, accessibility, and observability review.
- [ ] M4 - Backup/restore, deployment dry run, and rollback drill.
- [ ] M5 - Final launch gate.

## 13. Surprises & Discoveries

Record readiness findings and validation failures here.

## 14. Decision Log

Record readiness decisions, accepted risks, and mode limitations here.

## 15. Outcomes & Retrospective

Complete after M5 with production readiness result and remaining risks.
