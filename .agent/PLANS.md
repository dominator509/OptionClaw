# ExecPlan Standard

An ExecPlan is a self-contained implementation document for one feature or system change. A new agent with no prior conversation must be able to continue from the ExecPlan alone.

## Required Sections

Every ExecPlan must include exactly these sections:

1. Purpose / Big Picture
2. Scope
3. Non-goals
4. Context and Orientation
5. Files to Read First
6. Files to Change
7. Interfaces and Contracts
8. Milestones
9. Concrete Steps
10. Validation and Acceptance
11. Idempotence and Recovery
12. Progress
13. Surprises & Discoveries
14. Decision Log
15. Outcomes & Retrospective

## Execution Rules

- One active ExecPlan only.
- Do not implement directly from `ROADMAP.md`.
- Do not ask for next steps unless a STOP condition from `AGENTS.md` applies.
- Complete milestones in order.
- Validate after each milestone.
- Update the ExecPlan after each milestone.
- Use only commands from `COMMANDS.md`.
- Confirm repository facts by reading files before editing.

## Milestone Rules

Each milestone must include:

- Goal.
- Files to read.
- Files to change.
- Exact edits expected.
- Validation command.
- Expected result.
- Recovery instruction.

A milestone is complete only when its edits are done, validation passes, and progress is updated.

## Validation Rules

- Use exact commands from `COMMANDS.md`.
- Prefer narrow validation after each milestone.
- Run full verification at final review when available.
- Record command and result in `Progress`.
- If validation fails, apply anti-fixation rules from `AGENTS.md`.

## Acceptance Rules

Acceptance criteria must be observable and test-backed. Avoid vague phrases. A valid criterion names behavior, command, test, output, or file state.

## Idempotence Rules

- Re-running an ExecPlan must not duplicate config, dependencies, CI jobs, or docs.
- Check whether files and sections already exist before adding them.
- If a prior partial implementation exists, continue from the first incomplete milestone.

## Recovery Rules

- Use the smallest safe fix for failures.
- If repository reality differs from the plan, record it in `Surprises & Discoveries`.
- If a broader change is needed, record a decision and continue only if it remains within scope.
- Stop only under `AGENTS.md` STOP conditions.

## Progress Update Rules

Use checkboxes with dated notes. Update after every milestone:

```text
- [x] M1 - Completed YYYY-MM-DD. Validation: <command> -> <result>.
- [ ] M2 - Not started.
```

## Decision Log Rules

Record:

- Dependency additions.
- File additions outside expected files.
- Spec interpretations.
- Command changes.
- Risk/security choices.
- Any assumption confirmed or changed.

## Completion Rules

An ExecPlan is complete when:

- All milestones are checked complete.
- Acceptance criteria pass.
- Required validation commands pass.
- Diff review is complete.
- Only expected files changed or extras are justified.
- Outcomes & Retrospective is updated.
- Remaining risks are documented.
