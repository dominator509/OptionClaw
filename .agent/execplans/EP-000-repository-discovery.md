# EP-000 Repository Discovery

## 1. Purpose / Big Picture

Discover the actual repository structure, stack, commands, current implementation state, risks, and missing information before implementation. This plan prevents lower-tier agents from guessing commands, APIs, package managers, CI, or architecture.

## 2. Scope

- Inventory repository files and current git state.
- Detect Rust/Cargo workspace or absence of project files.
- Detect package manager, tests, CI, config, environment files, scripts, and docs.
- Identify risks and missing information.
- Update `COMMANDS.md`, `ARCHITECTURE.md`, and `ASSUMPTIONS.md` only when repository evidence differs from this blueprint.

## 3. Non-goals

- Do not implement product features.
- Do not add dependencies.
- Do not create broker, wallet, LLM, or trading code.
- Do not perform broad cleanup.
- Do not run live trading or external provider commands.

## 4. Context and Orientation

The input says this is a greenfield Rust project, but the local repository may contain files. Repository evidence wins over assumptions unless a STOP condition applies. Use discovery to make later ExecPlans accurate.

## 5. Files to Read First

- `AGENTS.md`
- `COMMANDS.md`
- `ASSUMPTIONS.md`
- `ARCHITECTURE.md`
- `PROJECT_BRIEF.md`
- `ROADMAP.md`
- Existing `Cargo.toml` if present
- Existing `.github/workflows/*` if present
- Existing scripts under `scripts/`
- Existing README/docs if present

## 6. Files to Change

Expected changed files:

- `ASSUMPTIONS.md`
- `COMMANDS.md`
- `ARCHITECTURE.md`
- `.agent/execplans/EP-000-repository-discovery.md`

Forbidden changes:

- Source code changes unless needed only to inspect generated files.
- Dependency changes.
- CI implementation changes.
- Feature code.

## 7. Interfaces and Contracts

Discovery commands are read-only except documentation updates. Use only commands from `COMMANDS.md` or POSIX read-only inventory commands listed below.

Allowed inventory commands:

```sh
pwd
ls -la
find . -maxdepth 3 -type f | sort
find . -maxdepth 3 -type d | sort
git status --short
git branch --show-current
command -v cargo || true
cargo --version || true
rustc --version || true
find . -maxdepth 3 -name 'Cargo.toml' -o -name 'package.json' -o -name 'Makefile' -o -name 'justfile' | sort
find . -maxdepth 4 -path './.git' -prune -o -type f -name '*.rs' -print | sort
find . -maxdepth 4 -path './.git' -prune -o -type f -name '*.yml' -o -name '*.yaml' -print | sort
```

## 8. Milestones

### M1: Repository inventory

- Goal: Capture current file structure and git state.
- Files to read: root directory, `.gitignore` if present, existing docs.
- Files to change: `.agent/execplans/EP-000-repository-discovery.md` Progress and Surprises.
- Exact edits expected: Add inventory summary to `Surprises & Discoveries`; mark M1 complete.
- Validation command: `./scripts/preflight.sh`
- Expected result: `preflight: ok`; if Cargo project is missing, preflight may still pass while noting EP-001 must create it.
- Recovery instruction: If preflight script is not executable, run `sh scripts/preflight.sh`; then record need to chmod scripts in EP-001.

### M2: Stack and package manager detection

- Goal: Confirm Cargo/Rust or identify actual stack.
- Files to read: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `Makefile`, `justfile`, package manifests if present.
- Files to change: `ASSUMPTIONS.md`, `COMMANDS.md` only if evidence differs.
- Exact edits expected: Confirm or update package manager assumptions and commands.
- Validation command: `cargo --version || true`
- Expected result: Cargo version is printed or absence is recorded as a blocker for EP-001.
- Recovery instruction: If Cargo missing, do not invent another build system. Record STOP for EP-001 unless user installs Rust or approves alternate stack.

### M3: Test, CI, environment, and script detection

- Goal: Find existing tests, CI, env files, and scripts.
- Files to read: `tests/`, `src/`, `.github/workflows/`, `.gitlab-ci.yml`, `.env.example`, `config/`, `scripts/`.
- Files to change: `COMMANDS.md`, `ASSUMPTIONS.md`, EP-000 progress.
- Exact edits expected: Update command references only if existing repository provides better evidence than blueprint scripts.
- Validation command: `git status --short`
- Expected result: Only documentation files changed by discovery.
- Recovery instruction: If unexpected generated files exist, list them and decide whether they are repository baseline or accidental changes before continuing.

### M4: Architecture and risk discovery

- Goal: Compare existing repository reality to `ARCHITECTURE.md` and risk assumptions.
- Files to read: all detected source modules and docs.
- Files to change: `ARCHITECTURE.md`, `ASSUMPTIONS.md`, EP-000 progress.
- Exact edits expected: Add concrete notes if existing repo already has modules, CI, config, or a different structure.
- Validation command: `git diff --name-only`
- Expected result: Changed files are only expected docs and EP-000.
- Recovery instruction: If implementation files changed, revert unrelated changes unless they preexisted; do not proceed with feature work in EP-000.

### M5: Final discovery report

- Goal: Make EP-001 ready with no hidden context.
- Files to read: updated docs.
- Files to change: EP-000 `Outcomes & Retrospective`.
- Exact edits expected: Record discovered stack, commands, missing info, blockers, and recommended next ExecPlan.
- Validation command: `git diff --name-only`
- Expected result: Changed files match expected changed files or extras are justified.
- Recovery instruction: If a required doc update is unclear, record a safe assumption and continue unless STOP condition applies.

## 9. Concrete Steps

1. Run inventory commands from Section 7.
2. Read all discovered manifests and CI files.
3. Confirm or update assumptions.
4. Confirm or update command docs.
5. Confirm or update architecture repo map.
6. Record risks and missing information.
7. Run validation commands.
8. Update Progress, Decisions, and Outcomes.

## 10. Validation and Acceptance

Required final validation:

```sh
./scripts/preflight.sh
git diff --name-only
```

Acceptance criteria:

- Repository structure summarized.
- Package manager status confirmed.
- Test command status confirmed.
- CI status confirmed.
- Environment/secrets status confirmed.
- Risks and missing information recorded.
- Only expected docs changed or extras justified.

## 11. Idempotence and Recovery

Re-running discovery should update the same sections, not duplicate them. If prior notes exist, append dated updates. If commands fail due missing tools, record exact output and whether it blocks EP-001.

## 12. Progress

- [ ] M1 - Repository inventory. Discovery completed 2026-06-23. Validation attempt: `sh scripts/preflight.sh` -> failed because `sh` is unavailable on PATH; `bash.exe` resolves to WSL and reports no installed distributions.
- [ ] M2 - Stack and package manager detection. Cargo and Rust are installed locally (`cargo 1.95.0`, `rustc 1.95.0`), but no `Cargo.toml` exists yet.
- [ ] M3 - Test, CI, environment, and script detection. Repo has docs/scripts only; no `tests/`, `src/`, `.github/workflows/`, or `config/` tree was found.
- [ ] M4 - Architecture and risk discovery. Current checkout is a documentation scaffold, not a Git worktree; `git status --short` and `git branch --show-current` both failed with `not a git repository`.
- [ ] M5 - Final discovery report. Blocked until a POSIX shell is available or EP-001 creates the Rust foundation.

## 13. Surprises & Discoveries

Record discovery output here as dated notes.

- 2026-06-23: `git status --short` and `git branch --show-current` both reported `fatal: not a git repository`, so `C:\dev\OptionClaw` is not a Git checkout in this environment.
- 2026-06-23: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` and `rustc 1.95.0 (59807616e 2026-04-14)` are installed, but no `Cargo.toml` is present.
- 2026-06-23: Repository inventory found docs and scripts only. Top-level files include `AGENTS.md`, `ARCHITECTURE.md`, `ASSUMPTIONS.md`, `COMMANDS.md`, `CONTRIBUTING.md`, `DECISIONS.md`, `DEPLOYMENT.md`, `ENVIRONMENT.md`, `OBSERVABILITY.md`, `OPERATIONS.md`, `PRODUCTION_READINESS.md`, `PROJECT_BRIEF.md`, `RELEASE.md`, `ROADMAP.md`, `ROLLBACK.md`, `SECURITY.md`, `TESTING.md`, `.agent/`, and `scripts/`.
- 2026-06-23: No `src/`, `tests/`, `config/`, `.github/workflows/`, or `Cargo.toml` tree was discovered during inventory.
- 2026-06-23: `./scripts/preflight.sh` could not be executed in this environment because `sh` is unavailable on PATH and `C:\Windows\System32\bash.exe` launches WSL, which reports no installed distributions.
- 2026-06-23: `git diff --name-only` returned `warning: Not a git repository. Use --no-index to compare two paths outside a working tree` followed by the help text; `git status --short` returned `fatal: not a git repository (or any of the parent directories): .git`.

## 14. Decision Log

Record any changed assumptions or command updates here.

- 2026-06-23: Kept `COMMANDS.md` unchanged because the documented commands still match the repository’s intended Rust/Cargo workflow; the failure is environmental rather than a repo command drift.
- 2026-06-23: Treated the missing POSIX shell as an environment blocker for `scripts/preflight.sh`, not as a repository defect. EP-001 should start from the documented foundation path once a runnable shell is available.
- 2026-06-23: Did not update `COMMANDS.md` because discovery did not show a command-contract change, only a machine-specific shell gap.

## 15. Outcomes & Retrospective

Complete this section after M5 with final status, blockers, and recommended next ExecPlan.

- EP-000 discovered that the checkout is a docs-and-scripts scaffold with no Rust project yet.
- EP-000 could not finish the required preflight validation because the machine lacks a usable POSIX shell runner.
- EP-000 could not complete final diff review with Git because the checkout is not a Git repository in this environment.
- Recommended next ExecPlan: `EP-001-foundation.md` after the shell blocker is resolved, because that plan creates the initial Cargo project and CLI scaffold.
