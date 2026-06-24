# Prompt: Execute Active ExecPlan

Read `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, and `[EXECPLAN_PATH]`.

Optional user request/context: `[OPTIONAL_USER_REQUEST]`

Implement `[EXECPLAN_PATH]` to completion.

Rules:

- Do not ask for next steps.
- Do not implement from `ROADMAP.md` directly.
- Do not broaden scope.
- Stop only for STOP conditions in `AGENTS.md`.
- Run `./scripts/preflight.sh` before editing.
- Complete milestones in order.
- Validate after each milestone using commands from `COMMANDS.md`.
- Update `[EXECPLAN_PATH]` Progress after each milestone.
- Record surprises and decisions in `[EXECPLAN_PATH]`.
- Do not invent commands, APIs, config keys, environment variables, data schemas, or provider capabilities.
- Apply the anti-fixation rule for repeated failures.
- At the end, run required final validation, run `git diff --name-only`, update Outcomes & Retrospective, and report changed files, commands, results, decisions, risks, and acceptance status.
