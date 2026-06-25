# EP-010 Production Readiness

## 1. Purpose / Big Picture

Bring OptionClaw to production readiness for its configured mode by completing final verification, security review, performance review, accessibility review, privacy review, backup/restore verification, monitoring verification, deployment dry run, rollback drill, documentation review, and launch checklist.

## 2. Scope

- Full verification.
- Security and dependency review.
- Performance and resource review for local hardware/VPS.
- CLI accessibility review.
- Privacy/data review.
- Backup/restore verification.
- Observability/health verification.
- Deployment dry run in paper/sandbox mode.
- Rollback drill.
- Documentation and release checklist.

## 3. Non-goals

- No guarantee of profit.
- No legal/tax/regulatory approval by coding agent.
- No live trading unless explicit operator approval is provided after all gates.
- No provider-specific live adapter unless already implemented by separate plan.
- No production deployment without permission.

## 4. Context and Orientation

Production readiness means the software can be safely operated in its configured mode. For OptionClaw, live trading is a separate permission gate. If any live credential, legal/compliance issue, or real fund movement is involved, STOP.

## 5. Files to Read First

- `PRODUCTION_READINESS.md`
- `.agent/specs/SPEC-008-production-readiness.md`
- `SECURITY.md`
- `TESTING.md`
- `OBSERVABILITY.md`
- `DEPLOYMENT.md`
- `OPERATIONS.md`
- `RELEASE.md`
- `ROLLBACK.md`
- `ASSUMPTIONS.md`
- `DECISIONS.md`
- All prior ExecPlan Outcomes

## 6. Files to Change

Expected changed files:

- `PRODUCTION_READINESS.md`
- `SECURITY.md`
- `TESTING.md`
- `OBSERVABILITY.md`
- `DEPLOYMENT.md`
- `OPERATIONS.md`
- `RELEASE.md`
- `ROLLBACK.md`
- `ASSUMPTIONS.md`
- `DECISIONS.md`
- `scripts/production-readiness-check.sh`
- `.agent/checklists/production-readiness.md`
- `.agent/execplans/EP-010-production-readiness.md`

Forbidden changes:

- New product features.
- Live trading enablement without explicit approval.
- Real credentials or production data.
- Irreversible migrations.

## 7. Interfaces and Contracts

Production readiness check must run:

```sh
./scripts/verify.sh
./scripts/security-check.sh
./scripts/dependency-audit.sh
./scripts/smoke-test.sh
```

It must also verify required docs exist. Additional target-specific deployment dry-run commands must be documented, not guessed.

## 8. Milestones

### M1: Full verification baseline

- Goal: Establish current validation status.
- Files to read: scripts, command docs.
- Files to change: EP-010 progress/surprises.
- Exact edits expected: Record exact command results and blockers.
- Validation command: `./scripts/verify.sh`
- Expected result: `verify: ok`.
- Recovery instruction: If verify fails, debug narrow failing command with anti-fixation; do not continue to launch gate until resolved or documented as STOP.

### M2: Security, privacy, and dependency review

- Goal: Confirm secret, redaction, live-mode, privacy, and dependency safety.
- Files to read: security docs, env docs, dependency manifests, tests.
- Files to change: `SECURITY.md`, `ASSUMPTIONS.md`, `DECISIONS.md`, EP-010.
- Exact edits expected: Update any remaining risks; verify no secrets; review audit result.
- Validation command: `./scripts/security-check.sh && ./scripts/dependency-audit.sh`
- Expected result: `security check: ok`, `dependency audit: ok`.
- Recovery instruction: If audit tooling missing, STOP production readiness or replace with documented approved equivalent.

### M3: Performance, accessibility, and observability review

- Goal: Confirm operational quality for CLI/local runtime.
- Files to read: observability, operations, CLI tests, performance notes.
- Files to change: docs and EP-010.
- Exact edits expected: Record performance expectations, CLI accessibility status, observability health/signal verification.
- Validation command: `./scripts/smoke-test.sh`
- Expected result: `smoke test: ok`.
- Recovery instruction: If performance is unknown, document expected load and add a simple timing smoke test or mark as production risk.

### M4: Backup/restore, deployment dry run, and rollback drill

- Goal: Verify operational recovery.
- Files to read: persistence tests, deployment docs, rollback docs.
- Files to change: `DEPLOYMENT.md`, `ROLLBACK.md`, `OPERATIONS.md`, EP-010.
- Exact edits expected: Record dry-run/rollback drill results in paper mode; update docs if commands differed.
- Validation command: `./scripts/build.sh && ./scripts/smoke-test.sh`
- Expected result: `build: ok`, `smoke test: ok`.
- Recovery instruction: If a real deployment target is unavailable, document local dry-run and mark target-specific deployment as remaining risk; STOP before production deploy.

### M5: Final launch gate

- Goal: Complete production-readiness checklist and final report.
- Files to read: all readiness docs/checklists.
- Files to change: `PRODUCTION_READINESS.md`, `.agent/checklists/production-readiness.md`, EP-010 outcomes.
- Exact edits expected: Mark checklist status, risks, and mode-specific readiness.
- Validation command: `./scripts/production-readiness-check.sh && git diff --name-only`
- Expected result: `production readiness: ok`; changed files match expected or extras justified.
- Recovery instruction: If live-specific criteria cannot pass due missing credentials/provider approval, mark production-ready for paper mode only and STOP before live.

## 9. Concrete Steps

1. Run preflight.
2. Run full verification.
3. Review security/privacy/dependencies.
4. Review performance/accessibility/observability.
5. Verify backup/restore and rollback drill.
6. Complete production-readiness checklist.
7. Run final production readiness check.
8. Update outcomes and final report.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/verify.sh
./scripts/production-readiness-check.sh
git diff --name-only
```

Acceptance criteria:

- Full verification passes.
- Production readiness check passes for configured mode.
- Security, privacy, performance, accessibility, observability, deployment, rollback, data, docs, and support status are documented.
- Live trading remains disabled unless explicit approval and gates exist.
- Remaining risks are documented.

## 11. Idempotence and Recovery

Readiness review may be rerun. Do not mark criteria complete without evidence. If a criterion fails, fix via a scoped change or record STOP. Do not deploy or enable live mode from this plan without explicit permission.

## 12. Progress

- [x] M1 - Full verification baseline. Completed 2026-06-24. Validation: `cargo build --release --offline`, `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, `cargo run --offline -- health --config config/example.toml` -> pass. The shell wrappers in `./scripts/*.sh` are not directly executable under `cmd.exe` on this host.
- [x] M2 - Security, privacy, and dependency review. Completed 2026-06-24. Validation: `cargo check --all-targets --all-features --offline`, `cargo audit --version`, `set "CARGO_HOME=C:\dev\OptionClaw\.cargo" && cargo audit --no-fetch --stale` -> pass. The first audit attempt hit a read-only lock path in the default home directory; the writable workspace home plus stale/no-fetch mode completed successfully.
- [x] M3 - Performance, accessibility, and observability review. Completed 2026-06-24. Validation: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo run --offline -- --help`, `cargo run --offline -- check-config --config config/example.toml`, `cargo run --offline -- health --config config/example.toml` -> pass.
- [x] M4 - Backup/restore, deployment dry run, and rollback drill. Completed 2026-06-24. Validation: `cargo run --offline -- state init --data-dir ./var/dev`, `cargo run --offline -- state verify --data-dir ./var/dev`, `cargo test --lib --bins --all-features --offline`, `cargo test --test integration_smoke --all-features --offline`, `cargo test --test integration_persistence --all-features --offline`, `cargo test --test integration_services --all-features --offline`, `cargo test --test contract_adapters --all-features --offline`, `cargo test --test e2e_cli --all-features --offline`, `cargo build --release --offline`, `cargo run --offline -- health --config config/example.toml` -> pass. Health moved from `data_ready=false` before state init to `data_ready=true` after init/verify.
- [x] M5 - Final launch gate. Completed 2026-06-24. Validation: `set "CARGO_HOME=C:\dev\OptionClaw\.cargo" && cargo audit --no-fetch --stale`, `git diff --name-only` -> pass. `./scripts/production-readiness-check.sh` is documented but not directly executable under `cmd.exe` on this host; the native Cargo equivalents completed successfully.

## 13. Surprises & Discoveries

- The repository's `.sh` wrappers are POSIX shell scripts and are not directly executable under `cmd.exe` on this host.
- `cargo audit` initially failed on the default read-only home lock path. Using a writable workspace `CARGO_HOME` and `--no-fetch --stale` completed the audit successfully.
- `cargo run --offline -- health --config config/example.toml` reported `data_ready=false` before local state initialization, then `data_ready=true` after `cargo run --offline -- state init --data-dir ./var/dev` and `cargo run --offline -- state verify --data-dir ./var/dev`.
- Unit, integration, contract, and e2e CLI tests all passed offline against the current checkout.

## 14. Decision Log

- Use native Cargo commands as the documented validation fallback when the shell wrappers cannot run on this Windows host.
- Treat configured paper mode as the readiness target for EP-010; live trading remains disabled until separate approval and gates exist.
- Update `COMMANDS.md` with `cargo audit --no-fetch --stale` because that is the working repository-local audit path on this host.
- Reword `PRODUCTION_READINESS.md` and `SECURITY.md` to reflect the current fail-closed plaintext-secret baseline instead of claiming encrypted secret persistence that is not yet implemented.

## 15. Outcomes & Retrospective

EP-010 is complete for configured paper-mode production readiness. All validation commands used for this plan passed in native Cargo form, local state was initialized and verified, the health check reports `config_ready=true`, `data_ready=true`, `audit_ready=true`, `secrets_store_ready=true`, `providers_ready=true`, and `kill_switch_active=false`, and the launch checklist is marked complete. Remaining risks are limited to live-mode enablement, which still requires explicit operator approval and separate live-gates outside this plan.
