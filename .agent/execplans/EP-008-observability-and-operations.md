# EP-008 Observability and Operations

## 1. Purpose / Big Picture

Add structured logs, redaction, metrics-like operational signals, health checks, alert expectations, dashboards/log-query guidance, and runbooks so OptionClaw can be operated safely.

## 2. Scope

- Structured logging initialization.
- Redacted log fields.
- Health command implementation/verification.
- Metrics-like counters or structured signals.
- Operational runbook updates.
- Observability tests and smoke coverage.

## 3. Non-goals

- No mandatory external monitoring SaaS.
- No distributed tracing backend.
- No web dashboard.
- No live trading enablement.

## 4. Context and Orientation

OptionClaw is local CLI-first, so observability starts with structured logs and health output. Logs must never expose secrets. Operational docs must tell operators what to check and how to respond.

## 5. Files to Read First

- `OBSERVABILITY.md`
- `OPERATIONS.md`
- `.agent/specs/SPEC-007-observability.md`
- `src/observability/mod.rs`
- `src/cli/*`
- `src/services/health_service.rs`
- `src/secrets/*`
- Existing tests

## 6. Files to Change

Expected changed files:

- `src/observability/mod.rs`
- `src/observability/logging.rs`
- `src/observability/metrics.rs`
- `src/services/health_service.rs`
- `src/cli/*`
- `tests/integration_observability.rs`
- `tests/e2e_cli.rs`
- `OBSERVABILITY.md`
- `OPERATIONS.md`
- `.agent/execplans/EP-008-observability-and-operations.md`

Forbidden changes:

- External SaaS dependency unless explicitly justified.
- Secret-bearing logs.
- Live execution.

## 7. Interfaces and Contracts

Observability contracts:

- `init_logging(log_level)` initializes structured logs once.
- `record_metric(event)` emits local structured metric-like signal or in-memory testable counter.
- `health(config)` returns statuses for config, data dir, secrets store, kill switch, providers, audit log, mode.
- Logs include fields documented in `OBSERVABILITY.md`.

## 8. Milestones

### M1: Structured logging and redaction

- Goal: Add logging initialization and redaction tests.
- Files to read: observability and secrets modules.
- Files to change: `src/observability/logging.rs`, `src/observability/mod.rs`, tests.
- Exact edits expected: Initialize log level; ensure redaction wrapper is used; tests or snapshots prove secret redaction.
- Validation command: `./scripts/test-unit.sh`
- Expected result: `unit tests: ok`.
- Recovery instruction: If capturing logs is complex, test redaction formatter directly and record limitation.

### M2: Metrics-like operational signals

- Goal: Add local counters/log events for critical flows.
- Files to read: services and risk/execution modules.
- Files to change: `src/observability/metrics.rs`, service modules, tests.
- Exact edits expected: Emit/test counters for config success/failure, risk accept/reject, paper execution, adapter failure, audit failure.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If metrics sink is unknown, use structured log/event abstraction with in-memory test sink.

### M3: Health command and smoke coverage

- Goal: Make `optionclaw health` operationally useful.
- Files to read: health service and CLI.
- Files to change: `src/services/health_service.rs`, CLI modules, `tests/e2e_cli.rs`.
- Exact edits expected: Health report includes required statuses and redacts secrets; E2E test covers success/failure.
- Validation command: `./scripts/smoke-test.sh && ./scripts/test-e2e.sh`
- Expected result: `smoke test: ok`, `e2e tests: ok`.
- Recovery instruction: If health requires state dir not initialized, report degraded state with clear next action.

### M4: Runbooks, alerts, and dashboards/log queries

- Goal: Update operational docs to match implementation.
- Files to read: `OBSERVABILITY.md`, `OPERATIONS.md`.
- Files to change: `OBSERVABILITY.md`, `OPERATIONS.md`.
- Exact edits expected: Document fields, health statuses, alert triggers, local log query examples, failure mode mapping.
- Validation command: `git diff --name-only`
- Expected result: Changed docs are expected.
- Recovery instruction: If a documented signal is not implemented, either implement it or mark as future with production blocker.

### M5: Observability final validation

- Goal: Verify logs/health/ops are production-oriented.
- Files to read: changed files.
- Files to change: EP-008 progress/outcomes.
- Exact edits expected: Update plan and decisions.
- Validation command: `./scripts/lint.sh && ./scripts/format-check.sh && ./scripts/typecheck.sh && ./scripts/test-integration.sh && ./scripts/smoke-test.sh`
- Expected result: all commands print ok.
- Recovery instruction: If observability tests are flaky due timestamps, inject deterministic clock in tests.

## 9. Concrete Steps

1. Run preflight.
2. Add structured logging.
3. Add metrics/event abstraction.
4. Implement health checks.
5. Add tests.
6. Update runbooks and observability docs.
7. Validate and update plan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/lint.sh
./scripts/format-check.sh
./scripts/typecheck.sh
./scripts/test-integration.sh
./scripts/test-e2e.sh
./scripts/smoke-test.sh
git diff --name-only
```

Acceptance criteria:

- Structured logs exist and redact secrets.
- Health command works.
- Operational signals exist for critical flows.
- Runbooks map failure modes to actions.
- Smoke tests pass.

## 11. Idempotence and Recovery

Logging initialization must be safe if called once per process. Tests must not depend on global logger order without isolation. If external metrics are later needed, add a separate ExecPlan.

## 12. Progress

- [ ] M1 - Structured logging and redaction.
- [ ] M2 - Metrics-like operational signals.
- [ ] M3 - Health command and smoke coverage.
- [ ] M4 - Runbooks, alerts, and dashboards/log queries.
- [ ] M5 - Observability final validation.

## 13. Surprises & Discoveries

Record observability gaps and validation failures here.

## 14. Decision Log

Record logging/metrics/health design decisions here.

## 15. Outcomes & Retrospective

Complete after M5.
