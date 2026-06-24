# OptionClaw Environment Setup

## Required Tools

| Tool | Version Rule | Required | Purpose | Verify Command |
|---|---|---|---|---|
| Rust toolchain | Stable Rust, version recorded during EP-000 | Yes | Build and test | `rustc --version` |
| Cargo | Bundled with Rust | Yes | Package manager | `cargo --version` |
| Git | Any supported version | Yes | Diff and repository state | `git --version` |
| POSIX shell | `/usr/bin/env sh` compatible | Yes | Scripts | `sh --version` or shell-specific equivalent |
| cargo-audit | Current available version | Required before production readiness | Dependency audit | `cargo audit --version` |

EP-000 must record exact versions found locally in the active ExecPlan.

## Package Manager

Cargo is the default package manager. Do not introduce another package manager without updating `COMMANDS.md`, scripts, and `DECISIONS.md`.

## Environment Variables

These variables are planned contracts. Implement variables only when the linked ExecPlan reaches that feature. Unknown provider-specific variables require confirmation before implementation.

| Name | Required/Optional | Environment | Example Value | Secret | Description | Validation Rule |
|---|---|---|---|---|---|---|
| `OPTIONCLAW_CONFIG` | Optional | local/staging/production | `config/example.toml` | No | Default config path when CLI flag is omitted. | If set, file must exist and parse. |
| `OPTIONCLAW_DATA_DIR` | Optional | local/staging/production | `./var/dev` | No | Local state, audit, and paper-trading data directory. | Must be writable; production path must not be inside repository unless explicitly approved. |
| `OPTIONCLAW_TRADING_MODE` | Optional | all | `paper` | No | Trading mode. | Allowed values: `paper`, `sandbox`, `live`; default `paper`; `live` requires EP-006 and EP-010 gates. |
| `OPTIONCLAW_LOG_LEVEL` | Optional | all | `info` | No | Structured log level. | Allowed values: `trace`, `debug`, `info`, `warn`, `error`. |
| `OPTIONCLAW_LLM_PROVIDER` | Optional | all | `mock` | No | LLM adapter selection. | Allowed initial value: `mock`; other values require provider adapter implementation. |
| `OPTIONCLAW_LLM_ENDPOINT` | Optional | local/staging/production | `http://127.0.0.1:11434/v1` | No | OpenAI-compatible or local model endpoint. | Must be a valid URL if provider is not `mock`. |
| `OPTIONCLAW_LLM_API_KEY` | Optional until real provider | local/staging/production | `opcl_fake_key_for_tests_only` | Yes | LLM provider API key. | Must not be logged; required only for providers that need it. |
| `OPTIONCLAW_MARKET_DATA_PROVIDER` | Optional | all | `fixture` | No | Market data adapter selection. | Initial allowed value: `fixture`; real providers require adapter ExecPlan. |
| `OPTIONCLAW_MARKET_DATA_API_KEY` | Optional until real provider | all | `opcl_fake_market_key` | Yes | Market data provider API key. | Must not be logged; required only for providers that need it. |
| `OPTIONCLAW_BROKER_PROVIDER` | Optional until broker adapter | sandbox/production | `mock` | No | Broker/exchange execution provider. | Initial allowed values: `mock`, `paper`; real providers require contract tests. |
| `OPTIONCLAW_BROKER_API_KEY` | Optional until broker adapter | sandbox/production | `opcl_fake_broker_key` | Yes | Broker/exchange API key. | Required only for sandbox/live provider; must not be logged. |
| `OPTIONCLAW_BROKER_API_SECRET` | Optional until broker adapter | sandbox/production | `opcl_fake_broker_secret` | Yes | Broker/exchange API secret. | Required only for sandbox/live provider; must not be logged. |
| `OPTIONCLAW_WALLET_PROVIDER` | Optional | sandbox/production | `disabled` | No | Wallet connector selection. | Default `disabled`; real signing requires dedicated security plan. |
| `OPTIONCLAW_KILL_SWITCH_FILE` | Optional | all | `./var/dev/KILL_SWITCH` | No | Path checked before execution. Presence or configured state disables execution. | Must be readable if set. |
| `OPTIONCLAW_MAX_ACCOUNT_RISK_PCT` | Required before live | sandbox/production | `1.0` | No | Max account equity at risk per order intent. | Decimal greater than 0 and less than or equal to configured cap. |
| `OPTIONCLAW_MAX_DAILY_LOSS_PCT` | Required before live | sandbox/production | `3.0` | No | Daily loss kill threshold. | Decimal greater than 0; live mode requires value. |
| `OPTIONCLAW_ENABLE_LIVE_TRADING` | Required before live | production | `false` | No | Explicit live-trading enable flag. | Must be exactly `true` plus all live gates; default false. |

## Secrets

- Use fake values only in examples and tests.
- Do not commit `.env` files.
- Local encrypted secret storage is implemented in EP-006.
- Missing real credentials are STOP conditions only for tasks that cannot use mocks or fixtures.

## Local Development Setup

1. Install Rust and Cargo.
2. Place the blueprint pack into the repository.
3. Run `./scripts/preflight.sh`.
4. Execute EP-000, then EP-001.
5. After EP-001, run `./scripts/verify.sh`.

## Local Database Setup

Not applicable. OptionClaw initially uses local files only.

## Test Environment Setup

- Use fixture configs under `fixtures/config/` or `config/example.toml`.
- Use temporary directories for state.
- Use mock providers for LLM, market data, broker/exchange, and wallet.
- Do not require network or secrets for default tests.

## Staging Environment Setup

Staging is a local or VPS deployment using paper or sandbox mode only. It requires:

- Release binary.
- Config file with `OPTIONCLAW_TRADING_MODE=paper` or `sandbox`.
- Data directory outside source checkout.
- Logs directed to the operator's chosen supervisor.
- No live credentials unless sandbox provider requires test credentials.

## Production Environment Setup

Production live trading requires:

- EP-010 production readiness passed.
- Explicit operator approval outside the repository.
- Validated provider integration.
- Configured risk limits.
- Kill switch tested.
- Encrypted secrets.
- Backup and rollback process.

## Configuration Validation

`cargo run -- check-config --config <path>` is the planned validation command after EP-001/EP-004. Before the command exists, scripts must fail clearly rather than silently pass.

## Environment Parity Rules

- Use the same binary artifact across staging and production when possible.
- Use different config and secrets, not different code.
- Tests must run without production credentials.
- Live mode must not be enabled in development by default.

## Troubleshooting

| Symptom | Check | Recovery |
|---|---|---|
| `cargo` not found | `cargo --version` | Install Rust toolchain; rerun preflight. |
| Script says `Cargo.toml` missing | `ls Cargo.toml` | Complete EP-001 foundation. |
| Missing secret error | Confirm mode and provider | Use mock/fixture provider or stop if live credentials are required. |
| Config parse failure | Run check-config | Fix config key from `ENVIRONMENT.md`; do not invent keys. |
| Dependency audit command missing | `cargo audit --version` | Install `cargo-audit` or update commands with approved equivalent. |
