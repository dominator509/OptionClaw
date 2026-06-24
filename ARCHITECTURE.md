# OptionClaw Architecture

## Purpose

This document defines concrete repository boundaries and invariants for OptionClaw. Coding agents must use it to prevent feature drift, infrastructure leakage into domain logic, and unsafe live-trading behavior.

## Current Checkout Snapshot

EP-000 repository discovery found that the present checkout contains documentation and scripts only. No `Cargo.toml`, `src/`, `tests/`, `config/`, `.github/workflows/`, or other Rust source tree exists yet in this environment. EP-001 is responsible for creating the initial Cargo foundation that matches the intended map below.

## System Overview

OptionClaw is a local-first Rust CLI. The intended runtime pipeline is:

1. Load validated configuration.
2. Load encrypted secrets through a secrets boundary.
3. Ingest market/news/fundamental/model signals through adapters.
4. Build a normalized market context.
5. Ask deterministic strategy and risk modules to produce, accept, reject, or skip an order intent.
6. Optionally ask an LLM provider for reasoning or scoring.
7. Apply deterministic pre-trade risk gates after LLM output.
8. Record the decision and audit context.
9. In paper mode, simulate an order lifecycle.
10. In live mode only after production approval, submit through a broker/exchange adapter.
11. Emit structured logs, metrics, and health state.

## Intended Repository Map

```text
/
  Cargo.toml
  Cargo.lock
  src/
    main.rs                    # CLI entrypoint only
    lib.rs                     # Module exports
    cli/                       # CLI argument parsing and terminal output
    config/                    # Config loading, validation, typed settings
    domain/                    # Pure domain entities and business rules
    risk/                      # Deterministic risk checks and limits
    strategy/                  # Strategy evaluation and signal composition
    llm/                       # LLM provider traits and adapters
    market_data/               # Market data provider traits and adapters
    execution/                 # Order intent, paper executor, live executor traits
    persistence/               # Local file state, audit log, schema versions
    secrets/                   # Encrypted secrets and redaction
    observability/             # Logging, metrics, health
    errors/                    # Error taxonomy and conversions
  tests/
    integration_*.rs
    e2e_*.rs
  fixtures/
    market/
    config/
    broker/
  config/
    example.toml
  scripts/
  .github/workflows/ci.yml
```

EP-001 creates the initial version of this map. Later plans may add files only within the boundaries above unless the plan explicitly updates this architecture.

## Layer Responsibilities

| Layer | Responsibility | May Import | Must Not Import |
|---|---|---|---|
| `domain` | Pure entities: option contract, quote, signal, order intent, position, account snapshot, trading mode. | Standard library, numeric/date primitives, local domain modules. | CLI, config, adapters, HTTP clients, files, environment, secrets, broker APIs, LLM APIs. |
| `risk` | Deterministic pre-trade and portfolio risk gates. | `domain`, pure math helpers. | Broker clients, LLM adapters, file IO, CLI. |
| `strategy` | Convert normalized signals into candidate order intents. | `domain`, `risk` for read-only risk context. | Live execution, wallet operations, secrets. |
| `llm` | Provider-neutral LLM request/response traits and adapters. | `domain` DTOs, config, HTTP client crate. | Risk bypasses, direct execution, wallet code. |
| `market_data` | Provider-neutral market/news/fundamental data ingestion. | `domain`, config, HTTP client crate. | Order execution, secrets except provider tokens through secrets boundary. |
| `execution` | Paper and live order execution boundaries. | `domain`, `risk`, config, provider adapter traits, persistence. | Strategy internals, LLM provider internals. |
| `persistence` | Local encrypted/append-only files and schema versions. | `domain`, `config`, `secrets` for encryption services. | CLI formatting, broker SDK direct calls. |
| `secrets` | Secret loading, encryption, redaction, zeroization wrappers. | Config, filesystem primitives, crypto crates. | Strategy decisions, market data parsing. |
| `cli` | Command parsing, user-visible messages, exit codes. | Public application/service APIs, config. | Provider SDK internals, domain-private helpers. |
| `observability` | Structured logs, metrics, health checks. | Error taxonomy, config. | Secret values, private keys, raw credentials. |

## Dependency Rules

- `domain` must remain pure and deterministic.
- `risk` may depend on `domain`; `domain` must not depend on `risk`.
- `strategy` may produce `OrderIntent`; it must not submit orders.
- `llm` may produce advisory outputs; it must not approve live orders.
- `execution` must call `risk` immediately before simulated or live submission.
- `persistence` must not call broker, exchange, wallet, or LLM APIs.
- `cli` must call service-layer functions; business rules must not live in `main.rs`.

## Import Rules

- No module may import a provider-specific adapter except through a trait defined in the corresponding boundary module.
- Provider-specific types must be converted into domain types before crossing into `domain`, `risk`, or `strategy`.
- Error conversions must map provider errors into the repository error taxonomy in `errors`.
- Test-only helper imports must stay under `tests/` or `#[cfg(test)]` modules.

## Runtime Flow

```text
CLI command
  -> config validation
  -> secrets boundary
  -> app/service method
  -> market/LLM adapters or fixtures
  -> domain normalization
  -> strategy candidate
  -> deterministic risk gate
  -> audit append
  -> paper/live executor
  -> audit append
  -> structured terminal output and logs
```

## Data Flow

- Raw external payloads stay inside adapter modules.
- Normalized data uses domain structs.
- Order intent must include strategy ID, instrument, side, quantity, limit/market instruction, time-in-force, risk context ID, and mode.
- Audit records must be append-only and must not contain secrets.
- Live order acknowledgements must be persisted before terminal success output.

## Request / Command Flow

CLI commands are the external interaction contract. Initial planned commands:

- `optionclaw --help`
- `optionclaw check-config --config <path>`
- `optionclaw state init --data-dir <path>`
- `optionclaw state verify --data-dir <path>`
- `optionclaw paper run-once --config <path> --fixtures <path>`
- `optionclaw risk explain --config <path> --order-intent <path>`
- `optionclaw health --config <path>`

No live command may be implemented before EP-006 and EP-010 requirements pass.

## State Management Rules

- No global mutable trading state.
- Runtime state is passed explicitly through services or persisted through `persistence`.
- Config is immutable after validation.
- Secrets are loaded only at boundary execution time and zeroized where supported.
- Kill-switch state must be checked before every execution attempt.

## Persistence Boundaries

- No external database initially.
- Local data directory uses schema versioned files.
- Audit logs are append-only JSON Lines or another documented line-delimited format.
- State writes must be atomic: write temp file, fsync where practical, rename.
- Corrupt state must fail closed and preserve the original file for inspection.
- Migration must support dry-run and backup.

## External Integration Boundaries

Broker, exchange, market data, news, LLM, and wallet integrations must be adapter-based:

- Define a trait with domain inputs/outputs.
- Implement a mock/fixture adapter first.
- Add provider-specific adapter only after official docs, sandbox credentials, rate limits, and error contracts are verified.
- Provider credentials must be requested through the secrets boundary.
- No provider SDK may leak types into `domain`, `risk`, or `strategy`.

## Security Boundaries

- Secrets boundary owns decryption and redaction.
- Execution boundary owns order submission safety checks.
- Persistence boundary owns file permissions and encrypted secret storage.
- Observability boundary owns redaction before logs/metrics.
- CLI boundary owns explicit operator acknowledgement for dangerous commands.

## Validation Boundaries

- Config validation occurs before services run.
- Domain constructors validate invariants such as nonnegative prices, valid quantities, and known trading modes.
- Risk validation occurs after strategy/LLM output and immediately before execution.
- Adapter validation occurs before external payloads become domain structs.

## Error Handling Boundaries

- Domain errors are deterministic and user-actionable.
- Adapter errors include provider, operation, retryability, and redacted context.
- CLI converts errors into exit codes and readable messages.
- Logs include structured error fields without secrets.

## Observability Boundaries

- Use structured logging for command, mode, provider, strategy, risk decision, order-intent ID, latency, and result.
- Metrics must not expose secrets or exact private account identifiers.
- Health checks must report subsystem readiness without leaking credentials.

## Architectural Invariants

- Paper trading is the default.
- Risk gate cannot be bypassed by LLM output.
- Live execution cannot compile or run without explicit live-mode checks.
- Provider adapters cannot import CLI modules.
- Domain tests must not require network, secrets, time-of-day, or live market data.
- All external side effects are behind traits.
- Config keys are documented in `ENVIRONMENT.md` and specs before use.

## Forbidden Changes

- Adding a database without a new ExecPlan and architecture update.
- Adding web UI, SaaS tenancy, or authentication server without a new ExecPlan.
- Hard-coding broker, exchange, wallet, model, or API credentials.
- Implementing live trading before safety, security, and production readiness are complete.
- Bypassing audit logging for order-intent lifecycle events.
- Logging raw secrets, private keys, seed phrases, or full credential-bearing request payloads.

## How to Add a New Feature

1. Create or update a spec under `.agent/specs/`.
2. Create an ExecPlan from `.agent/templates/execplan-template.md`.
3. List expected changed files.
4. Add tests before or with implementation.
5. Update `COMMANDS.md` only if new commands are required.
6. Validate with scripts.
7. Record decisions in `DECISIONS.md` or an ADR when architectural.

## How to Add a New Dependency

1. Check `Cargo.toml` and existing code for an existing solution.
2. Confirm the dependency is required by the active ExecPlan.
3. Add it using an allowed command from `COMMANDS.md` or edit `Cargo.toml` only if necessary.
4. Record the reason, alternatives, and impact in the ExecPlan Decision Log.
5. Run install, typecheck, tests, and dependency audit.

## How to Modify Data Schema

1. Update `.agent/specs/SPEC-002-data-model.md`.
2. Add a schema version.
3. Add dry-run migration behavior.
4. Add backup/restore steps.
5. Add migration tests and corrupt-file tests.
6. Update `OPERATIONS.md` and `ROLLBACK.md`.

## How to Add a New Integration

1. Confirm the provider and mode are in scope.
2. Verify official API docs, sandbox availability, rate limits, authentication, and supported option instruments.
3. Define or extend a trait with domain types only.
4. Implement fixture/mock contract tests.
5. Implement sandbox adapter.
6. Add redaction and error mapping tests.
7. Live enablement requires explicit approval and production readiness.

## Architecture Review Checklist

- [ ] Only expected layers changed.
- [ ] Domain remains pure.
- [ ] No live trading bypasses risk gates.
- [ ] No provider type leaks across boundaries.
- [ ] No secrets in logs, fixtures, docs, or diffs.
- [ ] CLI behavior matches specs.
- [ ] Persistence changes include schema, backup, restore, and migration considerations.
- [ ] Tests cover success, rejection, and failure modes.
- [ ] `git diff --name-only` matches the active ExecPlan or extras are justified.
