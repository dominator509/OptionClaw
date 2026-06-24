# OptionClaw Release Process

## Release Types

| Type | Description | Required Checks |
|---|---|---|
| Development snapshot | Local unfinished build | Milestone validation only. |
| Release candidate | Candidate artifact for staging/paper mode | Full verification and smoke tests. |
| Production paper release | Production deployment in paper mode | Production readiness check and rollback plan. |
| Production live release | Live-capable deployment | Production paper release plus explicit live approval and all live gates. |

## Versioning

Use semantic versioning once the first release is created:

- MAJOR: incompatible CLI/config/state changes.
- MINOR: backwards-compatible features.
- PATCH: fixes and non-breaking operational updates.

## Changelog

Maintain `CHANGELOG.md` after EP-009 if releases begin. Each entry must include:

- Version.
- Date.
- Added/changed/fixed/security notes.
- Migration notes.
- Rollback notes.
- Known risks.

## Branch Strategy

If no branch strategy exists, use:

- `main`: releasable state.
- feature branches for active ExecPlans.
- no direct production edits.

Update this section if repository conventions differ.

## Release Candidate Criteria

- Active ExecPlan complete.
- Full verification passes.
- Dependency audit reviewed.
- No secrets in diff.
- Smoke test passes.
- Release notes drafted.
- Rollback method documented.

## Release Checklist

- [ ] Confirm version.
- [ ] Update changelog/release notes.
- [ ] Run `./scripts/verify.sh`.
- [ ] Run `./scripts/production-readiness-check.sh` for production releases.
- [ ] Build release artifact.
- [ ] Deploy to staging/paper target.
- [ ] Run staging smoke tests.
- [ ] Confirm logs are redacted.
- [ ] Confirm rollback artifact exists.
- [ ] Obtain required approval for production deployment.
- [ ] Deploy.
- [ ] Run post-deploy smoke tests.
- [ ] Monitor operational signals.

## Smoke Tests

At minimum:

```sh
optionclaw --help
optionclaw check-config --config <path>
optionclaw health --config <path>
```

Do not submit live orders in smoke tests.

## Approvals

- Production deployment requires operator approval.
- Production live trading requires operator approval after production-readiness criteria pass.
- Fund movement or wallet signing requires dedicated approval and a security plan.

## Release Notes

Release notes must include:

- User-visible changes.
- Config changes.
- State/schema changes.
- Security changes.
- Known limitations.
- Rollback instructions.

## Post-Release Monitoring

Monitor:

- Health check status.
- Error logs.
- Risk rejection counts.
- Provider errors.
- Audit append failures.
- Kill switch state.
- Resource usage on local hardware/VPS.
