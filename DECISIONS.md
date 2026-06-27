# OptionClaw Decision Log

## Decision Table

| ID | Date | Title | Status | Owner | Scope | ADR |
|---|---|---|---|---|---|---|
| D-001 | 2026-06-22 | Rust/Cargo is the default implementation stack | Accepted | Blueprint | Foundation | Inline |
| D-002 | 2026-06-22 | Initial interface is CLI-first | Accepted | Blueprint | Product/UI | Inline |
| D-003 | 2026-06-22 | No external database in initial architecture | Accepted | Blueprint | Persistence | Inline |
| D-004 | 2026-06-22 | Paper trading is default and live trading is gated | Accepted | Blueprint | Safety/Security | Inline |
| D-005 | 2026-06-22 | Provider integrations use traits and fixtures before live adapters | Accepted | Blueprint | Integrations | Inline |
| D-006 | 2026-06-22 | LLM output is advisory and cannot bypass deterministic risk gates | Accepted | Blueprint | Domain/Risk | Inline |
| D-007 | 2026-06-25 | Alpaca is the first live options provider | Accepted | EP-011 | Live execution | Inline |
| D-008 | 2026-06-25 | ROI approval is an expiring internal evidence gate | Accepted | EP-011 | Research/Risk | Inline |

## Initial ADR Entries

### D-001: Rust/Cargo is the default implementation stack

- Context: Project requirements prefer Rust for speed and CLI execution.
- Decision: Use Rust and Cargo as default stack unless repository discovery proves otherwise.
- Alternatives considered: Go, Python, TypeScript, mixed-language stack.
- Consequences: Lower latency and strong type safety; coding agents must create Cargo scripts and Rust tests.
- Status: Accepted.
- Date: 2026-06-22.
- Owner: Blueprint.

### D-002: Initial interface is CLI-first

- Context: Requirements prefer a fast and lean CLI.
- Decision: Build CLI user flows first. Web/mobile UI is out of scope.
- Alternatives considered: Web dashboard, daemon-only service, mobile app.
- Consequences: Accessibility maps to terminal behavior; E2E tests use CLI command execution.
- Status: Accepted.
- Date: 2026-06-22.
- Owner: Blueprint.

### D-003: No external database in initial architecture

- Context: Preferred stack says no database.
- Decision: Use local schema-versioned files and append-only audit logs.
- Alternatives considered: SQLite, Postgres, embedded key-value store.
- Consequences: Simpler local deployment; EP-003 must implement atomic writes, backups, restore, and corruption handling.
- Status: Accepted.
- Date: 2026-06-22.
- Owner: Blueprint.

### D-004: Paper trading is default and live trading is gated

- Context: Autonomous options trading can lose funds or create irreversible external effects.
- Decision: Default all trading to paper mode. Live execution requires explicit config, secrets, risk gates, kill switch, validated adapter, production readiness, and operator approval.
- Alternatives considered: Live-by-default, CLI confirmation only.
- Consequences: Safer development path; users must intentionally enable live mode later.
- Status: Accepted.
- Date: 2026-06-22.
- Owner: Blueprint.

### D-005: Provider integrations use traits and fixtures before live adapters

- Context: Broker/exchange platform is not specified and providers change.
- Decision: Implement provider-neutral traits and fixtures first; add provider-specific adapters only through later ExecPlans.
- Alternatives considered: Hard-code one broker/exchange, use unofficial SDKs.
- Consequences: Less drift and better testability; live integration requires verified official documentation.
- Status: Accepted.
- Date: 2026-06-22.
- Owner: Blueprint.

### D-006: LLM output is advisory and cannot bypass deterministic risk gates

- Context: LLMs can hallucinate and may produce unsafe recommendations.
- Decision: LLM output can enrich reasoning/scoring but every order intent must pass deterministic risk gates after LLM processing.
- Alternatives considered: LLM-only autonomous decisions, human approval for every decision.
- Consequences: Safer autonomy; risk logic must be test-backed and independent from model behavior.
- Status: Accepted.
- Date: 2026-06-22.
- Owner: Blueprint.

### D-007: Alpaca is the first live options provider

- Context: EP-011 targets live-readiness for US-listed equity options and Alpaca documentation supports options trading, sandbox fixtures, market data, and market/limit day orders.
- Decision: Implement Alpaca first behind repository provider traits, with level-2 long calls and long puts only.
- Alternatives considered: Add multiple brokers, use unofficial SDKs, delay provider selection.
- Consequences: Narrower validation surface, mockable HTTP contract tests, and no second broker until Alpaca live readiness is complete.
- Status: Accepted.
- Date: 2026-06-25.
- Owner: EP-011.

### D-008: ROI approval is an expiring internal evidence gate

- Context: The project needs aggressive ROI evidence without promising future returns.
- Decision: Require signed internal approval artifacts with ROI, drawdown, trade-count, profit-factor, zero-bypass, config-hash, and seven-day freshness checks.
- Alternatives considered: Manual approval only, static config flag, or guaranteed-profit language.
- Consequences: Live submit remains blocked when evidence is stale, mismatched, or below thresholds; external broker/legal/profitability approval remains outside the repository.
- Status: Accepted.
- Date: 2026-06-25.
- Owner: EP-011.

## ADR Index

Future ADRs should live under `docs/adr/` unless the repository chooses another documented path. Use `.agent/templates/adr-template.md`.

## Rules for Adding New Decisions

- Add a row to the decision table for every architecture, security, integration, persistence, deployment, or dependency decision that affects future work.
- Record context, decision, alternatives, consequences, status, date, and owner.
- Link to the active ExecPlan and relevant spec.
- Do not hide material decisions only in commit messages.
- Update `ASSUMPTIONS.md` when a decision confirms or invalidates an assumption.
