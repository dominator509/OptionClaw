# SPEC-002 Data Model

## Status

Draft baseline for no-database local persistence.

## Owner

Blueprint / data owner.

## Linked Roadmap Phase

Phase 2: Data and persistence.

## Linked ExecPlans

EP-003, EP-007, EP-010.

## User-Visible Goal

OptionClaw can persist local paper-trading state, audit records, schema metadata, and encrypted secrets metadata without requiring an external database.

## Non-Goals

- No external database server initially.
- No multi-user shared state.
- No production live order persistence before live trading gates.
- No unencrypted secret storage.

## Terms

- Data directory: root for OptionClaw local state.
- Schema version: version marker for persisted file formats.
- Audit log: append-only records of decision and execution lifecycle events.
- Backup: point-in-time copy before migration or rollback.

## Required Behavior

- Initialize data directory idempotently.
- Store schema version.
- Append audit records without overwriting existing records.
- Write state atomically using temp-file then rename pattern.
- Verify state integrity.
- Detect unsupported schema versions.
- Provide dry-run migration behavior before modifying data.
- Back up state before migrations.

## Inputs

- Data directory path.
- Domain records.
- Audit event data.
- Migration commands.
- Backup target path.

## Outputs

- Schema metadata file.
- Audit log file.
- Paper-trading state files.
- Backup directory/archive.
- Verification report.

## Error States

- Path not writable.
- Missing schema metadata.
- Unsupported schema version.
- Corrupt JSON/record.
- Audit append failure.
- Backup failure.
- Migration dry-run failure.

## Data Rules

- Use schema version `1` for initial local files.
- Audit records must include timestamp, event type, mode, order intent ID where applicable, risk decision, and redacted context.
- No raw secrets in audit records.
- Preserve corrupt files; do not overwrite them during recovery.

## Security Rules

- Secrets must be encrypted and separated from audit logs.
- State files must avoid credentials.
- File permissions should be restrictive where supported.

## Accessibility Rules

State verification messages must be readable and action-oriented.

## Performance Rules

Audit append should be lightweight enough for per-intent recording on local disk.

## Observability Rules

Persistence failures must emit structured error fields and fail closed for execution paths.

## Required Tests

- State init idempotence.
- State verify success.
- Corrupt file detection.
- Unsupported schema rejection.
- Audit append and readback.
- Atomic write behavior with temp files.
- Backup and restore.
- Migration dry-run.

## Acceptance Criteria

- No external DB is required.
- Integration tests pass for persistence operations.
- CLI state commands work after EP-005.
- Data docs and rollback docs are updated.
