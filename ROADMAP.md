# OptionClaw Roadmap

Do not implement directly from this file. Implementation must happen through an ExecPlan.

This roadmap sequences work from discovery to production readiness. Each phase points to specs and ExecPlans. Coding agents must choose one active ExecPlan and complete it before moving to the next.

| Phase | Purpose | Dependencies | Exit Criteria | Linked Specs | Linked ExecPlans |
|---|---|---|---|---|---|
| Phase 0: Repository discovery and foundation | Discover actual repository state, confirm stack, create baseline Rust CLI structure, scripts, and CI. | Blueprint pack present. | `COMMANDS.md`, `ARCHITECTURE.md`, and `ASSUMPTIONS.md` reflect repository evidence; foundation validation passes. | SPEC-000, SPEC-008 | EP-000, EP-001 |
| Phase 1: Core domain | Implement pure domain entities, strategy inputs, order-intent model, trading mode, and deterministic risk primitives. | Phase 0 complete. | Unit tests cover domain invariants and risk rejection paths. | SPEC-001, SPEC-006 | EP-002 |
| Phase 2: Data and persistence | Implement no-database local file persistence, append-only audit log, schema versions, backup, restore, and migration dry-run. | Phase 1 complete. | Integration tests prove state init/verify, atomic writes, audit append, backup/restore, and corrupt file failure. | SPEC-002, SPEC-006 | EP-003 |
| Phase 3: API or service layer | Implement internal service contracts, CLI-facing methods, adapter traits for market data, LLM, paper execution, and future live execution. | Phase 2 complete. | Contract tests cover service methods, mock adapters, request validation, and error mapping. | SPEC-003, SPEC-006 | EP-004 |
| Phase 4: UI or client layer | Implement CLI commands, output states, non-color-only status, readable errors, and acceptance tests. | Phase 3 complete. | CLI E2E tests pass for help, config check, state commands, paper run-once, risk explain, and health. | SPEC-004 | EP-005 |
| Phase 5: Auth, permissions, and security | Implement local security baseline, encrypted secrets, file permissions, redaction, kill switch, and live-mode safeguards. | Phases 1-4 complete. | Security tests pass; live execution remains disabled unless all gates are satisfied. | SPEC-005, SPEC-006 | EP-006 |
| Phase 6: Testing hardening | Raise reliability with regression, failure-mode, contract, and CI tests. | Phases 1-5 complete. | Full verification passes on a clean checkout and CI. | SPEC-001 through SPEC-008 | EP-007 |
| Phase 7: Observability and operations | Add structured logs, metrics, health checks, alerts expectations, and runbooks. | Phase 6 complete. | Smoke tests and health checks produce expected operational signals without leaking secrets. | SPEC-007 | EP-008 |
| Phase 8: Deployment and release | Define release artifact, local hardware/VPS deployment, systemd/container options, rollback, and release checklist. | Phase 7 complete. | Build artifact deploys to staging-like local target; rollback path tested. | SPEC-008 | EP-009 |
| Phase 9: Production readiness | Run final functional, security, performance, privacy, observability, deployment, rollback, and documentation gates. | Phase 8 complete. | `./scripts/production-readiness-check.sh` passes; remaining risks are documented; live trading remains gated until explicit approval. | SPEC-008 | EP-010 |

## Production Readiness Milestone

Production readiness is reached only after Phase 9 exit criteria pass. For OptionClaw, production readiness does not imply profitability or financial suitability. It means the software is functionally implemented, test-backed, observable, secure, deployable, rollback-capable, and operationally documented for its configured mode.
