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

- [ ] M1 - Release artifact and config examples.
- [ ] M2 - Deployment wrapper and docs.
- [ ] M3 - CI/CD release checks.
- [ ] M4 - Rollback path and smoke tests.
- [ ] M5 - Release final validation.

## 13. Surprises & Discoveries

Record deployment target discoveries and validation failures here.

## 14. Decision Log

Record deployment wrapper and release decisions here.

## 15. Outcomes & Retrospective

Complete after M5.
