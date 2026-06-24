# Prompt: Debug a Validation Failure

Read `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, and `[EXECPLAN_PATH]`.

Failing command: `[FAILING_COMMAND]`

Optional context: `[OPTIONAL_USER_REQUEST]`

Debug the failing validation command without broad rewrites.

Rules:

- Do not rewrite unrelated code.
- Capture the exact failing command.
- Capture the exact error output.
- Form one hypothesis at a time.
- Make the smallest targeted fix.
- Rerun the narrowest command that validates the hypothesis.
- On the first same-root failure, inspect and make a small fix.
- On the second same-root failure, create or run a narrower diagnostic.
- On the third same-root failure, stop the current approach, record failed hypotheses in `Surprises & Discoveries`, choose a simpler safe implementation path, and continue if safe.
- Update `[EXECPLAN_PATH]` with the error, hypothesis, fix, and validation result.
- Stop only for STOP conditions in `AGENTS.md`.
