# EP-004 API or Service Layer

## 1. Purpose / Big Picture

Implement OptionClaw's service-layer interaction boundary: validated service methods, provider-neutral adapter traits, paper execution workflow, request/response contracts, error mapping, and contract/integration tests.

## 2. Scope

- Service methods called by CLI.
- Adapter traits for market data, LLM advisory, paper executor, and future execution provider.
- Fixture/mock adapters.
- Request validation and response contracts.
- Error mapping into the taxonomy.
- Contract and integration tests.

## 3. Non-goals

- No public web API.
- No live broker/exchange adapter.
- No real wallet signing.
- No network calls in default tests.
- No new UI beyond service support for CLI.

## 4. Context and Orientation

OptionClaw is CLI-first, so the API boundary is internal Rust services plus CLI contracts. External provider integrations must use traits and mocks first. Live order submission remains forbidden.

## 5. Files to Read First

- `ARCHITECTURE.md`
- `.agent/specs/SPEC-003-api-contracts.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `src/cli/mod.rs`
- `src/config/mod.rs`
- `src/domain/*`
- `src/risk/mod.rs`
- `src/persistence/*`
- `src/execution/mod.rs`
- `src/llm/mod.rs`
- `src/market_data/mod.rs`

## 6. Files to Change

Expected changed files:

- `src/services/mod.rs`
- `src/services/config_service.rs`
- `src/services/state_service.rs`
- `src/services/paper_service.rs`
- `src/services/risk_service.rs`
- `src/services/health_service.rs`
- `src/market_data/mod.rs`
- `src/llm/mod.rs`
- `src/execution/mod.rs`
- `src/errors/mod.rs`
- `src/lib.rs`
- `fixtures/market/sample_snapshot.json`
- `fixtures/llm/sample_advisory.json`
- `tests/integration_services.rs`
- `tests/contract_adapters.rs`
- `.agent/execplans/EP-004-api-or-service-layer.md`

Forbidden changes:

- HTTP server routes.
- Real broker/wallet APIs.
- Live execution.
- Database setup.

## 7. Interfaces and Contracts

Required service contracts:

- `check_config(config_path) -> ConfigReport`
- `init_state(data_dir) -> StateReport`
- `verify_state(data_dir) -> StateReport`
- `run_paper_once(config_path, fixture_path) -> PaperRunReport`
- `explain_risk(config_path, order_intent_path) -> RiskReport`
- `health(config_path) -> HealthReport`

Required adapter traits:

- `MarketDataProvider::snapshot(request) -> MarketSnapshot`
- `LlmAdvisor::advise(context) -> AdvisoryResult`
- `PaperExecutor::execute(intent) -> ExecutionReport`
- `ExecutionProvider` trait may exist but live implementation must return `LIVE_TRADING_DISABLED` until future approval.

## 8. Milestones

### M1: Service module and config/state contracts

- Goal: Move CLI-facing logic into service methods.
- Files to read: config and persistence modules.
- Files to change: `src/services/*`, `src/lib.rs`, tests.
- Exact edits expected: Add config/state service methods and reports; unit/integration tests call services directly.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If service module conflicts with existing names, use existing app/service naming and update architecture docs.

### M2: Adapter traits and fixture providers

- Goal: Define provider-neutral traits and fake adapters.
- Files to read: `src/market_data/mod.rs`, `src/llm/mod.rs`, `src/execution/mod.rs`.
- Files to change: adapter modules, fixtures, contract tests.
- Exact edits expected: Add traits with domain inputs/outputs; add fixture provider implementations for tests.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If async runtime is not present, implement synchronous traits first and record decision.

### M3: Paper run-once workflow

- Goal: Create one end-to-end fixture-backed paper decision/execution flow.
- Files to read: domain/risk/persistence modules.
- Files to change: `src/services/paper_service.rs`, `src/execution/mod.rs`, `tests/integration_services.rs`.
- Exact edits expected: Load fixture, create candidate/order intent, run risk gate, append audit, simulate execution report.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok` with paper run success and rejection tests.
- Recovery instruction: If strategy logic is minimal, implement deterministic placeholder strategy that only uses fixture input and is clearly marked for future strategy plans.

### M4: Risk explain and health contracts

- Goal: Add service reports for risk explanation and health.
- Files to read: `SPEC-003`, `SPEC-007`.
- Files to change: `src/services/risk_service.rs`, `src/services/health_service.rs`, tests.
- Exact edits expected: Evaluate serialized intent, produce reason codes; health reports config/state/mock provider readiness.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If serialization format is not yet stable, define JSON fixture format in tests and document in spec.

### M5: Final service validation

- Goal: Ensure contracts are documented, tested, and do not expose live behavior.
- Files to read: changed services/adapters.
- Files to change: EP-004 progress/outcomes, relevant docs if contracts changed.
- Exact edits expected: Update plan and decisions.
- Validation command: `./scripts/lint.sh && ./scripts/format-check.sh && ./scripts/typecheck.sh && ./scripts/test-integration.sh`
- Expected result: all commands print ok.
- Recovery instruction: If default tests attempt network, replace with fixtures and record decision.

## 9. Concrete Steps

1. Run preflight.
2. Inspect current CLI and modules.
3. Add services.
4. Add adapter traits and fixtures.
5. Implement paper run-once flow.
6. Implement risk and health reports.
7. Add contract/integration tests.
8. Validate and update plan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/lint.sh
./scripts/format-check.sh
./scripts/typecheck.sh
./scripts/test-integration.sh
git diff --name-only
```

Acceptance criteria:

- Service methods exist and are tested.
- Adapter traits use domain types.
- Fixture providers support tests without network/secrets.
- Paper run-once records audit and produces report.
- Live execution remains disabled.

## 11. Idempotence and Recovery

If services already exist, extend them instead of duplicating. If a fixture format changes, update all related tests and docs in the same milestone. On repeated failures, reduce paper flow to the smallest deterministic path that still exercises risk and audit.

## 12. Progress

- [x] M1 - Service module and config/state contracts.
- [x] M2 - Adapter traits and fixture providers.
- [x] M3 - Paper run-once workflow.
- [x] M4 - Risk explain and health contracts.
- [x] M5 - Final service validation.

## 13. Surprises & Discoveries

Record repository differences and validation failures here.

- `./scripts/preflight.sh` failed in `cmd.exe` with `'.' is not recognized as an internal or external command, operable program or batch file.` The native Cargo fallback commands in `COMMANDS.md` were used for validation instead.
- `cargo run --offline -- paper run-once --config config/example.toml --fixtures fixtures/market/sample_snapshot.json` failed because the current CLI does not yet expose a `paper` command. The service-layer implementation and integration tests cover the workflow, but the CLI smoke command remains unavailable.

## 14. Decision Log

Record adapter, sync/async, and fixture-format decisions here.

- Added synchronous, fixture-backed provider traits for market data, LLM advisory, and paper execution to keep the service layer deterministic and testable without network access.
- Introduced `AppError::Input(InputError::Invalid { .. })` for malformed service request and fixture JSON so contract failures have a stable, user-facing taxonomy.
- Used repository-local fixtures under `fixtures/market/sample_snapshot.json` and `fixtures/llm/sample_advisory.json` for paper workflow validation instead of external services.
- Kept the live execution provider disabled with a deterministic `LIVE_TRADING_DISABLED` status to preserve the repo's default safety posture.

## 15. Outcomes & Retrospective

Complete after M5.

EP-004 is implemented at the service layer and validated with integration and adapter tests. The new service contracts now support config inspection, state initialization and verification, risk explanation, health reporting, and a deterministic paper run-once workflow with audit and state updates.

Validation completed successfully with native Cargo fallbacks:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features --offline -- -D warnings`
- `cargo check --all-targets --all-features --offline`
- `cargo test --lib --bins --all-features --offline`
- `cargo test --test integration_services --all-features --offline`
- `cargo test --test contract_adapters --all-features --offline`
- `cargo test --test integration_smoke --all-features --offline`
- `cargo test --test integration_persistence --all-features --offline`

The only unresolved repository mismatch is the absent `paper run-once` CLI command referenced by `COMMANDS.md`; the service layer itself is complete, but the CLI smoke path is not yet wired in this checkout.

As a quick confirmation of the existing CLI surface, `cargo run --offline -- check-config --config config/example.toml` succeeded and reported `config ok: mode=paper`.
