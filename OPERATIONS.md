# OptionClaw Operations Runbook

## Local Operations

- Run `./scripts/preflight.sh` before development work.
- Run `./scripts/verify.sh` before final review.
- Use paper mode by default.
- Store local state under `./var/dev` or another non-production path.
- Do not put real secrets in local fixtures.
- The local data directory is schema-versioned and should contain `schema.json`, `audit/events.jsonl`, `paper/state.json`, and `backups/`.

## Staging Operations

- Use target-like hardware where possible.
- Use paper or sandbox mode only.
- Validate config with the CLI.
- Run smoke tests after every deployment.
- Confirm logs are redacted.

## Production Operations

- Start in production paper mode.
- Enable live mode only after production readiness and explicit approval.
- Keep rollback artifact available.
- Monitor health, logs, risk rejections, provider errors, and kill-switch state.
- Treat live order records as production data.

## Health Checks

Planned health command:

```sh
optionclaw health --config <path>
```

Health must report:

- Config parse status.
- Data directory availability.
- Secrets store availability without revealing secrets.
- Kill switch state.
- Provider mode readiness.
- Audit log write readiness.

## Common Failure Modes

| Failure | Symptom | Immediate Action | Recovery |
|---|---|---|---|
| Invalid config | CLI exits nonzero with config error | Do not run trading command | Fix documented key and rerun check-config. |
| Missing secret | Provider initialization fails | Use mock/fixture or stop | Add encrypted secret through approved flow. |
| Kill switch active | Execution refused | Verify operator intent | Remove/clear kill switch only with operator approval. |
| Corrupt local state | State verify fails | Stop trading commands | Restore from backup or preserve corrupt file for diagnosis. |
| Provider outage | Adapter errors increase | Stay in paper/sandbox or pause | Retry within bounded policy; do not spam provider. |
| Risk rejection spike | Many intents rejected | Review strategy/config | Do not lower risk gates without a decision record. |
| Audit write failure | Execution refused | Stop trading commands | Fix disk/path permissions or restore storage. |

## Troubleshooting

1. Run `optionclaw health --config <path>` after EP-008.
2. Inspect recent structured logs.
3. Verify data directory permissions.
4. Verify config values from `ENVIRONMENT.md`.
5. Run the narrowest failing validation command.
6. Apply the anti-fixation rule from `AGENTS.md`.

## Database Backup / Restore

No database initially. Local file backup rules:

- Back up the entire data directory before schema migration.
- Verify backup readability.
- Restore only while OptionClaw is stopped.
- Preserve failed/corrupt files for postmortem.
- Treat audit append failures as execution blockers until the local disk or permissions are repaired.

## Scheduled Jobs

No scheduled jobs are required initially. If a daemon or scheduler is added, it must have a dedicated ExecPlan defining cadence, locking, missed-run behavior, observability, and shutdown semantics.

## Incident Triage

Severity guide:

- SEV-1: Live unintended order, fund movement, secret leak, or data loss.
- SEV-2: Production live trading disabled, audit failure, state corruption, repeated provider failure.
- SEV-3: Paper/sandbox failure, CLI command regression, non-critical observability issue.

## Escalation Rules

- Stop live execution immediately on SEV-1.
- Activate kill switch if execution safety is uncertain.
- Preserve logs and state files.
- Do not rotate secrets by printing or exporting them into chat/logs.
- Record incident details using `.agent/checklists/incident-response.md`.

## Maintenance Windows

- Prefer maintenance while no live positions are open.
- Back up local state before upgrades.
- Run smoke tests after maintenance.
- Verify rollback artifact before changes.

## Operational Safety Rules

- Paper mode for routine testing.
- Sandbox before live.
- Live commands require explicit approvals and gates.
- Never ignore audit write failures.
- Never disable redaction to debug production.
- Never tune risk limits during an incident without documenting the decision.
