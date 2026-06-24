# Contributing to OptionClaw

## Setup

1. Read `AGENTS.md`.
2. Read `COMMANDS.md`.
3. Run `./scripts/preflight.sh`.
4. Work from one active ExecPlan.
5. Run validation commands listed in the ExecPlan.

## Branch Rules

- Use one branch per ExecPlan or scoped change.
- Do not mix unrelated changes.
- Keep commits reviewable.
- Do not commit secrets, `.env`, local state, target artifacts, or production data.

## Coding Standards

- Rust code must pass `rustfmt`, `clippy`, and `cargo check`.
- Keep domain logic pure.
- Use typed errors and explicit validation.
- Prefer small modules with clear boundaries.
- Do not use `unwrap`/`expect` in production paths unless justified and safe.
- Do not log secrets.
- Keep CLI output readable and non-color-only.

## Test Requirements

- Add tests for every behavior change.
- Unit-test domain and risk logic.
- Integration-test persistence and adapters.
- E2E-test CLI behavior.
- Add regression tests for bugs.
- Use fake secrets and deterministic fixtures.

## Documentation Requirements

Update docs when changing:

- Commands.
- Config/environment variables.
- Architecture boundaries.
- Security behavior.
- Deployment or operations.
- Data schema or migrations.
- User-visible CLI behavior.

## Commit Guidance

Recommended commit message style:

```text
<scope>: <imperative summary>

- What changed
- Tests run
- Risks or follow-up
```

Examples:

- `foundation: add Rust CLI scaffold`
- `risk: add order intent validation`
- `persistence: add audit log schema v1`

## Pull Request Checklist

- [ ] Active ExecPlan named.
- [ ] Scope matches ExecPlan.
- [ ] Tests added/updated.
- [ ] Validation commands pass.
- [ ] Docs updated.
- [ ] No secrets or production data.
- [ ] `git diff --name-only` reviewed.
- [ ] Remaining risks documented.

## Code Review Checklist

- [ ] Source-of-truth hierarchy followed.
- [ ] Domain boundaries preserved.
- [ ] No live trading bypasses risk gates.
- [ ] No provider-specific type leaks.
- [ ] No command/API/config hallucinations.
- [ ] Error handling is typed and actionable.
- [ ] Observability redacts secrets.
- [ ] Tests cover failure modes.

## Agent-Specific Contribution Rules

- Do not ask for next steps unless a STOP condition applies.
- Continue through the active ExecPlan.
- Use commands from `COMMANDS.md`.
- Validate after each milestone.
- Update the ExecPlan as work proceeds.
- Apply bounded retry rules on failures.
- Record assumptions and decisions.
- Final response must list changed files, commands, results, decisions, risks, and acceptance status.
