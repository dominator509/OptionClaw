# Prompt: Continue a Partially Completed ExecPlan

Read `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, and `[EXECPLAN_PATH]`.

Optional user request/context: `[OPTIONAL_USER_REQUEST]`

Continue `[EXECPLAN_PATH]` from its current state.

Rules:

- Inspect `Progress` first.
- Inspect `Surprises & Discoveries`.
- Inspect `Decision Log`.
- Resume at the first incomplete milestone.
- Validate prior assumptions against repository files before editing.
- Do not redo completed work unless validation proves it is broken.
- Continue autonomously through remaining milestones.
- Do not ask for next steps.
- Stop only for STOP conditions in `AGENTS.md`.
- Use only commands from `COMMANDS.md`.
- Update the ExecPlan as work proceeds.
- Run final validation and diff review before final response.
