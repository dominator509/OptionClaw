# EP-006 Auth, Security, and Permissions

## 1. Purpose / Big Picture

Implement OptionClaw's local CLI security baseline: encrypted secret handling, redaction, file permission checks, live-mode gates, kill-switch behavior, and security tests. Traditional user authentication is not applicable to initial single-user CLI mode.

## 2. Scope

- Confirm auth is out of scope for local CLI.
- Implement authorization-like gates for modes and dangerous actions.
- Implement redaction helpers.
- Implement encrypted local secrets store or interface with fail-closed behavior.
- Implement kill switch checks.
- Implement live-mode disabled/gated behavior.
- Add security tests.

## 3. Non-goals

- No web auth/session system.
- No multi-user roles.
- No real live broker credentials in tests.
- No wallet private-key custody.
- No real fund movement.

## 4. Context and Orientation

OptionClaw is a local single-user CLI. Security focuses on secrets, redaction, local permissions, and preventing unsafe live behavior. Missing real secrets are not blockers for paper mode.

## 5. Files to Read First

- `SECURITY.md`
- `.agent/specs/SPEC-005-auth-and-permissions.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `ENVIRONMENT.md`
- `src/secrets/mod.rs`
- `src/config/mod.rs`
- `src/execution/mod.rs`
- `src/risk/mod.rs`
- `src/errors/mod.rs`
- Existing tests

## 6. Files to Change

Expected changed files:

- `src/secrets/mod.rs`
- `src/secrets/redaction.rs`
- `src/secrets/store.rs`
- `src/config/mod.rs`
- `src/execution/mod.rs`
- `src/risk/mod.rs`
- `src/errors/mod.rs`
- `tests/integration_security.rs`
- `tests/e2e_cli.rs`
- `SECURITY.md`
- `ENVIRONMENT.md`
- `.agent/execplans/EP-006-auth-security-and-permissions.md`

Forbidden changes:

- Real credentials.
- Live order submission implementation.
- Wallet signing/private-key storage.
- Web auth or session code.

## 7. Interfaces and Contracts

Security contracts:

- `Redacted<T>` or equivalent prevents secret display.
- Secret store encrypts values or fails closed until configured.
- `authorize_execution(mode, gates) -> Result<()>` rejects unsafe modes.
- Kill switch path/state disables execution.
- Live mode requires: enable flag, risk limits, provider configured, secrets present, kill switch checked, production readiness marker or explicit approval mechanism.

If encryption crate APIs are unknown, inspect crate docs locally or use existing repository crypto abstraction. Do not invent crypto APIs.

## 8. Milestones

### M1: Redaction and sensitive config handling

- Goal: Prevent secrets from printing.
- Files to read: config, errors, security spec.
- Files to change: `src/secrets/redaction.rs`, `src/config/mod.rs`, `src/errors/mod.rs`, tests.
- Exact edits expected: Add redaction type/helpers; mark secret config values; tests prove Display/Debug/log-like output redacts.
- Validation command: `./scripts/test-unit.sh`
- Expected result: `unit tests: ok`.
- Recovery instruction: If generic redaction is complex, implement explicit wrapper for strings first and record limitation.

### M2: Local secret store baseline

- Goal: Add encrypted or fail-closed local secret storage behavior.
- Files to read: `SECURITY.md`, `ENVIRONMENT.md`.
- Files to change: `src/secrets/store.rs`, tests, docs.
- Exact edits expected: Implement fake/test secret provider and encrypted store interface; plaintext secret files rejected; no real keys in tests.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok` with no-plaintext tests.
- Recovery instruction: If encryption dependency is uncertain, implement trait plus mock store and STOP only before production/live; record blocker for production encryption.

### M3: Kill switch and live-mode gates

- Goal: Ensure dangerous execution fails closed.
- Files to read: execution/risk/config modules.
- Files to change: `src/execution/mod.rs`, `src/risk/mod.rs`, `src/config/mod.rs`, tests.
- Exact edits expected: Add kill switch check, live-mode gate validation, tests for missing gates and active kill switch.
- Validation command: `./scripts/test-unit.sh && ./scripts/test-integration.sh`
- Expected result: `unit tests: ok`, `integration tests: ok`.
- Recovery instruction: If production readiness marker is unresolved, require explicit config false by default and return `LIVE_TRADING_DISABLED`.

### M4: Security CLI/error behavior

- Goal: Show safe user-facing security errors.
- Files to read: CLI and error specs.
- Files to change: `tests/e2e_cli.rs`, `src/cli/*`, `src/errors/mod.rs`.
- Exact edits expected: E2E test for live disabled and secret redaction in errors.
- Validation command: `./scripts/test-e2e.sh`
- Expected result: `e2e tests: ok`.
- Recovery instruction: If CLI does not expose live flags, test config validation rejecting live mode without gates.

### M5: Security validation

- Goal: Complete security baseline.
- Files to read: changed files and docs.
- Files to change: EP-006 progress/outcomes, `SECURITY.md`, `ENVIRONMENT.md`.
- Exact edits expected: Update docs with actual variables/secret behavior.
- Validation command: `./scripts/security-check.sh && ./scripts/lint.sh && ./scripts/format-check.sh && ./scripts/typecheck.sh`
- Expected result: commands print ok.
- Recovery instruction: If dependency audit/security tooling missing, record production-readiness blocker and continue with local redaction tests.

## 9. Concrete Steps

1. Run preflight.
2. Confirm local CLI auth model.
3. Implement redaction.
4. Implement secret store interface/baseline.
5. Implement kill switch and live gates.
6. Add security tests.
7. Update docs and validate.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/test-unit.sh
./scripts/test-integration.sh
./scripts/test-e2e.sh
./scripts/security-check.sh
git diff --name-only
```

Acceptance criteria:

- Auth out-of-scope is explicit for local CLI.
- Redaction tests pass.
- Paper mode does not require secrets.
- Live mode fails closed without all gates.
- Kill switch blocks execution.
- No plaintext secrets in expected secret storage tests.

## 11. Idempotence and Recovery

If security helpers exist, extend rather than replace. If encryption cannot be safely implemented, keep live mode disabled and record STOP for production/live enablement.

## 12. Progress

- [ ] M1 - Redaction and sensitive config handling.
- [ ] M2 - Local secret store baseline.
- [ ] M3 - Kill switch and live-mode gates.
- [ ] M4 - Security CLI/error behavior.
- [ ] M5 - Security validation.

## 13. Surprises & Discoveries

Record repository differences and validation failures here.

## 14. Decision Log

Record redaction, encryption, and live-gate decisions here.

## 15. Outcomes & Retrospective

Complete after M5.
