# EP-005 User Interface or Client

## 1. Purpose / Big Picture

Implement the OptionClaw CLI user interaction layer: commands, help text, success/error output, empty states, loading/progress states where needed, and E2E acceptance tests.

## 2. Scope

- CLI commands for config, state, paper, risk, and health service methods.
- User-visible output contracts.
- Exit codes.
- Error states and empty states.
- Accessibility for terminal users.
- E2E tests.

## 3. Non-goals

- No web UI.
- No mobile UI.
- No background daemon UI.
- No live trading command unless it refuses safely.
- No strategy marketplace or social features.

## 4. Context and Orientation

The UI is the CLI. All command behavior must be scriptable, readable, and safe. CLI should call service-layer methods rather than embedding business logic.

## 5. Files to Read First

- `.agent/specs/SPEC-004-ui-ux-behavior.md`
- `.agent/specs/SPEC-003-api-contracts.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `src/cli/mod.rs`
- `src/main.rs`
- `src/services/*`
- `tests/e2e_cli.rs`
- `COMMANDS.md`

## 6. Files to Change

Expected changed files:

- `src/main.rs`
- `src/cli/mod.rs`
- `src/cli/output.rs`
- `src/cli/commands.rs`
- `src/errors/mod.rs`
- `tests/e2e_cli.rs`
- `fixtures/config/invalid_config.toml`
- `fixtures/orders/sample_order_intent.json`
- `COMMANDS.md` if smoke commands change
- `.agent/execplans/EP-005-user-interface-or-client.md`

Forbidden changes:

- Web routes.
- Live execution implementation.
- Provider network calls in E2E tests.
- Secrets in fixtures.

## 7. Interfaces and Contracts

Required commands after this plan:

```text
optionclaw --help
optionclaw --version
optionclaw check-config --config <path>
optionclaw state init --data-dir <path>
optionclaw state verify --data-dir <path>
optionclaw paper run-once --config <path> --fixtures <path>
optionclaw risk explain --config <path> --order-intent <path>
optionclaw health --config <path>
```

Output must include mode where relevant. Errors must include stable error code and safe next action.

## 8. Milestones

### M1: CLI command routing

- Goal: Add command enum/subcommands that call services.
- Files to read: service modules and CLI module.
- Files to change: `src/cli/mod.rs`, `src/cli/commands.rs`, `src/main.rs`.
- Exact edits expected: Add required subcommands and route to service methods; preserve help/version.
- Validation command: `./scripts/typecheck.sh`
- Expected result: `typecheck: ok`.
- Recovery instruction: If CLI parsing crate behavior differs, inspect current parser usage and make smallest compatible change.

### M2: Output, empty states, and errors

- Goal: Implement stable success/error output.
- Files to read: `SPEC-004`, `SPEC-006`.
- Files to change: `src/cli/output.rs`, `src/errors/mod.rs`, `tests/e2e_cli.rs`.
- Exact edits expected: Add output helpers, redacted error presentation, empty state messages for uninitialized state/no audit.
- Validation command: `./scripts/test-e2e.sh`
- Expected result: `e2e tests: ok` for help and error cases.
- Recovery instruction: If exact output is brittle, use predicate tests for required phrases and codes, not full snapshots.

### M3: State, paper, risk, and health command E2E tests

- Goal: Verify user flows end-to-end.
- Files to read: fixtures, services, e2e tests.
- Files to change: `tests/e2e_cli.rs`, fixtures, CLI modules.
- Exact edits expected: Add success/failure tests for state init/verify, paper run-once, risk explain, health.
- Validation command: `./scripts/test-e2e.sh`
- Expected result: `e2e tests: ok`.
- Recovery instruction: If commands depend on prior ExecPlans not complete, implement the smallest service call wrapper or fail clearly with documented error if out of scope.

### M4: CLI accessibility and smoke command sync

- Goal: Ensure CLI is usable and scripts reflect commands.
- Files to read: `COMMANDS.md`, `scripts/smoke-test.sh`.
- Files to change: CLI output/tests, `COMMANDS.md`, `scripts/smoke-test.sh` if command contract changed.
- Exact edits expected: Ensure non-color-only output, help for subcommands, smoke tests run current commands.
- Validation command: `./scripts/smoke-test.sh`
- Expected result: `smoke test: ok`.
- Recovery instruction: If smoke fails due missing fixture, add fake fixture or adjust smoke to help/check-config only with documented reason.

### M5: Final CLI validation

- Goal: Verify CLI layer is complete.
- Files to read: changed files.
- Files to change: EP-005 progress/outcomes.
- Exact edits expected: Update plan and decisions.
- Validation command: `./scripts/lint.sh && ./scripts/format-check.sh && ./scripts/typecheck.sh && ./scripts/test-e2e.sh && ./scripts/smoke-test.sh`
- Expected result: all commands print ok.
- Recovery instruction: If one E2E is flaky, remove nondeterminism; do not skip the flow.

## 9. Concrete Steps

1. Run preflight.
2. Inspect service contracts.
3. Add CLI subcommands.
4. Add output helpers and errors.
5. Add E2E tests.
6. Sync smoke script and commands if needed.
7. Validate and update plan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/lint.sh
./scripts/format-check.sh
./scripts/typecheck.sh
./scripts/test-e2e.sh
./scripts/smoke-test.sh
git diff --name-only
```

Acceptance criteria:

- Required CLI commands exist.
- Help and errors are readable.
- No output requires color.
- Live behavior refuses safely.
- E2E and smoke tests pass.

## 11. Idempotence and Recovery

If commands already exist, extend tests and output without changing names unless spec requires it. If CLI args conflict, prefer backwards-compatible aliases and document decision.

## 12. Progress

- [x] M1 - CLI command routing. Completed 2026-06-23. Validation: `cargo check --all-targets --all-features --offline` -> passed.
- [x] M2 - Output, empty states, and errors. Completed 2026-06-23. Validation: `cargo test --test e2e_cli --all-features --offline` -> passed.
- [x] M3 - State, paper, risk, and health command E2E tests. Completed 2026-06-23. Validation: `cargo test --test e2e_cli --all-features --offline` -> passed.
- [x] M4 - CLI accessibility and smoke command sync. Completed 2026-06-23. Validation: `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, and `cargo test --test integration_smoke --all-features --offline` -> passed.
- [x] M5 - Final CLI validation. Completed 2026-06-23. Validation: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features --offline -- -D warnings`, `cargo test --lib --bins --all-features --offline`, `cargo test --test e2e_cli --all-features --offline`, and `cargo test --test integration_smoke --all-features --offline` -> passed.

## 13. Surprises & Discoveries

Record repository differences and validation failures here.

- `./scripts/preflight.sh` failed under `rtk` with `[rtk: %1 is not a valid Win32 application. (os error 193)]`, so the native Cargo fallback commands from `COMMANDS.md` were used for validation.
- The CLI parser is manual and synchronous because the manifest does not include a CLI parsing dependency; that kept the change set narrow and avoided adding a new crate.
- Smoke verification used the documented CLI commands `optionclaw --help` and `optionclaw check-config --config config/example.toml`, which now reflect the expanded command surface.

## 14. Decision Log

Record CLI naming and output contract decisions here.

- Implemented a manual command router with plain-text output helpers rather than adding a parsing dependency, because the manifest has no CLI crate and the required surface is small and deterministic.
- Added stable CLI error codes `CLI_UNKNOWN_COMMAND`, `CLI_UNKNOWN_SUBCOMMAND`, `CLI_MISSING_ARGUMENT`, and `CLI_UNEXPECTED_ARGUMENT` for parse-time failures.
- Kept `check-config` output script-friendly with a stable `config ok: mode=...` line and extended the new commands with `mode=` where relevant.
- Added repository-local fixtures for invalid config and serialized order-intent input so the E2E tests do not require network access.
- Wired `paper run-once` to refuse live mode through the service layer and surface the existing `INPUT_INVALID` error code instead of creating a separate live execution command.

## 15. Outcomes & Retrospective

Complete after M5.

EP-005 is implemented at the CLI boundary and validated with help/version output, config validation, state init/verify, paper run-once, risk explain, and health command coverage.

Validation completed successfully with native Cargo fallbacks:

- `cargo check --all-targets --all-features --offline`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features --offline -- -D warnings`
- `cargo test --lib --bins --all-features --offline`
- `cargo test --test e2e_cli --all-features --offline`
- `cargo test --test integration_smoke --all-features --offline`
- `cargo run --offline -- --help`
- `cargo run --offline -- check-config --config config/example.toml`

The only environment-specific issue was the `rtk`/script-wrapper failure during preflight, so the documented Cargo fallback path was used for the rest of the validation. The CLI now exposes the required command surface and keeps live behavior safely refused.
