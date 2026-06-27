# OptionClaw Repo Brief

Compact context for Codex, Serena, and Obsidian links. Keep this file current when commands, architecture boundaries, or safety gates change.

## Purpose

OptionClaw is a local-first Rust CLI for options-trading research, paper trading, and tightly gated live execution. It is safety-first: no profit guarantees, no live trading by default, no secrets in repo files, and no broker/wallet/fund movement without explicit validated gates.

## Stack

- Rust 2021, Cargo.
- CLI binary: `optionclaw`.
- Data/config format: TOML config, JSON/JSONL fixtures and local state.
- HTTP client: `reqwest` for Alpaca adapter.
- Tests: Rust unit, integration, contract, and E2E tests.

## Entrypoints

- CLI entrypoint: `src/main.rs`.
- Library/module exports: `src/lib.rs`.
- CLI parsing/output: `src/cli/`.
- Service layer: `src/services/`.
- Domain/risk rules: `src/domain/`, `src/risk/`.
- Alpaca adapter: `src/alpaca/`.
- Persistence/audit/state: `src/persistence/`.
- Config validation: `src/config/`.

## Commands

Use `COMMANDS.md` as command authority. Common local commands:

- Help: `cargo run -- --help`
- Config check: `cargo run -- check-config --config config/example.toml`
- Health: `cargo run -- health --config config/example.toml`
- Format check: `cargo fmt --all -- --check`
- Typecheck: `cargo check --all-targets --all-features --offline`
- Unit/bin tests: `cargo test --lib --bins --all-features --offline`
- Live gate tests: `cargo test --test integration_live --all-features --offline`, `cargo test --test contract_alpaca --all-features --offline`, `cargo test --test e2e_live_cli --all-features --offline`
- Release build: `cargo build --release --offline`

On this Windows host, `C:\Users\domin\.cargo\bin\cargo.exe <documented-cargo-arguments>` is the reliable Cargo form when `cargo` is not on `PATH`.

## Important Directories

- `.agent/`: ExecPlans, specs, checklists, templates.
- `.serena/`: repo-local Serena config and cache.
- `config/`: fake/non-secret example configs.
- `fixtures/`: deterministic fake fixtures.
- `scripts/`: POSIX shell wrappers for documented checks.
- `src/`: Rust implementation.
- `tests/`: integration, contract, and E2E tests.
- `var/`: ignored local runtime state and generated reports.
- `target/`: ignored Cargo build output.

## Data, Auth, And External Services

- Default mode is paper.
- Local state/audit lives under `var/dev` or configured data dirs.
- Secrets must be env-only or approved secret-store values; never commit `.env` or raw credentials.
- Alpaca live readiness uses `OPTIONCLAW_ALPACA_API_KEY`, `OPTIONCLAW_ALPACA_API_SECRET`, and `OPTIONCLAW_ENABLE_LIVE_TRADING=true`.
- Real live submit still requires fresh ROI approval artifact, risk caps, inactive kill switch, provider/account options capability, and `--confirm-live`.
- Tests use mocks/fixtures; do not call real broker endpoints in routine validation.

## Do Not Touch / Risk Zones

- Do not edit generated `target/`, local `var/`, `.serena/cache/`, or tool/IDE folders.
- Do not commit secrets, broker statements, wallet keys, seed phrases, production logs, or real API responses.
- Do not weaken risk gates, redaction, audit, kill-switch checks, or live-submit confirmation.
- Do not implement directly from `ROADMAP.md`; use active ExecPlans under `.agent/execplans/`.

## Current Unknowns / TODOs

- POSIX shell wrappers may exit without expected stdout under PowerShell; native Cargo fallbacks are documented in `COMMANDS.md`.
- Real Alpaca credentials, external broker/KYC/options approval, and live provider reachability have not been verified in repo validation.
- No `README.md`, `CLAUDE.md`, `docs/`, or Obsidian-specific folder exists yet.
