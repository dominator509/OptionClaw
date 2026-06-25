# EP-009 Deployment and Release

## 1. Purpose / Big Picture

Prepare OptionClaw for repeatable release and deployment on local hardware, Raspberry Pi, or VPS using a release binary, documented config, staging/paper verification, smoke tests, and rollback path.

## 2. Scope

- Release artifact definition.
- Deployment config examples.
- Local/VPS deployment docs.
- Optional systemd service or container wrapper if repository evidence supports it.
- CI/CD release checks.
- Release checklist and rollback path.
- Staging verification in paper/sandbox mode.

## 3. Non-goals

- No production live trading approval.
- No cloud SaaS deployment.
- No Kubernetes unless explicitly required by repository evidence.
- No irreversible migrations.
- No real credentials in deployment examples.

## 4. Context and Orientation

Deployment target is operator-owned hardware. Start with paper mode. Live mode remains gated. Deployment commands in docs must be exact but not run against production without explicit permission.

## 5. Files to Read First

- `DEPLOYMENT.md`
- `RELEASE.md`
- `ROLLBACK.md`
- `PRODUCTION_READINESS.md`
- `COMMANDS.md`
- `.github/workflows/ci.yml`
- `scripts/build.sh`
- `scripts/smoke-test.sh`
- Existing deployment files if any

## 6. Files to Change

Expected changed files:

- `DEPLOYMENT.md`
- `RELEASE.md`
- `ROLLBACK.md`
- `PRODUCTION_READINESS.md`
- `config/production.example.toml`
- `.github/workflows/ci.yml`
- `scripts/build.sh`
- `scripts/smoke-test.sh`
- `scripts/production-readiness-check.sh`
- `deploy/systemd/optionclaw.service.example` if systemd is chosen
- `deploy/README.md`
- `.agent/execplans/EP-009-deployment-and-release.md`

Forbidden changes:

- Real production config or secrets.
- Production deployment execution.
- Live provider enablement.
- Irreversible migration.

## 7. Interfaces and Contracts

Deployment contracts:

- Build artifact: `target/release/optionclaw`.
- Config validation: `optionclaw check-config --config <path>`.
- Health verification: `optionclaw health --config <path>`.
- Smoke test: no live orders.
- Release checklist must include rollback artifact.

## 8. Milestones

### M1: Release artifact and config examples

- Goal: Define deployable artifact and production example config.
- Files to read: config module, build script, deployment docs.
- Files to change: `config/production.example.toml`, `DEPLOYMENT.md`, `RELEASE.md`.
- Exact edits expected: Add paper-mode production example with fake placeholders, documented paths, no secrets.
- Validation command: `./scripts/build.sh`
- Expected result: `build: ok`.
- Recovery instruction: If release build fails due lockfile, run install/typecheck and fix manifest rather than disabling `--locked` without decision.

### M2: Deployment wrapper and docs

- Goal: Document target deployment for local/VPS hardware.
- Files to read: existing deployment files.
- Files to change: `deploy/README.md`, optional `deploy/systemd/optionclaw.service.example`, `DEPLOYMENT.md`.
- Exact edits expected: Provide exact install/check/smoke commands with placeholders clearly marked; no production execution.
- Validation command: `git diff --name-only`
- Expected result: Deployment docs/files listed as expected.
- Recovery instruction: If systemd is not appropriate, document manual binary execution and record decision.

### M3: CI/CD release checks

- Goal: Ensure CI validates release readiness without deploying.
- Files to read: CI workflow and scripts.
- Files to change: `.github/workflows/ci.yml`, scripts if needed.
- Exact edits expected: CI runs verify; optional release job builds artifact on tags without secrets.
- Validation command: `./scripts/verify.sh`
- Expected result: `verify: ok`.
- Recovery instruction: If dependency audit missing locally, document as production blocker but keep CI command clear.

### M4: Rollback path and smoke tests

- Goal: Make rollback and smoke verification concrete.
- Files to read: rollback and smoke docs.
- Files to change: `ROLLBACK.md`, `scripts/smoke-test.sh`, `DEPLOYMENT.md`.
- Exact edits expected: Smoke validates help/config/health; rollback steps include binary/config/data verification.
- Validation command: `./scripts/smoke-test.sh`
- Expected result: `smoke test: ok`.
- Recovery instruction: If health command unavailable, do not fake pass; make smoke fail clearly or complete EP-008 first.

### M5: Release final validation

- Goal: Complete release readiness for paper/staging deployment.
- Files to read: changed files.
- Files to change: EP-009 progress/outcomes.
- Exact edits expected: Update plan with artifact path, deployment mode, rollback path, and remaining risks.
- Validation command: `./scripts/build.sh && ./scripts/smoke-test.sh && git diff --name-only`
- Expected result: build and smoke ok; changed files match expected.
- Recovery instruction: If scripts are stale, update `COMMANDS.md` and scripts with evidence.

## 9. Concrete Steps

1. Run preflight.
2. Confirm build artifact path.
3. Add production example config.
4. Add deployment docs/wrapper.
5. Align CI.
6. Update rollback and smoke tests.
7. Validate and update plan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/build.sh
./scripts/smoke-test.sh
git diff --name-only
```

Acceptance criteria:

- Release artifact builds.
- Deployment docs are exact and safe.
- Production example config has no secrets and defaults to paper.
- Smoke tests pass without live orders.
- Rollback path is documented.

## 11. Idempotence and Recovery

Do not overwrite real deployment configs. Examples must use `.example` suffix. Re-running this plan must update docs, not deploy. If target environment is unknown, document local/VPS defaults and STOP before production deployment.

## 12. Progress

- [x] M1 - Release artifact and config examples. Completed 2026-06-24. Validation: `cargo build --release --offline` -> passed. Direct `./scripts/build.sh` invocation failed under `cmd.exe` with `'.' is not recognized as an internal or external command, operable program or batch file.'`.
- [x] M2 - Deployment wrapper and docs. Completed 2026-06-24. Validation: `git diff --name-only` -> passed.
- [x] M3 - CI/CD release checks. Completed 2026-06-24. Validation: `cargo build --release --offline`, `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, `cargo run --offline -- health --config config/example.toml` -> passed. Direct `./scripts/verify.sh` invocation failed under `cmd.exe` with `'.' is not recognized as an internal or external command, operable program or batch file.'`.
- [x] M4 - Rollback path and smoke tests. Completed 2026-06-24. Validation: `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, `cargo run --offline -- health --config config/example.toml` -> passed. Direct `./scripts/smoke-test.sh` invocation failed under `cmd.exe` with `'.' is not recognized as an internal or external command, operable program or batch file.'`.
- [x] M5 - Release final validation. Completed 2026-06-24. Validation: `cargo build --release --offline`, `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, `cargo run --offline -- health --config config/example.toml`, `git diff --name-only` -> passed. The shell wrapper commands remained blocked by `cmd.exe`, but the documented native Cargo fallbacks completed successfully.

## 13. Surprises & Discoveries

Record deployment target discoveries and validation failures here.

- The repository did not contain a deploy wrapper, so EP-009 uses a manual release layout instead of a systemd example.
- The deployed config path must live under `/opt/optionclaw/config/` so the current data-dir derivation continues to place state under `/opt/optionclaw/var/dev`.
- `./scripts/build.sh` still fails under `cmd.exe` on this host because the shell wrapper is not directly executable here; the Cargo fallback build passed.
- `./scripts/verify.sh` and `./scripts/smoke-test.sh` remain blocked by the same `cmd.exe` wrapper issue, so the release checks rely on the documented native Cargo commands instead.
- `COMMANDS.md` was updated to document `cargo run -- health --config config/example.toml` because the smoke path now exercises health.

## 14. Decision Log

Record deployment wrapper and release decisions here.

- Chose not to add a systemd unit because the repository did not provide evidence for one and the plan made it optional.
- Chose to document a manual operator-owned `/opt/optionclaw` release layout so the existing `derive_data_dir` behavior remains valid.
- Chose to keep the production example config paper-only and fake-value-only to avoid implying live-readiness.
- Chose to extend the smoke test to health output so release verification catches readiness regressions without live orders.
- Chose to keep CI on `./scripts/verify.sh` rather than add a separate release job, because the verify script already composes build, tests, security, and smoke checks.

## 15. Outcomes & Retrospective

EP-009 is complete. The release path is documented around an operator-owned `/opt/optionclaw` layout, with paper-only production example config, rollback instructions, smoke coverage including health, and production-readiness checks that match the repo's current command set. The only environment-specific limitation encountered was the Windows `cmd.exe` inability to launch the `.sh` wrappers directly; the documented native Cargo fallback commands passed in full.
