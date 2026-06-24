# Preflight Checklist

Run before editing:

- [ ] From repository root, run `pwd` and confirm path is correct.
- [ ] Run `git status --short` and note existing changes.
- [ ] Run `./scripts/preflight.sh` and expect `preflight: ok`.
- [ ] Confirm `AGENTS.md`, `COMMANDS.md`, `.agent/PLANS.md`, and active ExecPlan exist.
- [ ] Confirm `cargo --version` if Rust implementation is active.
- [ ] Confirm `Cargo.toml` exists after EP-001.
- [ ] Confirm scripts exist under `scripts/`.
- [ ] Confirm test harness exists after EP-001.
- [ ] Confirm no required secrets are needed for paper/mock tasks.
- [ ] Confirm no local service is required unless the active ExecPlan says so.
- [ ] Record known blockers in the active ExecPlan.
