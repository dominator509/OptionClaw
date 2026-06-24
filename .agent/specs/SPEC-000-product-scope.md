# SPEC-000 Product Scope

## Status

Draft baseline for greenfield implementation.

## Owner

Blueprint / product owner.

## Linked Roadmap Phase

Phase 0 through Phase 9.

## Linked ExecPlans

EP-000, EP-001, EP-010.

## User-Visible Goal

A user can run a local Rust CLI that evolves into an autonomous options-trading assistant, starting with safe paper trading, deterministic risk gates, and auditable decisions.

## Non-Goals

- No guaranteed profit or financial advice.
- No live trading before production-readiness gates and explicit approval.
- No web/mobile UI initially.
- No multi-user SaaS initially.
- No external database initially.
- No private-key custody initially.
- No long-term investment planning.

## Terms

- Paper mode: simulated trading with no real orders.
- Sandbox mode: provider test environment with no real funds unless provider-specific rules say otherwise.
- Live mode: real orders or real funds.
- Order intent: internal representation of a proposed trade before execution.
- Risk gate: deterministic validation that can accept or reject an order intent.
- LLM brain: advisory model provider that can produce reasoning or scores but cannot bypass risk gates.

## Required Behavior

- Default mode is paper.
- CLI must expose help and validation commands.
- System must reject live behavior until live gates are implemented.
- System must make trading decisions auditable.
- User-facing output must clearly identify mode.
- Product must distinguish simulated/paper results from real performance.

## Inputs

- CLI args.
- Config files.
- Environment variables.
- Market/news/fundamental/model fixtures or provider responses.
- Local state files.

## Outputs

- Terminal summaries.
- Structured logs.
- Audit records.
- Paper-trade state.
- Health/status output.

## Error States

- Missing or invalid config.
- Missing secret for selected provider.
- Unsupported trading mode.
- Risk rejection.
- Adapter failure.
- Persistence failure.
- Live mode not authorized.

## Data Rules

- No external database initially.
- Local state is schema-versioned.
- Audit logs are append-only and secret-free.
- Data retention and backup rules must be documented before production.

## Security Rules

- No committed secrets.
- Redact sensitive values.
- Fail closed on live or risk uncertainty.
- Kill switch required before live trading.

## Accessibility Rules

CLI output must not depend on color only. Commands must support `--help` and readable errors.

## Performance Rules

Core domain/risk checks should be fast enough for per-intent execution. Performance claims must be measured before being documented as guarantees.

## Observability Rules

Critical operations must emit structured logs and operational signals after EP-008.

## Required Tests

- CLI help smoke test.
- Config validation test.
- Paper-mode default test.
- Live-mode disabled test.
- Audit output test once persistence exists.

## Acceptance Criteria

- Product scope docs are present and match non-goals.
- EP-001 creates a CLI that defaults to paper mode.
- E2E tests prove help and config validation behavior.
- Live mode fails closed until EP-006/EP-010 gates exist.
