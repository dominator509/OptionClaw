# COMMANDS.md

Coding agents must not invent commands. If a command is missing, update this file first with evidence from the repository.

## Working Directory Rule

Run all commands from the repository root. Scripts enforce repository-root execution by resolving their own location.

## Package Manager Rule

Default package manager/build tool: Cargo for Rust.

- Verify: `cargo --version`
- Project manifest: `Cargo.toml`
- Lockfile: `Cargo.lock`

If repository discovery finds a different build system, update this file and the scripts before running any non-documented command.

## Allowed Commands

| Purpose | Command | Expected Success Output |
|---|---|---|
| Preflight | `./scripts/preflight.sh` | `preflight: ok` |
| Install dependencies | `./scripts/install.sh` | `install: ok` |
| Lint | `./scripts/lint.sh` | `lint: ok` |
| Format check | `./scripts/format-check.sh` | `format check: ok` |
| Typecheck/static validation | `./scripts/typecheck.sh` | `typecheck: ok` |
| Unit tests | `./scripts/test-unit.sh` | `unit tests: ok` |
| Integration tests | `./scripts/test-integration.sh` | `integration tests: ok` |
| E2E/acceptance tests | `./scripts/test-e2e.sh` | `e2e tests: ok` |
| Build | `./scripts/build.sh` | `build: ok` |
| Security check | `./scripts/security-check.sh` | `security check: ok` |
| Dependency audit | `./scripts/dependency-audit.sh` | `dependency audit: ok` |
| Smoke test | `./scripts/smoke-test.sh` | `smoke test: ok` |
| Full verification | `./scripts/verify.sh` | `verify: ok` |
| Production readiness check | `./scripts/production-readiness-check.sh` | `production readiness: ok` |
| Local development start | `cargo run -- --help` after EP-001 | CLI help text exits successfully |
| Repository status | `git status --short` | Lists changed files or no output |
| Diff names | `git diff --name-only` | Lists changed file paths |

## Dependency Update Commands

Use only when the active ExecPlan explicitly requires dependency changes:

```sh
cargo add <crate-name>
cargo add <crate-name> --features <feature-a>,<feature-b>
cargo update -p <crate-name>
```

Replace placeholders only with crate names and features stated in the active ExecPlan or verified from repository evidence. After dependency changes, run:

```sh
./scripts/install.sh
./scripts/typecheck.sh
./scripts/test-unit.sh
./scripts/dependency-audit.sh
```

## Local Database Setup

Not applicable in the initial architecture. OptionClaw uses no external database. EP-003 defines local file persistence with schema versions, backups, and recovery. If a database is later added, create a new ExecPlan and update this section before implementation.

## Migration Command

No database migration command exists initially. Local file schema migration must be implemented as an explicit CLI command during EP-003, expected name:

```sh
cargo run -- migrate-local-state --dry-run --data-dir ./var/dev
```

This command is a planned contract, not available before EP-003. Scripts must fail clearly until the implementation exists.

## Local Development Environment

After EP-001 completes:

```sh
cargo run -- --help
cargo run -- check-config --config config/example.toml
```

## Native Cargo Validation Fallback

Use these commands when the repository scripts cannot run because a POSIX `sh` runner is unavailable on the local machine:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo check --all-targets --all-features
cargo test --lib --bins --all-features
cargo test --test integration_smoke --all-features
cargo test --test e2e_cli --all-features
cargo build --release
cargo run -- --help
cargo run -- check-config --config config/example.toml
```

These commands are the native equivalents of the repository scripts and may be used only when the script wrappers are not executable in the current environment.

If the machine cannot reach crates.io, add `--offline` to the Cargo commands above. Use the offline form only when the required crates are already present in the local Cargo cache.

```sh
cargo check --all-targets --all-features --offline
cargo test --lib --bins --all-features --offline
cargo test --test integration_smoke --all-features --offline
cargo test --test integration_persistence --all-features --offline
cargo test --test integration_services --all-features --offline
cargo test --test contract_adapters --all-features --offline
cargo test --test e2e_cli --all-features --offline
cargo build --release --offline
cargo run --offline -- --help
cargo run --offline -- check-config --config config/example.toml
cargo audit --no-fetch --stale
```

After EP-003 completes:

```sh
cargo run -- state init --data-dir ./var/dev
cargo run -- state verify --data-dir ./var/dev
```

After EP-004 completes:

```sh
cargo run -- paper run-once --config config/example.toml --fixtures fixtures/market/sample_snapshot.json
```

After EP-008 completes:

```sh
cargo run -- health --config config/example.toml
```

## Repository Bootstrap and Publish

Use these commands when initializing a brand-new Git repository or publishing the local checkout to GitHub:

```sh
git init
git branch -M main
git add .
git commit -m "<message>"
gh auth status
gh repo create dominator509/OptionClaw --source . --private --remote origin --push
git push -u origin main
```

Notes:

- Replace `--private` with `--public` only if the user explicitly requests a public repository.
- Use `main` as the initial branch unless the user requests a different branch name.
- If `gh` is unavailable or unauthenticated, stop and report the missing GitHub CLI/authentication blocker.
- On a brand-new repository with no commits yet, use `git add .` followed by `git diff --cached --name-only --root` for the first diff review.

## Forbidden Commands

Do not run these unless explicit user permission and an applicable ExecPlan allow them:

- `rm -rf` against repository root, home directory, data directories, wallet directories, or production paths.
- Commands that submit live orders, transfer funds, sign wallet transactions, or withdraw assets.
- Commands that export or print secrets.
- Production deployment commands.
- Irreversible migrations.
- `cargo publish`.
- Any command not documented here unless this file is first updated with repository evidence.

## Recovery Instructions

If a command fails:

1. Copy the exact command and error into the active ExecPlan `Surprises & Discoveries`.
2. Apply the anti-fixation rule from `AGENTS.md`.
3. Prefer the narrowest command that reproduces the failure.
4. Do not delete tests or loosen validation without recording a decision and preserving coverage.
5. If the script itself is stale, update the script and this file in the same change, then rerun `./scripts/preflight.sh`.
