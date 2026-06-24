# Consolidated Execution Rules

## One Active ExecPlan Rule

Implement exactly one active ExecPlan. Do not start another plan until the active plan is complete or a STOP condition applies.

## No Hidden Context Rule

Assume no prior conversation. Use only repository files, active user instruction, and terminal evidence.

## No Roadmap-Only Implementation Rule

`ROADMAP.md` is strategic. Do not implement from it directly. Create or use an ExecPlan.

## Continue-by-Default Rule

Do not ask for next steps. Continue through milestones in order, validate, update progress, and proceed.

## STOP-Only Rule

Stop only for STOP conditions in `AGENTS.md`. When stopping, provide exact evidence and smallest decision needed.

## Anti-Drift Rule

Do not broaden scope, refactor unrelated files, add unplanned features, or change architecture boundaries unless the active ExecPlan requires it.

## Anti-Hallucination Rule

Do not invent commands, APIs, imports, config keys, environment variables, routes, data schemas, or provider capabilities. Verify or create them explicitly.

## Anti-Fixation Rule

After repeated validation failures: first targeted fix, second narrower diagnostic, third abandon that approach, record failed hypotheses, choose simpler safe path, or stop if required.

## Test-Before-Completion Rule

Every behavior change must have tests. Completion without required tests is not allowed.

## Diff Review Rule

Run `git diff --name-only` before final response. Compare to expected changed files in the active ExecPlan. Justify extras in the Decision Log.

## Final Response Rule

Final response must include completed ExecPlan, changed files, commands run, results, acceptance status, decisions, assumptions, risks, and production-readiness status.
