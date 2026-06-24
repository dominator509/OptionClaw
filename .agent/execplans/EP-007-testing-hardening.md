# EP-007 Testing Hardening

## 1. Purpose / Big Picture

Harden OptionClaw reliability by expanding unit, integration, E2E, regression, failure-mode, contract, and CI validation coverage. This plan turns the earlier implementation into a stable, test-backed system.

## 2. Scope

- Review and fill gaps in unit coverage.
- Add integration coverage for persistence, services, adapters, and security gates.
- Add E2E coverage for CLI flows and failure cases.
- Add regression tests for known bugs/discoveries.
- Stabilize fixtures and cleanup.
- Ensure CI runs the full local verification sequence.

## 3. Non-goals

- No new product features.
- No live provider tests.
- No performance rewrites unless needed for deterministic tests.
- No deleting tests to pass CI.

## 4. Context and Orientation

Prior ExecPlans should have introduced baseline tests. This plan hardens coverage and removes flaky or missing validation. Use `TESTING.md` as the source of truth.

## 5. Files to Read First

- `TESTING.md`
- `COMMANDS.md`
- `.github/workflows/ci.yml`
- All `src/` modules
- All `tests/`
- All `fixtures/`
- Active failure notes from prior ExecPlans

## 6. Files to Change

Expected changed files:

- `tests/integration_*.rs`
- `tests/e2e_cli.rs`
- `tests/contract_*.rs`
- Unit test modules under `src/`
- `fixtures/**`
- `.github/workflows/ci.yml`
- `TESTING.md`
- `.agent/execplans/EP-007-testing-hardening.md`

Forbidden changes:

- Feature behavior changes not required by failing tests.
- Live credentials or network-dependent tests.
- Removing critical tests without replacement.

## 7. Interfaces and Contracts

Testing contracts:

- Default test suite must run offline.
- Fixtures must be deterministic and secret-free.
- CI must call scripts, not duplicate divergent commands.
- Failure-mode tests must assert stable error codes.

## 8. Milestones

### M1: Coverage inventory and gap list

- Goal: Identify missing tests by spec and module.
- Files to read: specs, tests, source modules.
- Files to change: EP-007 `Surprises & Discoveries`, `TESTING.md` if matrix needs update.
- Exact edits expected: Add gap list mapped to specs and tests.
- Validation command: `./scripts/test-unit.sh`
- Expected result: `unit tests: ok` before adding more tests.
- Recovery instruction: If unit tests already fail, debug with anti-fixation before adding coverage.

### M2: Unit and failure-mode tests

- Goal: Add missing unit and negative tests.
- Files to read: domain/risk/errors/security modules.
- Files to change: unit test modules.
- Exact edits expected: Tests for boundary values, malformed inputs, risk rejection, LLM invalid output, redaction, live disabled.
- Validation command: `./scripts/test-unit.sh`
- Expected result: `unit tests: ok`.
- Recovery instruction: If tests reveal real bug, fix smallest production code path and record decision.

### M3: Integration and contract tests

- Goal: Add service, persistence, and adapter contract tests.
- Files to read: services/adapters/persistence.
- Files to change: `tests/integration_*.rs`, `tests/contract_*.rs`, fixtures.
- Exact edits expected: Tests for adapter success/failure, persistence corruption, audit failure, state commands.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If fixture duplication grows, factor test helpers within tests; do not add external services.

### M4: E2E, smoke, and CI hardening

- Goal: Ensure user-facing flows and CI match local verification.
- Files to read: CLI tests, CI workflow, scripts.
- Files to change: `tests/e2e_cli.rs`, `.github/workflows/ci.yml`, possibly scripts only with command evidence.
- Exact edits expected: E2E success and failure flows; CI runs `./scripts/verify.sh`.
- Validation command: `./scripts/test-e2e.sh && ./scripts/smoke-test.sh`
- Expected result: `e2e tests: ok`, `smoke test: ok`.
- Recovery instruction: If CI uses unavailable tools, keep CI minimal but aligned with documented scripts and record production blocker.

### M5: Full verification stability

- Goal: Prove the full sequence passes locally.
- Files to read: changed tests and scripts.
- Files to change: EP-007 outcomes.
- Exact edits expected: Update plan with final command results and remaining coverage risks.
- Validation command: `./scripts/verify.sh`
- Expected result: `verify: ok` or documented audit-tool blocker before production.
- Recovery instruction: If verify fails due one command, run narrow command and apply anti-fixation.

## 9. Concrete Steps

1. Run preflight.
2. Inventory tests by spec.
3. Add missing unit tests.
4. Add missing integration/contract tests.
5. Add missing E2E/smoke tests.
6. Align CI.
7. Run verify and update plan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/verify.sh
git diff --name-only
```

Acceptance criteria:

- Required tests per `TESTING.md` exist.
- Offline full verification passes or only documented external audit tool blocker remains.
- CI invokes documented scripts.
- No tests require real secrets or live APIs.
- Flaky tests are fixed or documented with deterministic replacement.

## 11. Idempotence and Recovery

Rerunning should not duplicate fixtures. If tests fail repeatedly, isolate by module and use a simpler deterministic fixture. Do not weaken assertions without preserving acceptance behavior.

## 12. Progress

- [ ] M1 - Coverage inventory and gap list.
- [ ] M2 - Unit and failure-mode tests.
- [ ] M3 - Integration and contract tests.
- [ ] M4 - E2E, smoke, and CI hardening.
- [ ] M5 - Full verification stability.

## 13. Surprises & Discoveries

Record coverage gaps, flaky tests, and validation failures here.

## 14. Decision Log

Record test strategy decisions and any command/CI changes here.

## 15. Outcomes & Retrospective

Complete after M5.
