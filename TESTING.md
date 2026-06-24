# OptionClaw Testing Strategy

## Test Pyramid

1. Unit tests: highest volume, fastest, no network, no secrets, no filesystem except temporary directories when necessary.
2. Integration tests: persistence, adapters, config, service contracts, and mock provider interactions.
3. E2E/acceptance tests: CLI commands and user-visible behavior.
4. Smoke tests: build artifact runs, config validates, health command works.
5. Security and dependency tests: redaction, secret handling, audit, and dependency audit.

## Unit Test Rules

- Domain tests must be deterministic and pure.
- Risk tests must cover accept, reject, boundary, and missing-config cases.
- LLM parsing tests must cover malformed, overconfident, missing-field, and rejected model output.
- Unit tests must not require broker credentials, wallet keys, model API keys, or network access.

## Integration Test Rules

- Use fixtures under `fixtures/` and temporary directories.
- Test local file persistence, schema versions, atomic writes, audit log append, backup, restore, and corruption handling.
- Test adapter traits against mock implementations before provider-specific implementations.
- Never call live broker, exchange, wallet, model, or news APIs in standard integration tests.

## E2E Test Rules

- E2E tests exercise the compiled CLI or `cargo run` equivalent.
- Required flows: `--help`, `check-config`, `state init`, `state verify`, `paper run-once`, `risk explain`, and `health` as those commands become available.
- CLI errors must produce nonzero exit codes and readable messages.
- Output must not rely on color only.

## Contract Test Rules

Contract tests are required for:

- Market data provider traits.
- LLM provider traits.
- Execution provider traits.
- Persistence repository traits.
- CLI command input/output contracts.

Provider-specific contract tests must run against fixtures or sandbox environments only unless explicit live permission exists.

## Smoke Test Rules

Smoke tests must verify:

- Binary starts.
- Help command exits successfully.
- Example config validates.
- Health command reports local readiness after EP-008.
- No secrets are printed.

## Regression Test Rules

Every fixed bug must add a regression test unless impossible. If impossible, document why in the active ExecPlan and add a narrower diagnostic test.

## Performance Test Rules

- Domain and risk checks should be fast enough to run per order intent.
- EP-007 must add benchmark or timing smoke checks for critical paths if bottlenecks are identified.
- Performance tests must use fixtures and must not call live APIs.

## Accessibility Test Rules

For CLI accessibility:

- Required information must be visible without color.
- Errors must include action-oriented messages.
- Commands must support `--help`.
- Output should be parseable for operators and logs.

## Security Test Rules

- Redaction tests must prove secrets are not displayed in logs/errors.
- Config tests must fail closed when live mode lacks required risk settings.
- Secret storage tests must verify plaintext is not written to expected encrypted files.
- Live execution tests must verify disabled-by-default behavior.

## Test Data Rules

- Test data must be fake, synthetic, or public fixture data with no private account identifiers.
- Fixtures must not include real API keys, wallet addresses tied to the user, broker account IDs, seed phrases, or private market data exports.
- Fixture timestamps should be deterministic.

## Mocking Rules

- Mock external providers at adapter traits.
- Do not mock domain logic to make higher-level tests pass.
- Paper-trading simulator is a first-class test substitute for live execution.

## Fixture Rules

- Store stable fixtures in `fixtures/`.
- Keep fixture schemas documented in specs.
- Update tests and schema versions together.

## Required Tests Per Feature

| Feature Type | Required Tests |
|---|---|
| Domain entity | Unit tests for valid construction and invalid inputs. |
| Risk rule | Unit tests for accept, reject, boundary, missing config, and explanation. |
| Persistence change | Integration tests for write/read, schema version, backup/restore, and corrupt file. |
| CLI command | E2E tests for help, success, invalid input, and output contract. |
| Adapter | Contract tests with fixture/mock provider and error mapping. |
| Secret behavior | Unit/integration tests for redaction, permissions, and no plaintext storage. |
| Observability | Tests or smoke checks for structured fields and secret redaction. |

## Validation Matrix

| Command | Required Before Completion | Notes |
|---|---|---|
| `./scripts/lint.sh` | Yes after EP-001 | Clippy with warnings denied. |
| `./scripts/format-check.sh` | Yes after EP-001 | Rustfmt check. |
| `./scripts/typecheck.sh` | Yes after EP-001 | Cargo check all targets/features. |
| `./scripts/test-unit.sh` | Yes after EP-001 | Fast tests. |
| `./scripts/test-integration.sh` | Yes after EP-003 | Persistence and service tests. |
| `./scripts/test-e2e.sh` | Yes after EP-005 | CLI acceptance tests. |
| `./scripts/build.sh` | Yes after EP-001 | Release build. |
| `./scripts/security-check.sh` | Yes after EP-006 | Redaction and safety checks. |
| `./scripts/dependency-audit.sh` | Yes before production readiness | Requires audit tooling or documented equivalent. |
| `./scripts/smoke-test.sh` | Yes after EP-005 | CLI smoke. |
| `./scripts/verify.sh` | Yes before final response when available | Full local sequence. |

## Definition of Test Done

Testing is done for an ExecPlan when required tests exist, all validation commands in the plan pass, failing/negative paths are covered, fixtures contain no secrets, and the active ExecPlan records command results.
