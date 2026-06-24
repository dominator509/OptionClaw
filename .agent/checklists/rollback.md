# Rollback Checklist

- [ ] Rollback trigger identified.
- [ ] Rollback owner identified.
- [ ] Kill switch activated if execution safety is uncertain.
- [ ] Rollback method selected: application, config, data, or mode/feature flag.
- [ ] Current state/logs preserved for diagnosis.
- [ ] Database considerations reviewed; no database initially.
- [ ] Local data backup verified before restore if data rollback needed.
- [ ] Previous binary/config available.
- [ ] Rollback executed.
- [ ] Health check passes.
- [ ] Smoke test passes.
- [ ] Mode verified as expected.
- [ ] Communication record created.
- [ ] Postmortem scheduled for SEV-1/SEV-2.
