# OptionClaw Observability

## Logging Strategy

Use structured logs for all runtime operations. Logs must be machine-readable where practical and human-readable in CLI summaries. Default log level is `info`.

## Structured Log Fields

Required fields as applicable:

- `timestamp`
- `level`
- `command`
- `mode`
- `strategy_id`
- `risk_profile_id`
- `order_intent_id`
- `provider`
- `operation`
- `latency_ms`
- `result`
- `error_code`
- `retryable`
- `correlation_id`

Never include raw secrets.

## Redaction Rules

Redact values for:

- API keys.
- Tokens.
- Passwords.
- Private keys.
- Seed phrases.
- Secret config values.
- Provider request headers.
- Sensitive account identifiers.

Redaction must happen before values enter logging or terminal output.

## Metrics

Initial metrics may be emitted as structured log counters or local text output until a metrics sink is selected. Required operational signals:

- Command success/failure count.
- Config validation failures.
- Risk accept/reject counts.
- Paper execution count.
- Provider error count by provider and operation.
- Audit append failures.
- Health status.
- Kill switch active state.
- LLM response parse failures.

## Traces

Distributed tracing is not required for initial local CLI. Local span-like structured fields are required around:

- Config load.
- Market data load.
- LLM request.
- Risk evaluation.
- Persistence write.
- Execution simulation/submission.

## Health Checks

Health command after EP-008:

```sh
optionclaw health --config <path>
```

Health output must include status for config, data directory, secrets store, kill switch, audit log, providers, and mode. It must not reveal secrets.

## Uptime Checks

For daemon mode, add uptime checks in a future ExecPlan. For CLI mode, smoke tests are the initial health substitute.

## Dashboards

No external dashboard is required initially. EP-008 should define a minimal local dashboard format or log query examples for:

- Risk rejections.
- Provider errors.
- Audit write failures.
- Execution attempts by mode.
- Kill switch activation.

## Alerts

Alert expectations for production:

- Any live execution error.
- Audit append failure.
- Kill switch active while scheduled trading is expected.
- Provider authentication failure.
- Repeated provider rate-limit errors.
- State verification failure.
- Secret redaction test failure in CI.

## Service-Level Indicators

- CLI command success rate.
- Paper run-once success rate.
- Risk evaluation latency.
- Audit append success rate.
- Provider adapter success rate.
- Health check success.

## Service-Level Objectives

Initial internal SLO targets before live mode:

- 100% of order intents pass through risk gate before execution.
- 100% of audit-required lifecycle events are persisted or execution fails closed.
- 0 known secret leaks in logs/tests/fixtures.
- Smoke test passes for each deployed artifact.

## Debugging Production Issues

1. Activate kill switch if execution safety is uncertain.
2. Capture command, config path, mode, version, and correlation ID.
3. Inspect redacted logs.
4. Run health command.
5. Verify local state.
6. Follow incident response checklist.
7. Do not expose secrets in tickets, chat, or logs.

## Observability Acceptance Criteria

- Structured logs exist for critical flows.
- Redaction tests pass.
- Health command works without secrets.
- Metrics/signals are documented.
- Alerts are documented for production.
- Observability docs map each signal to a failure mode.
