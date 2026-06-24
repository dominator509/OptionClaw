# EP-001 Foundation

## 1. Purpose / Big Picture

Establish the Rust repository foundation for OptionClaw: Cargo project structure, CLI scaffold, formatting, linting, typechecking, tests, CI, environment validation, verify script compatibility, and documentation baseline.

## 2. Scope

- Create or normalize a Rust Cargo binary/library project named `optionclaw`.
- Add baseline module structure matching `ARCHITECTURE.md`.
- Add CLI help and config-check placeholder behavior in paper mode.
- Add formatting/lint/typecheck/test/build compatibility.
- Add baseline unit, integration, and E2E tests.
- Add CI workflow if no equivalent exists.
- Add example config and fixture directories.

## 3. Non-goals

- No trading strategy implementation.
- No live broker/exchange integration.
- No wallet signing.
- No encrypted secrets implementation yet.
- No database.
- No production deployment.

## 4. Context and Orientation

This is the first implementation plan after discovery. If `Cargo.toml` already exists, preserve existing working patterns and adapt only the minimum required foundation. If the repository is empty, create a standard Cargo project without using commands not listed in `COMMANDS.md` unless `COMMANDS.md` is updated first.

## 5. Files to Read First

- `AGENTS.md`
- `COMMANDS.md`
- `ARCHITECTURE.md`
- `TESTING.md`
- `ENVIRONMENT.md`
- `.agent/specs/SPEC-000-product-scope.md`
- `.agent/specs/SPEC-004-ui-ux-behavior.md`
- Existing `Cargo.toml` if present
- Existing `src/` and `tests/` if present

## 6. Files to Change

Expected changed files:

- `Cargo.toml`
- `Cargo.lock`
- `src/main.rs`
- `src/lib.rs`
- `src/cli/mod.rs`
- `src/config/mod.rs`
- `src/domain/mod.rs`
- `src/risk/mod.rs`
- `src/strategy/mod.rs`
- `src/llm/mod.rs`
- `src/market_data/mod.rs`
- `src/execution/mod.rs`
- `src/persistence/mod.rs`
- `src/secrets/mod.rs`
- `src/observability/mod.rs`
- `src/errors/mod.rs`
- `tests/integration_smoke.rs`
- `tests/e2e_cli.rs`
- `config/example.toml`
- `fixtures/README.md`
- `.github/workflows/ci.yml`
- `.gitignore`
- `COMMANDS.md` if commands are adjusted from evidence
- `.agent/execplans/EP-001-foundation.md`

Forbidden changes:

- Broker/wallet/live trading code.
- Real secrets.
- Large architecture rewrites.
- Database setup.

## 7. Interfaces and Contracts

Initial CLI contracts:

```text
optionclaw --help
optionclaw --version
optionclaw check-config --config config/example.toml
```

`check-config` must parse the example config and report paper mode. It must not contact networks or require secrets.

Recommended dependencies for greenfield foundation, only after checking existing dependencies:

- `clap` with derive feature for CLI.
- `serde` and `toml` for config parsing.
- `thiserror` for typed errors.
- `anyhow` only at CLI boundary if needed.
- `tracing` and `tracing-subscriber` for future logs.
- Test dev dependencies: `assert_cmd`, `predicates`, `tempfile`.

Record every dependency addition in Decision Log.

## 8. Milestones

### M1: Cargo project and module skeleton

- Goal: Create Rust package and module directories.
- Files to read: existing manifests and `ARCHITECTURE.md`.
- Files to change: `Cargo.toml`, `Cargo.lock`, `src/main.rs`, `src/lib.rs`, module `mod.rs` files, `.gitignore`.
- Exact edits expected: Define package `optionclaw`, binary target, library modules, and no-op module exports.
- Validation command: `./scripts/typecheck.sh`
- Expected result: `typecheck: ok`.
- Recovery instruction: If `Cargo.toml` missing caused earlier script failure, create it first. If typecheck fails, fix module paths only; do not add feature logic.

### M2: CLI scaffold and config example

- Goal: Implement help/version and config validation for example config.
- Files to read: `SPEC-004`, `ENVIRONMENT.md`.
- Files to change: `src/main.rs`, `src/cli/mod.rs`, `src/config/mod.rs`, `src/errors/mod.rs`, `config/example.toml`.
- Exact edits expected: Add CLI parser, `check-config --config <path>`, config struct with paper default, typed config error, user-readable success output.
- Validation command: `./scripts/smoke-test.sh`
- Expected result: `smoke test: ok` and CLI help exits successfully.
- Recovery instruction: If CLI crate API differs, inspect local docs/errors and use smallest working parser implementation; record dependency/API decision.

### M3: Baseline tests

- Goal: Add unit, integration, and E2E smoke coverage.
- Files to read: `TESTING.md`, `SPEC-000`, `SPEC-004`.
- Files to change: `src/config/mod.rs`, `tests/integration_smoke.rs`, `tests/e2e_cli.rs`.
- Exact edits expected: Unit test config parsing; integration test library loads example config; E2E test runs `--help` and `check-config`.
- Validation command: `./scripts/test-unit.sh && ./scripts/test-integration.sh && ./scripts/test-e2e.sh`
- Expected result: `unit tests: ok`, `integration tests: ok`, `e2e tests: ok`.
- Recovery instruction: If test binary names differ, update scripts and `COMMANDS.md` with evidence before rerunning.

### M4: Formatting, linting, and CI

- Goal: Make local and CI validation deterministic.
- Files to read: `COMMANDS.md`, existing CI files.
- Files to change: `.github/workflows/ci.yml`, source files only if lint requires.
- Exact edits expected: Add CI job running scripts in verify order; fix rustfmt/clippy warnings without broad refactors.
- Validation command: `./scripts/lint.sh && ./scripts/format-check.sh`
- Expected result: `lint: ok`, `format check: ok`.
- Recovery instruction: If CI platform already exists, update equivalent pipeline instead of adding GitHub Actions; record decision.

### M5: Full verification and documentation sync

- Goal: Ensure foundation is complete and restartable.
- Files to read: changed files, docs.
- Files to change: `COMMANDS.md` if needed, EP-001 progress/outcomes.
- Exact edits expected: Update progress, decisions, and any command changes; ensure scripts are executable or runnable with `sh`.
- Validation command: `./scripts/verify.sh`
- Expected result: `verify: ok` or documented dependency-audit blocker if audit tool is missing before production.
- Recovery instruction: If dependency audit tool is missing, record as production-readiness blocker but do not skip other validations.

## 9. Concrete Steps

1. Run `./scripts/preflight.sh`.
2. Inspect existing foundation files.
3. Create Cargo scaffold or adapt existing.
4. Add minimal CLI and config behavior.
5. Add tests.
6. Add or update CI.
7. Run milestone validations.
8. Run final verification.
9. Update this ExecPlan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/lint.sh
./scripts/format-check.sh
./scripts/typecheck.sh
./scripts/test-unit.sh
./scripts/test-integration.sh
./scripts/test-e2e.sh
./scripts/build.sh
./scripts/smoke-test.sh
git diff --name-only
```

Acceptance criteria:

- Cargo project builds.
- CLI help/version works.
- Example config validates in paper mode.
- Baseline tests pass.
- CI workflow exists or repository-specific equivalent is documented.
- No secrets or live trading code added.

## 11. Idempotence and Recovery

If foundation files already exist, extend them minimally instead of recreating. If dependency additions fail, remove partial dependency edits or finish the same dependency addition; do not leave broken manifests. If the same validation fails three times, record failed hypotheses and choose simpler scaffold behavior.

## 12. Progress

- [x] M1 - Cargo project and module skeleton. Completed 2026-06-23. Validation: `cargo check --all-targets --all-features --offline` -> passed after adding the Cargo workspace scaffold and generating `Cargo.lock`.
- [x] M2 - CLI scaffold and config example. Completed 2026-06-23. Validation: `cargo run --offline -- --help` and `cargo run --offline -- check-config --config config/example.toml` -> both passed.
- [x] M3 - Baseline tests. Completed 2026-06-23. Validation: `cargo test --lib --bins --all-features --offline`, `cargo test --test integration_smoke --all-features --offline`, and `cargo test --test e2e_cli --all-features --offline` -> passed after a help-text wording fix.
- [x] M4 - Formatting, linting, and CI. Completed 2026-06-23. Validation: `cargo clippy --all-targets --all-features --offline -- -D warnings` and `cargo fmt --all -- --check` -> both passed after a small lint cleanup.
- [x] M5 - Full verification and documentation sync. Completed 2026-06-23. Validation: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features --offline -- -D warnings`, `cargo check --all-targets --all-features --offline`, `cargo test --lib --bins --all-features --offline`, `cargo test --test integration_smoke --all-features --offline`, `cargo test --test e2e_cli --all-features --offline`, `cargo build --release --offline`, `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, and `git diff --name-only` -> passed; `git diff --name-only` was empty after staging because this is the repository's initial commit state.

## 13. Surprises & Discoveries

Record repository differences and validation failures here.

- 2026-06-23: The repository started with docs, scripts, and an empty `.git` directory, not a usable Git repository. `git init` required elevated permission because `.git` was read-only in the workspace policy.
- 2026-06-23: `./scripts/preflight.sh` could not run because the machine has no usable POSIX `sh` runner; `bash.exe` is the WSL launcher and reports no installed distributions.
- 2026-06-23: `cargo check --all-targets --all-features` initially failed because crates.io was unreachable. An offline Cargo fallback succeeded once the dependency set was reduced to crates available in the local cache.
- 2026-06-23: The initial Clap-based CLI dependency tree was not fully cached offline. The foundation was simplified to a standard-library CLI parser to keep the repo buildable in this environment.
- 2026-06-23: The CLI help output is now generated manually and explicitly advertises paper mode and offline-safe validation, matching the CLI-first spec without a parser dependency.
- 2026-06-23: The initial e2e predicate expected the help text to mention `paper mode` in lowercase. The help text was aligned to that wording rather than weakening the test.
- 2026-06-23: Clippy surfaced a `result_large_err` warning on the config error path. Boxing the underlying error payloads resolved the lint without changing CLI behavior.

## 14. Decision Log

Record dependency choices, CI decisions, and command updates here.

- 2026-06-23: Replaced the planned Clap-based CLI dependency with a standard-library parser because the local Cargo cache did not contain the full Clap dependency tree for offline builds.
- 2026-06-23: Replaced `thiserror` with manual `Display`/`Error` implementations for the same offline-cache reason and to keep the error surface minimal.
- 2026-06-23: Added native Cargo validation commands and explicit `--offline` variants to `COMMANDS.md` because the repository scripts are POSIX shell wrappers and the machine cannot execute them.
- 2026-06-23: Kept CI on GitHub Actions but implemented it with direct Cargo commands instead of shell scripts so it mirrors the offline-compatible local workflow.
- 2026-06-23: Used `cargo check --all-targets --all-features --offline` for the M1 validation because network access to crates.io was unavailable.
- 2026-06-23: Used `cargo run --offline -- --help` and `cargo run --offline -- check-config --config config/example.toml` for the M2 validation because the same offline cache needed to cover the binary execution path.
- 2026-06-23: Boxed `toml::de::Error` and `std::io::Error` inside `ConfigError` after Clippy reported a `result_large_err` warning, keeping the public error text stable while satisfying the lint gate.
- 2026-06-23: Used `git diff --cached --name-only --root` for the first repository diff review because this checkout began with an empty `.git` directory and no prior commit history; `git diff --name-only` was also run and returned empty after staging.
- 2026-06-23: Removed `/Cargo.lock` from `.gitignore` so the binary crate's lockfile is tracked and committed with the foundation.

## 15. Outcomes & Retrospective

Complete after M5 with final validation results and remaining risks.

- EP-001 created the initial Cargo-based foundation for OptionClaw and keeps the CLI defaulting to paper mode.
- The repository now builds, formats, lints, and passes unit, integration, e2e, and release-build validation entirely offline.
- The original POSIX shell wrappers remain in the tree for compatibility, but the repo now documents native Cargo/offline fallbacks so validation can run on this Windows machine.
- Remaining risk: the shell wrappers still cannot execute locally without a POSIX `sh` runner, so future agents should prefer the documented Cargo fallback commands in this environment.
