@C:\Users\domin\.codex\RTK.md

# AGENTS.md

## 1. Mission

You are a coding agent operating inside the OptionClaw repository. Your mission is to implement exactly one active ExecPlan at a time, using repository evidence, tests, and documented commands. OptionClaw is a Rust-first local CLI for automated options-trading research, paper trading, and tightly gated live execution. Because the domain can cause financial loss, security incidents, and irreversible transactions, scope control and STOP conditions are mandatory.

Do not ask the user for next steps. Proceed autonomously through the active ExecPlan unless a STOP condition applies.

For compact durable repo context, read `REPO_BRIEF.md` before broad exploration. It is intended for Codex, Serena, and Obsidian linking; `AGENTS.md` remains the authority for agent rules.

## 2. Source-of-Truth Priority

When instructions conflict, use this priority order:

1. Current user instruction.
2. `AGENTS.md`.
3. The active ExecPlan under `.agent/execplans/`.
4. Existing repository code and tests.
5. `ARCHITECTURE.md`.
6. Relevant spec under `.agent/specs/`.
7. `ROADMAP.md`.

The roadmap is strategic only. Never implement directly from `ROADMAP.md`.

## 3. Required Workflow

1. Read `AGENTS.md`.
2. Read `COMMANDS.md`.
3. Read `.agent/PLANS.md`.
4. Read the active ExecPlan in full.
5. Read every file listed in the ExecPlan's "Files to Read First" section.
6. Run `./scripts/preflight.sh` from the repository root.
7. Complete milestones in the order listed in the active ExecPlan.
8. After each milestone, run the milestone validation command and record the result in the ExecPlan `Progress` section.
9. Update `Surprises & Discoveries` when repository reality differs from the plan.
10. Record every material assumption or implementation choice in the ExecPlan `Decision Log`.
11. Continue autonomously to the next milestone unless a STOP condition applies.
12. At completion, run the required validation commands, perform a diff review, update `Outcomes & Retrospective`, and provide the final response described below.

## 4. STOP Conditions

Stop only when at least one condition below applies. When stopping, provide the exact blocker, evidence from file or terminal output, the smallest decision needed, and a recommended default.

- A required secret, credential, paid service, exchange account, broker account, wallet, API key, or external account is missing and the active task cannot continue safely with a mock, fixture, sandbox, or paper-trading substitute.
- Any action may destroy, overwrite, trade with, transfer, expose, or corrupt user, wallet, broker, exchange, or production data.
- A task requires legal, tax, regulatory, security, custody, or financial judgment not already specified in repository docs.
- A user-visible behavior choice is materially different from the spec and cannot be resolved by the smallest reversible option.
- Required tests or validation commands cannot run after documented recovery attempts using the anti-fixation rules.
- A production deployment, live-trading enablement, wallet signing, fund movement, irreversible migration, or real order submission is requested without explicit permission and passing production-readiness criteria.
- Repository files conflict with the active ExecPlan in a way that would require broad refactoring or implementation outside expected changed files.
- The active ExecPlan is missing required sections and cannot be made self-contained without changing scope.

## 5. Anti-Drift Rules

- Implement one active ExecPlan only.
- Do not jump between unrelated plans.
- Do not broaden scope to add strategies, providers, UI, services, databases, cloud infrastructure, or live trading unless the active ExecPlan explicitly requires it.
- Do not perform broad refactors, formatting rewrites, dependency swaps, file reorganizations, or unrelated cleanup.
- Every ExecPlan lists expected changed files. At final review, run `git diff --name-only` and compare results to that list.
- Any extra changed file must be justified in the ExecPlan `Decision Log`.
- Non-goals in specs and ExecPlans are binding.

## 6. Anti-Hallucination Rules

- Do not invent package APIs.
- Do not invent command names.
- Do not invent environment variables.
- Do not invent database tables or file schemas.
- Do not invent routes, CLI flags, config keys, strategy names, broker endpoints, wallet RPC methods, or provider capabilities.
- Confirm names by reading repository files, generated docs, crate documentation available locally, or official provider documentation when explicitly required by a separate integration plan.
- Use commands from `COMMANDS.md` only.
- If a command is missing or stale, update `COMMANDS.md` first with evidence from the repository before running it.
- Record assumptions in `ASSUMPTIONS.md` and the active ExecPlan `Decision Log`.

## 7. Anti-Fixation Rules

For any failing validation command:

1. First failure: read the error, identify the likely cause, and make the smallest targeted fix.
2. Second same-root failure: create or run a narrower diagnostic, isolate the failure, and avoid broad rewrites.
3. Third same-root failure: stop the current approach, record failed hypotheses in `Surprises & Discoveries`, choose a simpler implementation path if safe, and continue.

Never patch blindly around the same error indefinitely. Never delete tests to make validation pass unless the active ExecPlan explicitly instructs replacing that test and the replacement preserves coverage.

## 8. Dependency Rules

- Prefer the Rust standard library and existing repository dependencies.
- Before adding a dependency, inspect `Cargo.toml`, `Cargo.lock`, and existing imports.
- Add a dependency only when the active ExecPlan requires it or the need is recorded in the Decision Log.
- Do not add broker SDKs, wallet SDKs, LLM SDKs, telemetry vendors, or databases without an ExecPlan milestone requiring them.
- Pin production dependencies through `Cargo.lock`.
- After dependency changes, run `./scripts/install.sh`, `./scripts/typecheck.sh`, and `./scripts/dependency-audit.sh` when available.

## 9. File Creation Rules

- Create only files listed in the active ExecPlan unless the need is discovered and recorded.
- Keep generated files repository-local.
- Do not create files outside the repository root.
- Do not commit real secrets, production data, broker statements, wallet keys, seed phrases, API responses containing private data, or `.env` files.
- Example configs must use fake values clearly marked as non-secret.

## 10. Testing Rules

- Tests are required for every behavior change.
- Domain logic must have unit tests.
- Persistence and adapter boundaries must have integration tests using fixtures, mocks, or local test directories.
- CLI flows must have E2E or acceptance tests.
- Security-sensitive behavior must have negative tests.
- Live trading, wallet signing, and broker order placement must be tested through mocks or sandbox fixtures unless explicit live permission exists.
- Validation commands must come from `COMMANDS.md`.

## 11. Documentation Update Rules

Update documentation in the same change when implementation affects architecture, commands, configuration, environment variables, security behavior, deployment, operations, or specs. At minimum, update the active ExecPlan `Progress`, `Decision Log`, and `Outcomes & Retrospective`.

## 12. Security Rules

- Secrets must be encrypted at rest or supplied only through environment variables or OS secret stores.
- Logs must redact secrets, private keys, bearer tokens, account IDs when configured as sensitive, wallet addresses when configured as sensitive, and complete order payloads containing credentials.
- Fail closed when risk limits, trading mode, platform credentials, or kill-switch state are missing.
- Default trading mode is `paper`.
- Live order submission requires explicit `live` mode, configured provider, validated credentials, risk limits, kill switch enabled, and production readiness approval.
- Do not implement private-key custody without a dedicated security ExecPlan.

## 13. Production Data Rules

- Treat broker, exchange, wallet, production logs, live order records, and secrets as production data.
- Never run destructive commands against production data without explicit permission.
- Never use production credentials in tests.
- Migrations or schema changes must be reversible where practical and must include backup steps.
- Local file persistence changes must include backup/restore guidance.

## 14. Definition of Done

An ExecPlan is done only when all are true:

- All acceptance criteria pass.
- Required validation commands pass.
- ExecPlan `Progress` is updated.
- `Surprises & Discoveries` and `Decision Log` are updated.
- Final diff is reviewed with `git diff --name-only` and only expected files changed or extras are justified.
- Tests cover new or changed behavior.
- Docs are updated for changed commands, config, behavior, security, or operations.
- Remaining risks are documented.
- No STOP condition remains unresolved.

## 15. Final Response Requirements

Final responses from coding agents must include:

- ExecPlan completed.
- Changed files.
- Commands run.
- Command results.
- Acceptance criteria status.
- Decisions made.
- Assumptions confirmed or changed.
- Remaining risks.
- Whether production-readiness criteria passed or which criteria remain incomplete.
