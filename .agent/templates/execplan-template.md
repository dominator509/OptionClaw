# EP-XXX Title

## 1. Purpose / Big Picture

State the user-visible and architectural outcome. A new agent with no prior conversation must understand why this plan exists.

## 2. Scope

List included work.

## 3. Non-goals

List excluded work. Non-goals are binding.

## 4. Context and Orientation

Summarize relevant repository state, specs, and assumptions.

## 5. Files to Read First

- `AGENTS.md`
- `COMMANDS.md`
- Add specific files here.

## 6. Files to Change

Expected changed files:

- `path/to/file`

Forbidden changes:

- List explicit forbidden changes.

## 7. Interfaces and Contracts

Define exact functions, commands, routes, config keys, data schemas, or CLI behavior.

## 8. Milestones

### M1: Name

- Goal:
- Files to read:
- Files to change:
- Exact edits expected:
- Validation command:
- Expected result:
- Recovery instruction:

## 9. Concrete Steps

1. Run `./scripts/preflight.sh`.
2. Inspect required files.
3. Implement milestones in order.
4. Validate after each milestone.
5. Update this ExecPlan.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/verify.sh
git diff --name-only
```

Acceptance criteria:

- [ ] Add objective criteria.

## 11. Idempotence and Recovery

Explain safe re-run behavior and recovery from partial changes.

## 12. Progress

- [ ] M1 - Not started.

## 13. Surprises & Discoveries

Record unexpected repository facts, validation failures, and changed assumptions.

## 14. Decision Log

Record decisions, dependencies, command changes, and extra changed files.

## 15. Outcomes & Retrospective

Complete at the end with validation results, changed files, risks, and production-readiness status.
