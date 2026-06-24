# SPEC-004 UI / UX Behavior

## Status

Draft baseline for CLI-first interface.

## Owner

Blueprint / CLI owner.

## Linked Roadmap Phase

Phase 4: UI or client layer.

## Linked ExecPlans

EP-005, EP-007.

## User-Visible Goal

A user can operate OptionClaw through clear CLI commands, deterministic exit codes, readable status messages, and safe default behavior.

## Non-Goals

- No web UI.
- No mobile UI.
- No background daemon UX initially.
- No color-only status.

## Terms

- CLI: command-line interface and nearest equivalent user interaction layer.
- Empty state: no data directory, no audit records, or no positions.
- Loading state: long-running command progress in terminal.
- Error state: command failure with nonzero exit code.

## Required Behavior

- `optionclaw --help` lists commands and safety defaults.
- Commands show current mode when relevant.
- Paper mode is visibly labeled.
- Live mode unavailable/error messages are explicit and safe.
- Empty state messages explain next command.
- Errors include a stable code and safe next action.
- Success messages are concise and script-friendly.

## Inputs

- CLI flags and subcommands.
- Config files.
- Data directory paths.
- Fixture files.

## Outputs

- Human-readable terminal output.
- Exit code `0` on success.
- Nonzero exit code on failure.
- Optional JSON output only when documented and tested.

## Error States

- Missing arguments.
- Invalid config path.
- Invalid data directory.
- Fixture parse failure.
- Risk rejection.
- Live mode disabled.

## Data Rules

- Do not print secrets.
- Do not print full private account data.
- Use stable IDs in outputs where possible.

## Security Rules

- Dangerous live or wallet commands must not exist until gated.
- CLI must refuse live mode without required gates.
- Warnings cannot replace hard failures for unsafe operations.

## Accessibility Rules

- Operable through keyboard-only terminal.
- No required color interpretation.
- Clear text labels for status.
- Help text available for each command.

## Performance Rules

- Help and validation commands must not perform network calls.
- Long-running operations should show progress or a clear status line after EP-008 if needed.

## Observability Rules

- CLI output is user-facing; logs are operational. Do not mix secrets into either.

## Required Tests

- E2E help command.
- E2E check-config success/failure.
- E2E state init/verify after persistence exists.
- E2E paper run-once after service exists.
- E2E live disabled error.
- Snapshot or predicate tests for non-color-only messages.

## Acceptance Criteria

- CLI flows satisfy required behavior.
- E2E tests pass.
- Errors are actionable and redacted.
- No web/mobile UI is introduced.
