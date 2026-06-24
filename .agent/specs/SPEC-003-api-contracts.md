# SPEC-003 API / Service / CLI Contracts

## Status

Draft baseline for EP-004.

## Owner

Blueprint / service owner.

## Linked Roadmap Phase

Phase 3: API or service layer.

## Linked ExecPlans

EP-004, EP-005, EP-007.

## User-Visible Goal

OptionClaw exposes stable internal service methods and CLI-facing contracts for config validation, state management, paper execution, risk explanation, and health checks.

## Non-Goals

- No public web API initially.
- No live broker order submission initially.
- No wallet signing initially.
- No provider-specific CLI commands until adapters are implemented.

## Terms

- Service method: Rust function boundary called by CLI.
- Adapter trait: interface for external provider behavior.
- Contract test: test that verifies request/response behavior at a boundary.

## Required Behavior

Planned contracts:

- `check-config`: validates config and prints mode/status.
- `state init`: creates local state directory.
- `state verify`: checks schema and file integrity.
- `paper run-once`: runs one fixture-backed paper decision/execution cycle.
- `risk explain`: evaluates a serialized order intent and prints decision reasons.
- `health`: reports local readiness.

All commands must validate inputs and return nonzero on failure.

## Inputs

- CLI flags.
- Config path.
- Data directory.
- Fixture path.
- Serialized order intent path.
- Mock provider responses.

## Outputs

- Terminal text.
- Optional JSON output if implemented and documented.
- Audit records for execution flows.
- Exit codes.

## Error States

- Missing required flag.
- Invalid path.
- Config invalid.
- Unsupported mode.
- Adapter unavailable.
- Risk rejected.
- Persistence unavailable.

## Data Rules

- Service methods use domain types at boundaries after parsing.
- Provider payloads remain in adapter modules.
- CLI output must not include raw secrets.

## Security Rules

- Live commands are unavailable until live gates are implemented.
- Provider credentials are loaded only through secrets boundary.
- Request validation must happen before side effects.

## Accessibility Rules

- Every command supports help.
- Errors must include what failed and the next safe action.
- Status must not rely on color only.

## Performance Rules

- CLI help and config validation should start quickly without network calls.
- Paper run-once with fixtures should not require network.

## Observability Rules

- Service calls must emit structured logs after observability is implemented.
- Errors must include stable codes.

## Required Tests

- Contract tests for each service method.
- E2E tests for CLI commands.
- Invalid input tests.
- Adapter mock success/failure tests.
- No-live-command test.

## Acceptance Criteria

- Service contracts are documented and implemented.
- CLI commands call services rather than embedding business logic in `main.rs`.
- Contract and integration tests pass.
