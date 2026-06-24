# Agent Readiness Checklist

Before a coding agent starts, confirm:

- [ ] Exactly one active ExecPlan is named.
- [ ] ExecPlan is self-contained and has all required sections.
- [ ] ExecPlan lists exact files to read first.
- [ ] ExecPlan lists expected files to change.
- [ ] ExecPlan lists forbidden changes.
- [ ] ExecPlan contains exact commands from `COMMANDS.md`.
- [ ] ExecPlan contains expected command outputs.
- [ ] Acceptance criteria are observable and test-backed.
- [ ] Non-goals are explicit.
- [ ] STOP conditions are known from `AGENTS.md`.
- [ ] Recovery rules and retry budget are included.
- [ ] Bounded retry rule is understood: first targeted fix, second narrower diagnostic, third change approach or stop.
- [ ] Diff review requirement is understood.
- [ ] No hidden context is required.
- [ ] No vague requirement remains such as "make it better" or "handle edge cases" without tests.
- [ ] The agent will not ask for next steps unless a STOP condition applies.
