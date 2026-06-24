# OptionClaw Rollback Process

## Rollback Triggers

Rollback when any of these occur:

- Smoke tests fail after deployment.
- Health check fails.
- Secret leakage is suspected.
- Audit append fails.
- Local state migration corrupts data.
- Unintended live order or fund movement occurs.
- Critical CLI command regression.
- Provider integration behaves differently from contract tests.

## Rollback Decision Owner

The operator or release owner makes the rollback decision. During a SEV-1 incident, activate kill switch first, then rollback when safe.

## Rollback Types

| Type | Use When | Notes |
|---|---|---|
| Application rollback | New binary is faulty | Restore previous binary artifact. |
| Config rollback | Config causes failure | Restore prior config file from backup. |
| Data rollback | State migration/data corruption | Requires verified backup and explicit approval. |
| Feature flag / mode rollback | Live/sandbox mode unsafe | Return to paper mode or disable feature. |

## Application Rollback

Planned production steps after EP-009:

```sh
systemctl stop optionclaw
cp /opt/optionclaw/releases/<previous>/optionclaw /opt/optionclaw/bin/optionclaw
systemctl start optionclaw
optionclaw health --config /etc/optionclaw/config.toml
```

Adapt service commands to actual deployment. Do not run production service commands without permission.

## Database Rollback

No database initially. For local file state:

1. Stop OptionClaw.
2. Preserve current data directory for investigation.
3. Verify backup exists and matches expected schema version and layout.
4. Restore backup directory contents into the data directory.
5. Run state verification.
6. Run smoke test.

Current local layout:

- `schema.json`
- `audit/events.jsonl`
- `paper/state.json`
- `backups/`

## Config Rollback

1. Stop OptionClaw if running as a daemon.
2. Restore previous config file.
3. Run `optionclaw check-config --config <path>`.
4. Start OptionClaw or rerun command.
5. Run health check.

## Feature Flag / Mode Rollback

Initial mode rollback means setting trading mode to `paper` and ensuring live enablement is false. Also activate kill switch if execution safety is uncertain.

## Verification After Rollback

- Health check passes.
- Smoke test passes.
- Logs are redacted.
- State verification passes if state was changed.
- Mode is expected.
- No unintended live execution occurs.

## Communication

Record:

- Trigger.
- Time detected.
- Version rolled back from/to.
- Data/config changes.
- Verification results.
- Remaining risk.

## Postmortem

For SEV-1 and SEV-2 incidents, complete incident response checklist and add regression tests or runbook updates before the next release.
