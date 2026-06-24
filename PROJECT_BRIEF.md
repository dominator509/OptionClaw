# OptionClaw Project Brief

## Project Name

OptionClaw

## Problem Statement

OptionClaw is a local-first Rust command-line system for automated options-trading research, decisioning, simulation, and tightly controlled order execution. The project goal is to build a fast autonomous assistant that can evaluate short-term options opportunities using market data, technical signals, news/rumor/trend inputs, fundamentals, mathematical risk filters, and an LLM-assisted reasoning layer.

OptionClaw must not promise profit, guarantee compounding, or bypass financial, exchange, broker, tax, or jurisdictional requirements. The software must default to paper trading and simulation until live-trading credentials, risk limits, platform contracts, compliance requirements, and kill-switch controls are explicitly implemented and validated.

## Target Users

- Individual users who understand that options trading and crypto derivatives can lose the entire account balance.
- Technical operators running a local CLI on a workstation, Raspberry Pi, VPS, or similar local hardware.
- Developers extending broker/exchange adapters, risk controls, market data ingestion, and LLM decision workflows.

## Primary User Outcomes

- Configure a local automated trading assistant without committing secrets.
- Run reproducible paper-trading simulations before any live order can be placed.
- Enforce risk limits before every strategy recommendation or order submission.
- Observe decisions, errors, logs, and execution status from the CLI.
- Switch between OpenAI-compatible API models and local/self-hosted model endpoints through explicit configuration.

## Business Goals

- Provide a production-oriented foundation for low-latency Rust trading automation.
- Support adapter-based integration with options-capable trading platforms without hard-coding one provider into the domain.
- Make live trading possible only after explicit platform integration, sandbox validation, and security review.
- Keep the repository local-first with no required database service.

## Technical Goals

- Rust-first implementation with deterministic builds, static validation, test coverage, and CI.
- Layered architecture with pure domain logic isolated from infrastructure, broker APIs, wallets, and LLM providers.
- Encrypted local secrets and sensitive configuration.
- Append-only audit trail for decisions, risk checks, and order-intent lifecycle events.
- Fast CLI startup and predictable runtime behavior.

## Out-of-Scope Items

- Guaranteed profit, guaranteed conversion of small funds into six figures, or financial advice.
- Bypassing broker, exchange, KYC, tax, regulatory, or jurisdictional rules.
- Long-term portfolio management or retirement planning.
- Custodial wallet services or storing unencrypted private keys.
- Web UI, mobile app, social trading, copy trading, or multi-tenant SaaS in the initial roadmap.
- Production live trading before EP-010 acceptance criteria pass and explicit operator approval is recorded outside the codebase.

## Success Metrics

Success metrics are implementation and operational metrics, not profit guarantees:

- `scripts/verify.sh` passes on a clean checkout.
- `scripts/production-readiness-check.sh` passes before any production release.
- Paper-trading order lifecycle can run end-to-end with deterministic fixtures.
- Every live order path has a pre-trade risk gate and an emergency kill-switch gate.
- Secrets never appear in logs, git diffs, test fixtures, or panic output.
- Broker/exchange adapters pass contract tests against sandbox or mocked providers.
- Decision audit records contain enough context to explain why an order intent was accepted, rejected, or skipped.

## Production Readiness Definition

OptionClaw is production-ready only when all production-readiness criteria in `PRODUCTION_READINESS.md` and `.agent/specs/SPEC-008-production-readiness.md` pass. At minimum, this means functional acceptance tests, integration tests, security checks, dependency audit, observability, deployment, rollback, incident response, and live-trading safety controls are complete. Live trading is not production-ready while any platform integration, credentials, legal/compliance requirements, or risk limit settings are unresolved.
