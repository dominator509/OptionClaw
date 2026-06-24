# EP-002 Core Domain

## 1. Purpose / Big Picture

Implement OptionClaw's pure domain model and deterministic business logic for options trading decisions, without infrastructure leakage. This creates test-backed types and risk rules that all later adapters and CLI flows must use.

## 2. Scope

- Domain entities for option contracts, market snapshots, signals, account snapshots, positions, order intents, trading mode, and risk decisions.
- Deterministic risk rules and reason codes.
- Pure strategy candidate representation.
- Unit tests for invariants, validation, and risk rejections.

## 3. Non-goals

- No broker/exchange API calls.
- No wallet code.
- No LLM network calls.
- No file persistence.
- No CLI redesign beyond exposing domain types if necessary.
- No live order execution.

## 4. Context and Orientation

Domain code must compile and test without network, filesystem, secrets, or current market data. LLM output is advisory only. Every order intent must pass deterministic risk gates after strategy/LLM composition.

## 5. Files to Read First

- `ARCHITECTURE.md`
- `.agent/specs/SPEC-001-core-domain.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `src/domain/mod.rs`
- `src/risk/mod.rs`
- `src/strategy/mod.rs`
- `src/errors/mod.rs`
- Existing tests under `src/` and `tests/`

## 6. Files to Change

Expected changed files:

- `src/domain/mod.rs`
- `src/domain/instrument.rs`
- `src/domain/market.rs`
- `src/domain/order.rs`
- `src/domain/account.rs`
- `src/domain/signal.rs`
- `src/domain/risk.rs`
- `src/risk/mod.rs`
- `src/strategy/mod.rs`
- `src/errors/mod.rs`
- `src/lib.rs`
- Unit test modules in the same files or `src/domain/tests.rs`
- `.agent/execplans/EP-002-core-domain.md`

Forbidden changes:

- `src/persistence/`, `src/execution/`, provider adapters, secrets, CI, deployment unless needed only for imports from existing scaffold.

## 7. Interfaces and Contracts

Core types must provide constructors or validation functions that reject invalid values. Minimum contracts:

- `TradingMode::{Paper, Sandbox, Live}`.
- `OptionContract` with symbol/underlying, expiration, strike, option kind, venue/provider optional normalized ID.
- `MarketSnapshot` with timestamp, underlying price, option bid/ask/last, implied volatility if available.
- `Signal` with source kind, score, confidence, timestamp, and explanation.
- `OrderIntent` with ID, mode, contract, side, quantity, order type, limit price optional, strategy ID, created timestamp.
- `RiskLimits` with max account risk percent, max daily loss percent, max contracts/order, allow_live flag.
- `RiskDecision::{Accepted, Rejected}` with reason codes.

Use precise numeric types already in the repository. If none exist, add `rust_decimal` only after recording the dependency decision.

## 8. Milestones

### M1: Domain entities and validation

- Goal: Create pure domain structs/enums with validation.
- Files to read: `SPEC-001`, `ARCHITECTURE.md`, existing domain module.
- Files to change: `src/domain/*`, `src/errors/mod.rs`, `src/lib.rs`.
- Exact edits expected: Add structs/enums, constructors, domain error variants, module exports, and tests for invalid price/quantity/mode/expiration.
- Validation command: `./scripts/test-unit.sh`
- Expected result: `unit tests: ok`.
- Recovery instruction: If numeric/date dependencies are missing, check existing dependencies first; add the smallest necessary dependency and record it.

### M2: Risk limits and risk decision rules

- Goal: Implement deterministic risk gate behavior.
- Files to read: `src/risk/mod.rs`, `SPEC-001`, `SECURITY.md`.
- Files to change: `src/domain/risk.rs`, `src/risk/mod.rs`, tests.
- Exact edits expected: Add risk evaluation that rejects missing limits, live disabled, quantity over limit, estimated account risk over cap, daily loss over cap, kill switch active flag.
- Validation command: `./scripts/test-unit.sh`
- Expected result: `unit tests: ok` with accept/reject tests.
- Recovery instruction: If risk math is ambiguous, implement conservative fail-closed rule, record assumption, and continue.

### M3: Strategy candidate and LLM advisory boundary

- Goal: Represent strategy candidates and advisory model output without allowing bypass.
- Files to read: `src/strategy/mod.rs`, `src/llm/mod.rs`, `SPEC-001`.
- Files to change: `src/strategy/mod.rs`, `src/domain/signal.rs`, tests.
- Exact edits expected: Add candidate type and optional advisory score; tests prove rejected risk remains rejected even with high advisory score.
- Validation command: `./scripts/test-unit.sh`
- Expected result: `unit tests: ok`.
- Recovery instruction: If LLM module is not ready, keep advisory type in domain/strategy as plain data; do not call external APIs.

### M4: Error taxonomy integration

- Goal: Map domain/risk failures to stable error codes.
- Files to read: `SPEC-006`, `src/errors/mod.rs`.
- Files to change: `src/errors/mod.rs`, domain/risk tests.
- Exact edits expected: Add stable error/reason codes and Display messages suitable for CLI.
- Validation command: `./scripts/typecheck.sh && ./scripts/test-unit.sh`
- Expected result: `typecheck: ok`, `unit tests: ok`.
- Recovery instruction: If existing error crate differs, adapt to existing patterns and record decision.

### M5: Full domain validation

- Goal: Ensure no infrastructure leakage and final acceptance.
- Files to read: changed files and imports.
- Files to change: EP-002 progress/outcomes.
- Exact edits expected: Update progress, decisions, and outcomes; ensure domain modules do not import CLI, persistence, execution, secrets, or providers.
- Validation command: `./scripts/lint.sh && ./scripts/format-check.sh && ./scripts/typecheck.sh && ./scripts/test-unit.sh`
- Expected result: all commands print ok.
- Recovery instruction: If lint fails on dead code, add targeted tests/exports or adjust visibility; do not delete required types.

## 9. Concrete Steps

1. Run preflight.
2. Inspect current domain scaffold.
3. Implement pure entities.
4. Implement risk limits and decisions.
5. Add unit tests for every invariant.
6. Integrate error taxonomy.
7. Check imports for boundary violations.
8. Run validation and update this plan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/lint.sh
./scripts/format-check.sh
./scripts/typecheck.sh
./scripts/test-unit.sh
git diff --name-only
```

Acceptance criteria:

- Domain module has no infrastructure imports.
- Risk gate is deterministic and fail-closed.
- LLM advisory data cannot bypass risk rejection.
- Unit tests cover all required domain/risk behavior.
- Only expected files changed or extras justified.

## 11. Idempotence and Recovery

If types already exist, extend them without duplicate definitions. If constructor signatures need adjustment, update all tests in this plan only. If repeated failures occur, simplify the type surface while preserving required behavior.

## 12. Progress

- [x] M1 - Domain entities and validation. Validated with `cargo test --lib --bins --all-features --offline` after adding domain types and invariant checks.
- [x] M2 - Risk limits and risk decision rules. Validated with `cargo test --lib --bins --all-features --offline` after implementing deterministic fail-closed risk evaluation.
- [x] M3 - Strategy candidate and LLM advisory boundary. Validated with `cargo test --lib --bins --all-features --offline` after adding the advisory-only strategy candidate wrapper.
- [x] M4 - Error taxonomy integration. Validated with `cargo check --all-targets --all-features --offline` and `cargo test --lib --bins --all-features --offline`.
- [x] M5 - Full domain validation. Validated with `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features --offline -- -D warnings`, `cargo check --all-targets --all-features --offline`, and `cargo test --lib --bins --all-features --offline`.

## 13. Surprises & Discoveries

Record repository differences and validation failures here.

- The repository uses offline Cargo validation on this machine because the shell wrapper scripts are not directly runnable here; the documented Cargo fallback commands from `COMMANDS.md` were enough to finish validation.
- Clippy rejected a positional `OrderIntent::new` constructor once the domain surface grew, so the constructor now accepts an `OrderIntentSpec` wrapper to keep the API explicit and warning-free.
- `TradingMode` now lives in the domain module and is re-exported through config, which keeps config parsing aligned with the shared domain type.

## 14. Decision Log

Record dependency additions and risk-rule assumptions here.

- Kept the risk gate conservative: missing limits, live-disabled mode, quantity over limit, account-risk over cap, daily-loss over cap, and active kill switch all reject execution.
- Chose fixed-point micros for `Price` and bps for percentages so the core domain stays deterministic without adding new numeric dependencies.
- Used `OrderIntentSpec` as the public constructor payload so field-level validation stays intact without tripping Clippy's argument-count lint.

## 15. Outcomes & Retrospective

EP-002 is complete. The core domain now has validated option, market, signal, account, order, strategy, and risk types with stable domain and error codes. Validation passed offline with `cargo check --all-targets --all-features --offline`, `cargo test --lib --bins --all-features --offline`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets --all-features --offline -- -D warnings`.

Remaining risk is limited to future integration work: the domain layer is isolated and intentionally does not touch brokers, persistence, or live execution.
