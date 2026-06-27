# EP-011 Live Execution Approval and ROI Evidence Gate

## 1. Purpose / Big Picture

Build a dedicated live-readiness layer that can produce internal software approval for Alpaca live execution while still failing closed unless runtime secrets, explicit live enablement, risk limits, an inactive kill switch, provider/account capability, and a fresh ROI approval artifact are all present.

This plan treats high net ROI as an empirical gate over backtest and forward-paper evidence. It does not promise future returns or guarantee broker/KYC/options approval.

## 2. Scope

- Add a live-readiness spec and this ExecPlan.
- Add Alpaca as the first live provider behind existing provider abstractions.
- Add env/config contracts for Alpaca credentials, provider environment, risk caps, kill switch, and approval artifact paths.
- Add research commands that produce ROI evidence and write an internal approval artifact.
- Add live commands that check readiness and submit only long calls/puts after every gate passes.
- Add tests for ROI gates, approval freshness/hash matching, Alpaca contract mapping, live check, live submit refusal paths, kill switch blocking, and redacted output.
- Add documentation for environment, security, testing, production readiness, and operations.

## 3. Non-goals

- No guaranteed profit, guaranteed ROI, or investment advice.
- No broker/KYC/options approval guarantees.
- No tax, legal, regulatory, or custody advice.
- No unattended fund movement.
- No real live order submission during implementation or tests.
- No private-key custody, wallet signing, crypto-funded workflow, margin expansion, exercise/DNE automation, covered calls, cash-secured puts, spreads, or second broker adapter.
- No implementation directly from `ROADMAP.md`.

## 4. Context and Orientation

OptionClaw is Rust-first and currently production-ready for paper/local operation. EP-011 narrows live readiness to Alpaca US-listed equity options, using level-2 long calls and long puts only. Official Alpaca documentation confirms options trading support, options levels, sandbox approval fixtures, market data, and market/limit day orders. Real provider credentials remain env-only and must never be stored in config, logs, fixtures, audit records, reports, or approval artifacts.

## 5. Files to Read First

- `AGENTS.md`
- `COMMANDS.md`
- `.agent/PLANS.md`
- `.agent/execplans/EP-010-production-readiness.md`
- `PRODUCTION_READINESS.md`
- `SECURITY.md`
- `ENVIRONMENT.md`
- `ARCHITECTURE.md`
- `Cargo.toml`
- `Cargo.lock`
- `src/config/mod.rs`
- `src/execution/mod.rs`
- `src/market_data/mod.rs`
- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/cli/output.rs`
- `src/services/mod.rs`
- `src/services/risk_service.rs`
- `src/domain/order.rs`
- `src/domain/instrument.rs`
- `src/domain/risk.rs`
- `src/errors/mod.rs`

## 6. Files to Change

Expected changed files:

- `COMMANDS.md`
- `Cargo.toml`
- `Cargo.lock`
- `ENVIRONMENT.md`
- `SECURITY.md`
- `TESTING.md`
- `OBSERVABILITY.md`
- `OPERATIONS.md`
- `PRODUCTION_READINESS.md`
- `ASSUMPTIONS.md`
- `DECISIONS.md`
- `.agent/specs/SPEC-009-live-execution-approval.md`
- `.agent/execplans/EP-011-live-execution-approval.md`
- `config/live.example.toml`
- `fixtures/research/aggressive_growth.json`
- `fixtures/live/long_call_order.json`
- `src/lib.rs`
- `src/config/mod.rs`
- `src/execution/mod.rs`
- `src/alpaca/mod.rs`
- `src/services/mod.rs`
- `src/services/research_service.rs`
- `src/services/live_service.rs`
- `src/services/risk_service.rs`
- `src/cli/commands.rs`
- `src/cli/mod.rs`
- `src/cli/output.rs`
- `tests/contract_alpaca.rs`
- `tests/integration_live.rs`
- `tests/integration_security.rs`
- `tests/e2e_live_cli.rs`

Forbidden changes:

- Real credentials or production data.
- Real live orders or network calls to real broker endpoints during tests.
- Broad strategy, UI, database, cloud, or deployment changes.
- Additional broker SDKs or wallet/private-key support.

## 7. Interfaces and Contracts

Config/env:

- `provider = "alpaca"` is required for live mode.
- `provider_environment = "paper" | "sandbox" | "live"` selects Alpaca base URL defaults.
- `OPTIONCLAW_ALPACA_API_KEY` is required at runtime for live checks/submits.
- `OPTIONCLAW_ALPACA_API_SECRET` is required at runtime for live checks/submits.
- `OPTIONCLAW_ENABLE_LIVE_TRADING=true` is required at runtime for live checks/submits.
- `max_account_risk_bps`, `max_daily_loss_bps`, `max_contracts_per_order`, `kill_switch_file`, and `approval_artifact` are required for live mode.
- `alpaca_base_url` is optional and exists only for tests/mocks/sandbox fixtures.

CLI:

```sh
optionclaw research backtest --config <path> --fixtures <dir-or-file>
optionclaw research approve --config <path> --report <path>
optionclaw live check --config <path>
optionclaw live submit --config <path> --order-intent <path> --confirm-live
```

Approval artifact:

- Must be approved.
- Must be no older than seven days.
- Must match current strategy ID, risk profile ID, and config hash.
- Must meet ROI, drawdown, trade count, profit factor, and zero-bypass thresholds.

Live submit:

- Requires `--confirm-live`.
- Requires `trading_mode = "live"`.
- Requires `OPTIONCLAW_ENABLE_LIVE_TRADING=true`.
- Requires valid env-only Alpaca credentials.
- Requires provider/account options capability level 2 or higher.
- Requires inactive kill switch.
- Permits only buy-side long calls and long puts.

## 8. Milestones

### M1: Plan, preflight, dependency, and contract baseline

- Goal: Establish the active plan and dependency/config contracts.
- Files to read: source-of-truth docs and current config/dependency files.
- Files to change: `COMMANDS.md`, `Cargo.toml`, `Cargo.lock`, `.agent/execplans/EP-011-live-execution-approval.md`, `.agent/specs/SPEC-009-live-execution-approval.md`.
- Exact edits expected: Add reqwest/httpmock dependency contracts, allowed commands, plan, and spec.
- Validation command: `./scripts/preflight.sh`.
- Expected result: Command exits successfully.
- Recovery instruction: If preflight cannot run, record the exact host wrapper behavior and use native documented Cargo fallback only if already listed in `COMMANDS.md`.

### M2: Research ROI evidence and approval artifact

- Goal: Produce and validate aggressive-growth ROI evidence and approval artifacts.
- Files to read: config, services, CLI output.
- Files to change: `src/services/research_service.rs`, `src/services/mod.rs`, `src/cli/commands.rs`, `src/cli/mod.rs`, `src/cli/output.rs`, fixtures.
- Exact edits expected: Add backtest/approve services, CLI commands, artifact hashing, freshness checks, and threshold tests.
- Validation command: `cargo test --lib --bins --all-features --offline`.
- Expected result: Unit/bin tests pass.
- Recovery instruction: Fix only targeted compile or test failures; do not relax ROI thresholds.

### M3: Alpaca adapter and live gate services

- Goal: Add Alpaca adapter calls and fail-closed live check/submit services.
- Files to read: execution, risk, domain, errors, persistence.
- Files to change: `src/alpaca/mod.rs`, `src/services/live_service.rs`, `src/config/mod.rs`, `src/execution/mod.rs`, `src/services/risk_service.rs`.
- Exact edits expected: Add account/status, contract, preview, submit, cancel, status polling mapping, live gate checks, risk gates, kill-switch checks, and single-submission path.
- Validation command: `cargo check --all-targets --all-features --offline`.
- Expected result: All targets compile offline.
- Recovery instruction: Fix adapter/schema mismatches without adding real network calls or real credentials.

### M4: Contract, integration, and E2E coverage

- Goal: Prove live gates with mocks and refusal tests.
- Files to read: existing test style and fixtures.
- Files to change: `tests/contract_alpaca.rs`, `tests/integration_live.rs`, `tests/e2e_live_cli.rs`, fixtures.
- Exact edits expected: Add mocked Alpaca contract tests, live check integration tests, live submit refusal tests, and redacted-output assertions.
- Validation command: `cargo test --test integration_live --all-features --offline`, `cargo test --test contract_alpaca --all-features --offline`, `cargo test --test e2e_live_cli --all-features --offline`.
- Expected result: New live tests pass offline.
- Recovery instruction: Use mock servers and temp files only; never call real Alpaca endpoints.

### M5: Documentation, readiness, and final verification

- Goal: Update live-readiness docs, run full verification, and complete outcomes.
- Files to read: environment, security, testing, operations, production readiness docs.
- Files to change: docs, this ExecPlan, command docs if needed.
- Exact edits expected: Document contracts, risks, live approval boundaries, and final results.
- Validation command: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo check --all-targets --all-features --offline`, `cargo test --lib --bins --all-features --offline`, `cargo test --test integration_live --all-features --offline`, `cargo test --test contract_alpaca --all-features --offline`, `cargo test --test e2e_live_cli --all-features --offline`, `cargo audit --no-fetch --stale`, `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, `cargo run --offline -- research backtest --config config/live.example.toml --fixtures fixtures/research/aggressive_growth.json`, `git diff --name-only`.
- Expected result: All validations pass or documented native fallback passes; diff matches expected files or extras are justified.
- Recovery instruction: Apply anti-fixation; STOP only for AGENTS.md STOP conditions.

## 9. Concrete Steps

1. Read required docs and files.
2. Run preflight.
3. Add/update plan, spec, commands, and dependencies.
4. Implement research ROI evidence and approval artifact.
5. Implement Alpaca adapter and live gates.
6. Wire CLI commands and redacted output.
7. Add fixtures and tests.
8. Update docs and readiness notes.
9. Run required validation commands.
10. Run `git diff --name-only`.
11. Update outcomes and final response.

## 10. Validation and Acceptance

Required final validation:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features --offline
cargo test --lib --bins --all-features --offline
cargo test --test integration_live --all-features --offline
cargo test --test contract_alpaca --all-features --offline
cargo test --test e2e_live_cli --all-features --offline
cargo audit --no-fetch --stale
cargo run --offline -- --help
cargo run --offline -- check-config --config config/example.toml
cargo run --offline -- research backtest --config config/live.example.toml --fixtures fixtures/research/aggressive_growth.json
git diff --name-only
```

Acceptance criteria:

- `reqwest` and mock HTTP dev dependency are present.
- Live config/env contract is documented and enforced.
- Alpaca adapter maps account, contract, order submit, cancel, and status responses through mocks.
- ROI approval enforces all aggressive-growth thresholds.
- Live check fails closed for missing env, stale approval, mismatched hash, insufficient options level, and active kill switch.
- Live submit refuses without `--confirm-live`, refuses non-live order intents, and submits exactly once through a mock after gates pass.
- CLI output does not print Alpaca secrets.
- Docs state that software approval is internal only and does not guarantee profit or broker approval.

## 11. Idempotence and Recovery

Re-running research approval overwrites only repository-local generated approval/report files under `var/dev`. Tests use temp directories and mock HTTP servers. Live commands fail closed when credentials, approval, provider status, risk caps, kill switch, or explicit confirmation are missing. Do not clear kill switches, write secrets, or submit live orders while implementing this plan.

## 12. Progress

- [x] M1 - Completed 2026-06-25. Validation: `./scripts/preflight.sh` -> exit code 0 with no stdout on this Windows/PowerShell host; command did not print the documented `preflight: ok` line.
- [x] M2 - Completed 2026-06-25. Validation: `C:\Users\domin\.cargo\bin\cargo.exe test --lib --bins --all-features --offline` -> pass, 47 tests passed. First offline attempt failed because the newly required `httpmock` transitive crate `assert-json-diff v2.0.2` was not cached; reran the documented non-offline Cargo command with network approval to fetch EP-011 dependencies, then reran offline successfully.
- [x] M3 - Completed 2026-06-25. Validation: `C:\Users\domin\.cargo\bin\cargo.exe check --all-targets --all-features --offline` -> pass. First check failed because `tests/integration_security.rs` constructed `AppConfig` directly and needed `..AppConfig::default()` after EP-011 added live config fields; patched narrowly and reran successfully.
- [x] M4 - Completed 2026-06-25. Validation: `C:\Users\domin\.cargo\bin\cargo.exe test --test integration_live --all-features --offline` -> pass, 7 tests passed; `C:\Users\domin\.cargo\bin\cargo.exe test --test contract_alpaca --all-features --offline` -> pass, 5 tests passed; `C:\Users\domin\.cargo\bin\cargo.exe test --test e2e_live_cli --all-features --offline` -> pass, 3 tests passed.
- [x] M5 - Completed 2026-06-25. Validation: `C:\Users\domin\.cargo\bin\cargo.exe fmt --all -- --check` -> pass; `C:\Users\domin\.cargo\bin\cargo.exe clippy --all-targets --all-features -- -D warnings` -> pass; `C:\Users\domin\.cargo\bin\cargo.exe check --all-targets --all-features --offline` -> pass; `C:\Users\domin\.cargo\bin\cargo.exe test --lib --bins --all-features --offline` -> pass, 47 tests passed; existing integration/contract/E2E suites passed; `C:\Users\domin\.cargo\bin\cargo.exe audit --no-fetch --stale` -> exit 0 with stale/no-fetch package-cache warning; `C:\Users\domin\.cargo\bin\cargo.exe run --offline -- --help`, `check-config`, `health`, `research backtest`, and `research approve` -> pass; `C:\Users\domin\.cargo\bin\cargo.exe build --release --offline` -> pass; `./scripts/verify.sh` and `./scripts/production-readiness-check.sh` -> exit code 0 with no stdout on this host; `git diff --name-only` -> reviewed tracked changed files and showed only expected tracked paths plus line-ending warnings.

## 13. Surprises & Discoveries

- `./scripts/preflight.sh` exited 0 but did not emit the documented `preflight: ok` output under the current shell.
- The repo contains `.agent/specs/SPEC-008-production-readiness.md`; there is no `.agent/specs/SPEC-008-observability-and-operations.md` file.
- `cmd.exe`/PowerShell PATH did not expose `cargo` reliably in prior validation, so `COMMANDS.md` documents `C:\Users\domin\.cargo\bin\cargo.exe <documented-cargo-arguments>` as the local Cargo equivalent.
- The first M2 offline validation could not run until Cargo fetched `httpmock` transitive crates; once cached, the offline test command passed.
- `tests/integration_security.rs` used a direct `AppConfig` struct literal; the new live config fields required adding `..AppConfig::default()` to keep the paper-mode security test scoped.
- `httpmock` 0.8 exposes `assert_calls` as the non-deprecated mock assertion API, so the new tests use that instead of deprecated `assert_hits`.
- Direct `./scripts/*.sh` invocation under PowerShell exits 0 but does not emit the documented success output; `sh` is not installed, so native Cargo fallbacks provide the meaningful validation evidence on this host.
- Existing `e2e_cli` tests caught that a minimal live config returned a missing-provider error before the explicit live-disabled gate. The validation order now reports `LIVE_TRADING_DISABLED` first unless the operator sets `OPTIONCLAW_ENABLE_LIVE_TRADING=true`.
- `git diff --name-only` emits LF-to-CRLF warnings for modified text files on this Windows checkout.

## 14. Decision Log

- Interpret "100% approval" as repo-controlled internal software approval only; external broker/KYC/options approval remains outside code and cannot be guaranteed.
- Interpret "high net ROI" as an evidence threshold over backtest and forward-paper metrics, not a guarantee of future returns.
- Add `reqwest 0.12` with blocking/json/rustls TLS and `httpmock` for mock-only contract tests because EP-011 explicitly requires synchronous Alpaca HTTP calls and mock HTTP coverage.
- Keep Alpaca credentials env-only and never serialize them into configs, fixtures, approval artifacts, audit records, or CLI output.
- Use Alpaca level-2 long calls/puts only for the first live release.
- Include `tests/integration_security.rs` in expected changes because EP-011 expanded `AppConfig`, and one existing paper-mode security test needed a default rest initializer.
- Document `cargo fmt --all` and `sh ./scripts/*.sh` fallback commands in `COMMANDS.md`; `sh` is unavailable on this host, but documenting the fallback prevents future agents from inventing it.
- Validate live submit with mocks only. Real `live check` against Alpaca was not run because runtime Alpaca credentials/account approval were not supplied and would be a STOP condition for real provider verification.

## 15. Outcomes & Retrospective

EP-011 is complete for internal software live-readiness approval. The implementation adds Alpaca as the first live provider, env-only credential gates, live config contracts, aggressive ROI evidence and approval artifacts, live check/submit CLI commands, single-submit enforcement, kill-switch blocking, account/options capability checks, and mock-backed contract/integration/E2E coverage.

Acceptance status:

- Dependency contract: passed. `reqwest 0.12` with blocking/json/rustls TLS and `httpmock 0.8.3` are present in `Cargo.toml`/`Cargo.lock`.
- ROI gate: passed. Backtest and approval commands produce a signed, expiring internal approval artifact, and tests cover threshold/stale failures.
- Live check gates: passed in mocks. Tests cover missing secrets, stale approval, config-hash mismatch, insufficient options level, provider errors, and active kill switch.
- Live submit gates: passed in mocks. Tests cover missing `--confirm-live`, paper/sandbox mode refusal, stale ROI evidence, redacted output, risk gate acceptance, and exactly one mock provider submission.
- Existing regression surface: passed. Unit/bin, integration smoke/persistence/services/security/live, contract adapters/Alpaca, E2E CLI/live, clippy, typecheck, release build, audit, and CLI smoke commands all passed via documented commands or native fallback.

Remaining risks:

- Real Alpaca credentials, account status, options level, market access, and broker/KYC approval were not verified and cannot be guaranteed by code.
- Internal software approval is not legal, tax, regulatory, custody, or investment advice.
- ROI thresholds are empirical evidence gates, not a guarantee of future returns.
- POSIX shell wrappers do not produce documented output under this PowerShell host, and `sh` is unavailable; native Cargo fallbacks are the reliable local validation path.
- First release supports only Alpaca level-2 long calls and long puts; spreads, covered calls, cash-secured puts, exercise/DNE automation, margin expansion, crypto funding, private-key custody, and additional brokers remain out of scope.

Production-readiness status:

- Paper/local production readiness remains intact.
- EP-011 internal live software readiness passed with mocks and local fixtures.
- Real live production execution remains incomplete until an operator supplies env-only Alpaca credentials, confirms external broker/options approval, reruns `live check` against the intended provider environment, confirms the kill switch and risk caps, and explicitly invokes `live submit --confirm-live`.
