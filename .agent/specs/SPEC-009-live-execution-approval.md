# SPEC-009 Live Execution Approval

## Purpose

OptionClaw may approve live execution only through repository-controlled software gates. Internal approval proves that configured software, risk limits, provider capability checks, and ROI evidence meet the configured policy. It does not guarantee profit, broker approval, tax/legal suitability, or future performance.

## Provider Scope

The first live provider is Alpaca. The first live options scope is level-2 long calls and long puts. The adapter must support account/status capability checks, option contract lookup, order preview, order submit, order cancel, and order status polling. Tests must use mocks or sandbox fixtures only.

## Config and Environment Contract

Live mode requires:

- `trading_mode = "live"`
- `provider = "alpaca"`
- `provider_environment = "paper" | "sandbox" | "live"`
- `OPTIONCLAW_ENABLE_LIVE_TRADING=true`
- `OPTIONCLAW_ALPACA_API_KEY`
- `OPTIONCLAW_ALPACA_API_SECRET`
- `max_account_risk_bps`
- `max_daily_loss_bps`
- `max_contracts_per_order`
- `kill_switch_file`
- `approval_artifact`

`alpaca_base_url` may be configured only for tests, mocks, sandbox fixtures, or explicitly documented local verification. Raw broker credentials must never be written to disk, logs, audit records, fixtures, reports, or approval artifacts.

## ROI Evidence Gate

Aggressive ROI approval requires a fresh evidence artifact with all of these thresholds:

- Annualized net ROI after fees/slippage at least 25%.
- Forward-paper ROI at least 8% over the configured trial window.
- Profit factor at least 1.35.
- Max drawdown no greater than 20%.
- At least 200 backtest trades.
- At least 30 forward-paper trades.
- Zero risk-gate bypasses.

Evidence must include strategy ID, risk profile ID, config hash, generation time, and approval status. Approval artifacts expire after seven days and must match the current strategy/risk config hash.

## Live Check Gate

`optionclaw live check --config <path>` must verify:

- Live mode config contract.
- Required env-only Alpaca credentials and explicit enablement.
- Risk caps are configured and valid.
- Kill switch is inactive.
- Approval artifact is present, fresh, approved, and hash-matched.
- Alpaca account is active, not trading-blocked, and options approved/trading level is at least 2.

If provider/account status cannot confirm options capability at runtime, live check fails closed.

## Live Submit Gate

`optionclaw live submit --config <path> --order-intent <path> --confirm-live` must verify all live check gates and additionally:

- `--confirm-live` is present.
- Order intent mode is live.
- Order intent is a buy-side long call or long put.
- Configured risk limits accept the order.
- Audit append succeeds after mock/provider submit response.

The command must submit at most once for a single CLI invocation. It must not print secrets or full credential-bearing payloads.

## Non-goals

- Guaranteed profit or guaranteed ROI.
- Broker/KYC/options approval guarantees.
- Legal, tax, regulatory, or custody advice.
- Covered calls, cash-secured puts, spreads, exercise/DNE automation, margin expansion, crypto-funded workflows, private-key custody, or a second broker adapter.
- Real live order submission from tests or implementation validation.

## Acceptance

Acceptance requires unit, contract, integration, and E2E tests for positive and negative gates, redacted output, dependency audit, security check coverage, and documentation updates. Live production operation remains blocked until the operator supplies real credentials and explicitly enables live trading at runtime after all readiness criteria pass.
