# SPEC-005 Auth and Permissions

## Status

Draft baseline for local CLI security.

## Owner

Blueprint / security owner.

## Linked Roadmap Phase

Phase 5: Auth, permissions, and security.

## Linked ExecPlans

EP-006, EP-010.

## User-Visible Goal

OptionClaw protects secrets and prevents unauthorized dangerous behavior in a single-user local CLI deployment.

## Non-Goals

- No user login system initially.
- No roles/teams/tenant model initially.
- No web session management initially.
- No private-key custody initially.

## Terms

- Authentication: proving user identity. Out of scope for initial local CLI.
- Authorization: deciding whether a command/mode can proceed. Implemented through config, file permissions, live gates, and kill switch.
- Kill switch: configured local state that disables execution.

## Required Behavior

- Local CLI has no login, but sensitive files must use restrictive permissions where supported.
- Paper mode requires no secrets.
- Sandbox/live provider modes require documented secrets.
- Live mode requires explicit enable flag, risk limits, kill switch checks, production readiness, and operator approval.
- Secret values must not appear in logs/errors.
- Wallet signing is disabled unless a dedicated plan implements it.

## Inputs

- Config mode.
- Environment variables.
- Encrypted secrets file.
- Kill switch state.
- File permissions.

## Outputs

- Authorization decision.
- Redacted error messages.
- Audit/security log events.

## Error States

- Missing required secret.
- Insecure file permissions where enforceable.
- Live mode requested but disabled.
- Kill switch active.
- Secret decryption failure.
- Unsupported wallet signing request.

## Data Rules

- Store only encrypted secrets.
- Store audit events without secret values.
- Do not retain LLM prompts containing secrets.

## Security Rules

- Fail closed.
- Redact before logging.
- Test all negative paths.
- Do not add auth/web sessions without a separate plan.

## Accessibility Rules

Security errors must explain safe next action without exposing sensitive values.

## Performance Rules

Secret loading should not block normal help/config validation unnecessarily.

## Observability Rules

Security decisions must log event type, mode, provider, and result without secrets.

## Required Tests

- Redaction test.
- Missing secret failure.
- Paper mode no-secret success.
- Live mode missing-gate failure.
- Kill switch active failure.
- Encrypted secret file no-plaintext test.

## Acceptance Criteria

- Security baseline is implemented even without user login.
- Live mode fails closed.
- Secrets are encrypted/redacted.
- Tests pass.
