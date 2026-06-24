# OptionClaw Security Guidance

## Security Goals

- Prevent committed secrets.
- Prevent plaintext secret persistence.
- Prevent secret leakage through logs, errors, panic output, tests, and audit records.
- Prevent accidental live trading, wallet signing, or fund movement.
- Enforce risk gates and kill-switch checks before execution.
- Fail closed when configuration is missing or invalid.

## Threat Model Summary

| Threat | Control |
|---|---|
| API key committed to git | `.gitignore`, secret scans, docs, review checklist. |
| Secret printed in logs | Redaction wrapper and tests. |
| LLM hallucination triggers order | Deterministic risk gate after LLM output. |
| Misconfigured live trading | Default paper mode and live-mode validation gates. |
| Local state corruption | Atomic writes, backups, schema verification. |
| Broker/wallet credential misuse | Secrets boundary, least privilege, no private-key custody by default. |
| Dependency vulnerability | Dependency audit and review. |
| Unsafe provider integration | Contract tests, sandbox first, STOP for live credentials. |

## Authentication Rules

Initial local CLI mode has no user login. The operator controls local filesystem access. If a daemon, web UI, or multi-user mode is added, authentication must be specified in a new ExecPlan and spec before implementation.

## Authorization Rules

Initial authorization is command/config based:

- Default mode is paper trading.
- Live mode requires explicit config, risk limits, kill switch, provider credentials, and production-readiness approval.
- Dangerous commands must refuse to run without explicit documented gates.
- File permissions must restrict local secrets and state where supported.

## Input Validation Rules

Validate all external inputs at boundaries:

- CLI args.
- Config files.
- Environment variables.
- Market data payloads.
- LLM outputs.
- Broker/exchange responses.
- Wallet responses.
- Local state files.

Invalid inputs must produce typed errors and fail closed.

## Output Encoding Rules

- Terminal output must redact secrets.
- JSON output must not include raw credentials.
- Logs must use structured fields and redaction.
- Error messages must be useful without exposing sensitive values.

## Secret Management Rules

- No real secrets in repository files.
- Environment variables must be documented in `ENVIRONMENT.md` before use.
- Local encrypted secrets must use a documented KDF and authenticated encryption.
- Never store wallet seed phrases or private keys unless a dedicated security ExecPlan approves custody behavior.
- Tests use fake secrets only.

## Dependency Security Rules

- Add dependencies only through active ExecPlans.
- Prefer maintained crates with clear licenses.
- Run `./scripts/dependency-audit.sh` after dependency changes and before release.
- Do not use unofficial broker/wallet SDKs for live trading without a decision record.

## Logging Redaction Rules

Redact:

- API keys.
- Bearer tokens.
- Passwords.
- Private keys and seed phrases.
- Account IDs if marked sensitive.
- Wallet addresses if marked sensitive.
- Raw provider request headers.
- Any config value marked `secret` in `ENVIRONMENT.md`.

## Data Protection Rules

- Local state directory must be configurable.
- Secret files must use restrictive permissions where the OS supports it.
- Audit logs must avoid secrets and should include IDs, not credential values.
- Backup files must inherit protection requirements.

## Production Data Rules

- Never use production credentials in unit, integration, E2E, or smoke tests.
- Never run destructive commands on production data without explicit permission.
- Production logs must be considered sensitive.
- Live order records must be backed up before schema changes.

## Safe Migration Rules

- Migrations must support dry-run.
- Migrations must create backups before modification.
- Migrations must fail closed on unknown schema versions.
- Migrations must preserve original corrupt files for inspection.

## API Security Rules

Provider/API integration rules:

- Use TLS-capable clients.
- Respect provider authentication requirements.
- Map provider errors without logging credentials.
- Implement rate-limit handling where provider contracts require it.
- Use sandbox/testnet before live endpoints.

## CSRF/CORS/Session Rules

Not applicable for initial CLI-only architecture. If a web server is introduced, create a dedicated spec and ExecPlan for CSRF, CORS, sessions, cookies, and browser security headers.

## Rate Limiting Rules

- Provider adapters must honor documented provider rate limits.
- Local retry loops must use bounded retries and backoff.
- The system must not spam LLM, broker, exchange, wallet, or news endpoints after repeated failures.

## File Upload Rules

Not applicable for initial CLI-only architecture. Imported local files such as configs and fixtures must be validated and treated as untrusted input.

## Security Checklist

- [ ] No real secrets in git diff.
- [ ] Secret config keys documented.
- [ ] Redaction tests pass.
- [ ] Live trading disabled by default.
- [ ] Risk gates cannot be bypassed.
- [ ] Local secret files are encrypted.
- [ ] Dependency audit reviewed.
- [ ] Provider integrations use sandbox or fixtures.
- [ ] Error messages are redacted.
- [ ] Production data rules followed.

## STOP Conditions for Security-Sensitive Actions

Stop before any action that would:

- Submit a live order.
- Transfer or withdraw funds.
- Sign a wallet transaction.
- Store a private key or seed phrase.
- Print or export secrets.
- Change production data or live order records.
- Disable risk gates, redaction, audit, or kill switch behavior.
