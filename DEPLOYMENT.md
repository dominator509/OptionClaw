# OptionClaw Deployment

## Deployment Environments

| Environment | Mode | Purpose | Live Trading Allowed |
|---|---|---|---|
| Local development | paper | Development and tests | No |
| Local staging | paper or sandbox | Operator dry-run on target hardware | No live trading |
| Production paper | paper | Production-like monitoring without real orders | No |
| Production live | live | Real broker/exchange execution | Only after EP-010 and explicit approval |

## Deployment Architecture

Initial deployment is a single Rust binary on operator-controlled hardware:

```text
optionclaw binary
  + config file
  + encrypted secrets file
  + local data directory
  + logs/metrics output
  + optional systemd service or container wrapper
```

No database server is required initially.

## Build Artifact

The release artifact is the optimized Rust binary produced by:

```sh
./scripts/build.sh
```

Expected binary path after EP-001: `target/release/optionclaw`.

## Release Flow

1. Complete active ExecPlan.
2. Run `./scripts/verify.sh`.
3. Run `./scripts/production-readiness-check.sh` before production release.
4. Tag release only after verification passes.
5. Build release artifact.
6. Deploy to staging target.
7. Run smoke tests.
8. Deploy to production paper mode first.
9. Live mode requires explicit production approval.

## Deployment Steps

Planned after EP-009:

```sh
./scripts/verify.sh
./scripts/build.sh
install -m 0755 target/release/optionclaw /opt/optionclaw/bin/optionclaw
install -m 0640 config/production.example.toml /etc/optionclaw/config.toml
/opt/optionclaw/bin/optionclaw check-config --config /etc/optionclaw/config.toml
/opt/optionclaw/bin/optionclaw health --config /etc/optionclaw/config.toml
```

These commands require adaptation during EP-009 based on the actual deployment target. Do not run production install commands without explicit permission.

## Migration Steps

No database migrations initially. Local file schema migrations after EP-003 must follow:

1. Stop running OptionClaw process.
2. Run dry-run migration.
3. Create backup.
4. Run migration.
5. Verify state.
6. Run smoke test.

## Rollback Steps

Follow `ROLLBACK.md`. Minimum rollback is binary rollback plus config rollback and state verification. Data rollback is allowed only after backup verification and explicit operator approval.

## Post-Deploy Smoke Tests

```sh
optionclaw --help
optionclaw check-config --config /etc/optionclaw/config.toml
optionclaw health --config /etc/optionclaw/config.toml
```

Live trading smoke tests must not submit live orders. Use paper/sandbox mode.

## Required Approvals

- Production deployment requires explicit operator approval.
- Production live trading requires explicit operator approval after EP-010 passes.
- Real wallet signing or fund movement requires explicit operator approval and a dedicated security plan.

## Deployment STOP Conditions

Stop deployment when:

- Verification fails.
- Production readiness check fails.
- Required config or secrets are missing.
- Data migration has no backup.
- Live mode is requested without all gates.
- Rollback path has not been tested for the release.

## Production Verification

Production verification must record:

- Binary version.
- Config path and mode.
- Health check status.
- Smoke test result.
- Log redaction spot check.
- Kill switch test result in non-live mode.
- Rollback artifact availability.
