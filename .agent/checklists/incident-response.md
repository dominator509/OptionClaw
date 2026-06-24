# Incident Response Checklist

## Detect

- [ ] Alert, operator report, failed health check, or anomalous logs identified.
- [ ] Severity assigned.
- [ ] Time detected recorded.

## Triage

- [ ] Confirm mode: paper, sandbox, or live.
- [ ] Identify affected command/provider/data directory.
- [ ] Capture correlation/order intent IDs if available.
- [ ] Preserve redacted logs and state.

## Mitigate

- [ ] Activate kill switch if execution safety is uncertain.
- [ ] Stop running process if needed.
- [ ] Disable live mode if involved.
- [ ] Prevent further provider/wallet actions.

## Communicate

- [ ] Notify operator/release owner.
- [ ] Do not share secrets in messages.
- [ ] Record current status and next action.

## Resolve

- [ ] Apply smallest safe fix or rollback.
- [ ] Run narrow validation.
- [ ] Run smoke test.
- [ ] Verify health.

## Verify

- [ ] Confirm no ongoing unintended execution.
- [ ] Confirm logs are redacted.
- [ ] Confirm state integrity.
- [ ] Confirm mode and kill-switch state.

## Document

- [ ] Write incident summary.
- [ ] Record root cause or current hypothesis.
- [ ] Record commands run and results.
- [ ] Record data/config changes.

## Follow Up

- [ ] Add regression test.
- [ ] Update runbook/docs.
- [ ] Update decision/assumption logs if needed.
- [ ] Review release/rollback process.
