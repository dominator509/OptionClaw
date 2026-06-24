# SPEC-001 Core Domain

## Status

Draft baseline for EP-002.

## Owner

Blueprint / domain owner.

## Linked Roadmap Phase

Phase 1: Core domain.

## Linked ExecPlans

EP-002, EP-007.

## User-Visible Goal

OptionClaw can represent market context, option contracts, strategy signals, order intents, account state, positions, and risk decisions deterministically before any infrastructure integration.

## Non-Goals

- No broker SDK calls.
- No file persistence.
- No LLM network calls.
- No live order submission.
- No provider-specific types in domain modules.

## Terms

- Option contract: standardized option instrument fields.
- Signal: normalized input from technical/news/fundamental/math/model sources.
- Strategy candidate: pre-risk proposed trade idea.
- Order intent: validated proposal eligible for risk checking.
- Risk decision: accepted/rejected/skipped outcome with reasons.

## Required Behavior

- Domain constructors reject invalid instruments, prices, quantities, expirations, and modes.
- Order intent includes trading mode and risk context.
- Risk gates reject missing limits, excessive account risk, excessive daily loss, disabled execution, unknown instrument, and kill-switch-active state.
- LLM advisory score is optional and cannot be required for pure risk checks.
- Domain logic is deterministic given inputs.

## Inputs

- Normalized market snapshot.
- Account snapshot.
- Position snapshot.
- Strategy parameters.
- Optional advisory score/reasoning.
- Risk limits.

## Outputs

- Strategy candidate.
- Order intent.
- Risk decision with reason codes.
- Domain error types.

## Error States

- Invalid price.
- Invalid quantity.
- Unsupported side/type.
- Expired contract.
- Missing risk limits.
- Risk threshold exceeded.
- Unsupported trading mode.

## Data Rules

- Use precise decimal representation for prices/quantities where needed.
- Do not use floats for money unless explicitly isolated and justified.
- IDs must be deterministic in tests.
- No external side effects.

## Security Rules

- Domain structs must not contain raw secrets.
- Domain debug output must not include sensitive values.
- Live mode must be a typed explicit value, not a string sprinkled through code.

## Accessibility Rules

Domain reason codes must be mappable to human-readable CLI messages.

## Performance Rules

Risk evaluation should run without network or blocking file IO and complete within a small deterministic CPU-bound path for normal inputs.

## Observability Rules

Domain outputs must include stable reason codes suitable for logs and metrics.

## Required Tests

- Valid and invalid option contract construction.
- Valid and invalid order intent construction.
- Risk accept/reject boundary tests.
- Missing risk config rejects.
- LLM score cannot bypass rejection.
- Live mode cannot proceed without gates.

## Acceptance Criteria

- Domain and risk modules compile without infrastructure imports.
- Unit tests cover required behavior and error states.
- `./scripts/test-unit.sh` passes.
- `./scripts/typecheck.sh` passes.
