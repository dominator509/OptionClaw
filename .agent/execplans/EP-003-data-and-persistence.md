# EP-003 Data and Persistence

## 1. Purpose / Big Picture

Implement no-database local persistence for OptionClaw: schema-versioned state files, append-only audit logs, backups, restore, dry-run migration, and integration tests.

## 2. Scope

- Local data directory initialization and verification.
- Schema version metadata.
- Paper-trading state persistence.
- Append-only audit log with redacted records.
- Atomic writes.
- Backup and restore helpers.
- Migration dry-run behavior.
- Integration tests with temporary directories.

## 3. Non-goals

- No external database.
- No live broker order persistence.
- No real secrets storage yet except placeholders for future encrypted secrets metadata.
- No cloud backup.
- No destructive migration without backup.

## 4. Context and Orientation

The project requires no database, but autonomous trading needs restartable state and audits. Persistence must be local, explicit, schema-versioned, and safe. Execution must fail closed if audit append fails.

## 5. Files to Read First

- `ARCHITECTURE.md`
- `.agent/specs/SPEC-002-data-model.md`
- `.agent/specs/SPEC-006-error-handling.md`
- `OPERATIONS.md`
- `ROLLBACK.md`
- `src/persistence/mod.rs`
- `src/domain/*`
- `src/errors/mod.rs`
- Existing tests

## 6. Files to Change

Expected changed files:

- `src/persistence/mod.rs`
- `src/persistence/schema.rs`
- `src/persistence/audit.rs`
- `src/persistence/state.rs`
- `src/persistence/backup.rs`
- `src/errors/mod.rs`
- `src/lib.rs`
- `tests/integration_persistence.rs`
- `fixtures/state/README.md`
- `OPERATIONS.md`
- `ROLLBACK.md`
- `.agent/execplans/EP-003-data-and-persistence.md`

Forbidden changes:

- External database setup.
- Broker/wallet integrations.
- Live execution code.
- Secret encryption implementation beyond interfaces needed for later EP-006.

## 7. Interfaces and Contracts

Planned service/persistence contracts:

- `init_data_dir(path) -> Result<StateReport>` idempotently creates structure.
- `verify_data_dir(path) -> Result<StateReport>` validates schema and files.
- `append_audit(path, AuditEvent) -> Result<()>` appends secret-free event.
- `write_state_atomic(path, State) -> Result<()>` uses temp write and rename.
- `backup_data_dir(path, backup_path) -> Result<BackupReport>`.
- `migrate_dry_run(path) -> Result<MigrationReport>`.

Initial files under data dir:

```text
schema.json
audit/events.jsonl
paper/state.json
backups/
```

## 8. Milestones

### M1: Data directory and schema metadata

- Goal: Create idempotent init/verify for local state.
- Files to read: `SPEC-002`, current persistence module.
- Files to change: `src/persistence/schema.rs`, `src/persistence/state.rs`, `src/persistence/mod.rs`, tests.
- Exact edits expected: Create schema version `1`, directory layout, init/verify functions, integration tests.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If test script expects a different test name, update script and `COMMANDS.md` with evidence.

### M2: Audit log append and redacted records

- Goal: Add append-only audit events.
- Files to read: `SECURITY.md`, `OBSERVABILITY.md`, domain order/risk types.
- Files to change: `src/persistence/audit.rs`, `tests/integration_persistence.rs`.
- Exact edits expected: Define audit event struct, JSON Lines append, no secret fields, readback test.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If serialization dependency missing, check existing serde usage; add minimal serde support and record decision.

### M3: Atomic state writes and corruption detection

- Goal: Prevent partial writes and fail closed on corrupt state.
- Files to read: `ROLLBACK.md`, `SPEC-002`.
- Files to change: `src/persistence/state.rs`, tests.
- Exact edits expected: Write temp file then rename; detect invalid JSON/schema; preserve corrupt files.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok` including corruption tests.
- Recovery instruction: If fsync behavior is platform-specific, implement rename-based atomicity and record fsync limitation.

### M4: Backup, restore, and migration dry-run

- Goal: Add safe data-change operations.
- Files to read: `OPERATIONS.md`, `ROLLBACK.md`.
- Files to change: `src/persistence/backup.rs`, `OPERATIONS.md`, `ROLLBACK.md`, tests.
- Exact edits expected: Backup copy helper, restore guidance tests where practical, migration dry-run report for schema v1 no-op.
- Validation command: `./scripts/test-integration.sh`
- Expected result: `integration tests: ok`.
- Recovery instruction: If archive format is unclear, use directory copy as smallest reversible option and document decision.

### M5: Persistence final validation

- Goal: Confirm persistence is safe and documented.
- Files to read: changed files.
- Files to change: EP-003 progress/outcomes.
- Exact edits expected: Update plan, decisions, docs if commands or paths changed.
- Validation command: `./scripts/lint.sh && ./scripts/format-check.sh && ./scripts/typecheck.sh && ./scripts/test-integration.sh`
- Expected result: all commands print ok.
- Recovery instruction: If persistence tests are flaky due timing/temp paths, remove timing dependence and use deterministic temp directories.

## 9. Concrete Steps

1. Run preflight.
2. Inspect existing persistence scaffold.
3. Implement schema and state init/verify.
4. Implement audit log append.
5. Implement atomic writes and corruption handling.
6. Implement backup and dry-run migration.
7. Update operations/rollback docs.
8. Run validation and update this plan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/lint.sh
./scripts/format-check.sh
./scripts/typecheck.sh
./scripts/test-integration.sh
git diff --name-only
```

Acceptance criteria:

- No external database required.
- Data directory init/verify is idempotent.
- Audit append is append-only and secret-free.
- Corrupt state fails closed.
- Backup and dry-run migration behavior exists.
- Integration tests pass.

## 11. Idempotence and Recovery

Init must be safe to rerun. Backup names must not overwrite existing backups unless explicitly requested. Failed writes must not corrupt existing state. If migration behavior is incomplete, fail closed and document as blocker for production.

## 12. Progress

- [x] M1 - Data directory and schema metadata. Completed 2026-06-23. Validation: `cargo test --test integration_persistence --all-features --offline` -> passed.
- [x] M2 - Audit log append and redacted records. Completed 2026-06-23. Validation: `cargo test --test integration_persistence --all-features --offline` -> passed.
- [x] M3 - Atomic state writes and corruption detection. Completed 2026-06-23. Validation: `cargo test --test integration_persistence --all-features --offline` -> passed.
- [x] M4 - Backup, restore, and migration dry-run. Completed 2026-06-23. Validation: `cargo test --test integration_persistence --all-features --offline` -> passed.
- [x] M5 - Persistence final validation. Completed 2026-06-23. Validation: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features --offline -- -D warnings`, `cargo check --all-targets --all-features --offline`, `cargo test --lib --bins --all-features --offline`, `cargo test --test integration_smoke --all-features --offline`, `cargo test --test integration_persistence --all-features --offline`, and `cargo test --test e2e_cli --all-features --offline` -> passed.

## 13. Surprises & Discoveries

Record repository differences and validation failures here.

- `./scripts/preflight.sh`, `./scripts/install.sh`, and `./scripts/dependency-audit.sh` could not execute in this Windows shell because the repo scripts are POSIX `sh` wrappers and `cmd.exe` treats them as plain text. Cargo-native offline validation succeeded.
- Adding `serde_json` was the smallest dependency change that let the persistence layer use explicit JSON and JSONL files without hand-rolled parsing.
- The persistence integration suite now covers init/verify, append-only audit logging, atomic state writes, corruption detection, backup/restore, and dry-run migration in a single temp-directory harness.

## 14. Decision Log

Record persistence format and backup decisions here.

- Added `serde_json` to keep the file formats explicit, schema-versioned, and testable with standard JSON tooling.
- Chose a recursive directory copy for backups because it is the smallest reversible local backup strategy and preserves the exact state-layout shape required by rollback guidance.
- Local persistence stores `schema.json`, `audit/events.jsonl`, `paper/state.json`, and `backups/` under the data directory.
- Added `cargo test --test integration_persistence --all-features --offline` to `COMMANDS.md` so the new persistence integration suite is documented alongside the existing offline Cargo fallbacks.
- The install and dependency-audit wrappers were attempted and blocked by the shell environment, not by a repository logic failure.

## 15. Outcomes & Retrospective

EP-003 is complete. The local persistence layer now initializes and verifies a schema-versioned data directory, appends redacted audit records, writes paper state atomically, detects corruption and unsupported schema versions, and creates backups with a dry-run migration path for schema v1. The required cargo-native validations passed offline.

Remaining risk: the repository's shell-wrapper scripts still cannot run in this Windows shell, so future persistence changes should keep the documented Cargo fallback path up to date. The install and dependency audit wrappers were attempted and blocked by the shell environment, not by code.
