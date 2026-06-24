# SPEC-007 Observability

## Status

Draft baseline for EP-008.

## Owner

Blueprint / operations owner.

## Linked Roadmap Phase

Phase 7: Observability and operations.

## Linked ExecPlans

EP-008, EP-010.

## User-Visible Goal

Operators can understand OptionClaw health, failures, risk decisions, provider status, and execution state without exposing secrets.

## Non-Goals

- No mandatory external SaaS monitoring initially.
- No distributed tracing backend initially.
- No dashboard that requires a web server initially.

## Terms

- Structured log: key/value log event.
- Metric: count, gauge, or timing signal.
- Health check: command that reports readiness and safety state.

## Required Behavior

- Emit structured logs for config load, state verification, risk evaluation, paper execution, adapter calls, and errors.
- Redact sensitive values before logging.
- Provide health command.
- Provide operational signals for success/failure counts and latency where practical.
- Document alert expectations.

## Inputs

- Runtime events.
- Config/log level.
- Health command invocation.
- Provider/mock status.

## Outputs

- Structured logs.
- Health report.
- Metrics-like counters or log fields.
- Runbook entries.

## Error States

- Logging initialization failure.
- Redaction failure.
- Health check failure.
- Metrics sink unavailable if one is later added.

## Data Rules

- Logs must not include secrets.
- Health output must not reveal credentials.
- Use stable field names.

## Security Rules

- Redaction is mandatory before logs.
- Do not lower log level to reveal secrets in production.

## Accessibility Rules

Health output must be readable in terminal and not rely on color only.

## Performance Rules

Logging must not significantly delay critical risk/execution paths. If synchronous logging blocks, document and test the behavior.

## Observability Rules

Required fields are listed in `OBSERVABILITY.md`. Every critical flow should include correlation ID after EP-008.

## Required Tests

- Structured log field test or snapshot.
- Redaction test.
- Health success and failure tests.
- Smoke test verifies health command.

## Acceptance Criteria

- Observability docs match implementation.
- Health command works.
- Logs redact secrets.
- Smoke tests pass.
