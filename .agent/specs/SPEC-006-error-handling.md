# SPEC-006 Error Handling

## Status

Draft baseline.

## Owner

Blueprint / reliability owner.

## Linked Roadmap Phase

All phases.

## Linked ExecPlans

EP-001 through EP-010.

## User-Visible Goal

Failures are clear, typed, recoverable where possible, redacted, and observable.

## Non-Goals

- No silent failures.
- No panic-based normal control flow.
- No secret-bearing debug output.
- No infinite retry loops.

## Terms

- Error code: stable machine-readable identifier.
- Retryable: operation may be attempted again safely.
- Fail closed: refuse to continue when safety state is uncertain.

## Required Behavior

- CLI exits nonzero on errors.
- Errors include code, message, and safe next action.
- Provider errors map to internal taxonomy.
- Risk rejections are not system failures but must be explicit outcomes.
- Persistence/audit failures block execution paths.
- Retry behavior is bounded.

## Inputs

- Invalid user inputs.
- Invalid configs.
- Provider failures.
- Persistence failures.
- LLM malformed outputs.
- Security gate failures.

## Outputs

- Typed errors.
- Redacted terminal errors.
- Structured error logs.
- Nonzero exit codes for command failures.

## Error States

Recommended taxonomy:

- `CONFIG_INVALID`
- `INPUT_INVALID`
- `MODE_UNSUPPORTED`
- `SECRET_MISSING`
- `SECRET_DECRYPT_FAILED`
- `RISK_REJECTED`
- `KILL_SWITCH_ACTIVE`
- `PERSISTENCE_UNAVAILABLE`
- `SCHEMA_UNSUPPORTED`
- `ADAPTER_UNAVAILABLE`
- `PROVIDER_RATE_LIMITED`
- `PROVIDER_AUTH_FAILED`
- `LLM_OUTPUT_INVALID`
- `LIVE_TRADING_DISABLED`

## Data Rules

- Error payloads must not include secrets.
- Error codes must be stable enough for tests.
- Internal context may be logged only after redaction.

## Security Rules

- Security failures must fail closed.
- Do not include credential substrings in error messages.
- Authentication/provider failures must not reveal whether a secret value is close/correct.

## Accessibility Rules

Messages must be readable in plain text and include safe next action.

## Performance Rules

Retry loops must be bounded and use backoff when external calls are involved.

## Observability Rules

Logs must include `error_code`, `operation`, `retryable`, and redacted context.

## Required Tests

- CLI invalid input tests.
- Config error tests.
- Risk rejection message tests.
- Provider error mapping tests.
- Redaction tests.
- Bounded retry tests when retries exist.

## Acceptance Criteria

- Error taxonomy implemented.
- Tests verify representative errors.
- CLI output is redacted and actionable.
- No normal-path panics.
