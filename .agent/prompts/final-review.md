# Prompt: Final Review

Read `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, and `[EXECPLAN_PATH]`.

Perform final review for `[EXECPLAN_PATH]`.

Rules:

- Run the required final validation commands from the ExecPlan.
- Run `./scripts/verify.sh` if available and required.
- Run `./scripts/production-readiness-check.sh` if the ExecPlan requires production readiness.
- Run `git diff --name-only`.
- Compare changed files with the ExecPlan expected changed files.
- Verify every acceptance criterion.
- Confirm docs are updated.
- Confirm no secrets or production data are present.
- Update `Outcomes & Retrospective`.
- Produce a final report listing changed files, commands run, command results, acceptance criteria status, decisions, assumptions, remaining risks, and production-readiness status.
- Stop only for STOP conditions in `AGENTS.md`.
