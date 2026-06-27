use crate::{
    domain::TradingMode,
    errors::CliError,
    persistence::StateReport,
    services::ConfigReport,
    services::{
        ApprovalReport, BacktestReport, HealthReport, LiveCheckReport, LiveSubmitReport,
        PaperExecutionStatus, PaperRunReport, RiskReport,
    },
};

pub const HELP_TEXT: &str = concat!(
    "OptionClaw ",
    env!("CARGO_PKG_VERSION"),
    "\n",
    "\n",
    "Usage:\n",
    "  optionclaw --help\n",
    "  optionclaw --version\n",
    "  optionclaw check-config --config <path>\n",
    "  optionclaw state init --data-dir <path>\n",
    "  optionclaw state verify --data-dir <path>\n",
    "  optionclaw paper run-once --config <path> --fixtures <path>\n",
    "  optionclaw risk explain --config <path> --order-intent <path>\n",
    "  optionclaw health --config <path>\n",
    "  optionclaw research backtest --config <path> --fixtures <path>\n",
    "  optionclaw research approve --config <path> --report <path>\n",
    "  optionclaw live check --config <path>\n",
    "  optionclaw live submit --config <path> --order-intent <path> --confirm-live\n",
    "\n",
    "Commands:\n",
    "  check-config  Validate a config file without contacting external services.\n",
    "  state init    Create the local data directory layout.\n",
    "  state verify  Verify the local data directory layout.\n",
    "  paper run-once  Run one fixture-backed paper workflow.\n",
    "  risk explain  Evaluate a serialized order intent and print the decision.\n",
    "  health        Report local readiness.\n",
    "  research backtest  Produce ROI/drawdown evidence from fixtures.\n",
    "  research approve   Write an internal live-approval artifact.\n",
    "  live check         Verify live-readiness gates without submitting an order.\n",
    "  live submit        Submit a long call/put only after every live gate passes.\n",
    "\n",
    "Safety defaults:\n",
    "  - paper mode is the default.\n",
    "  - Help and validation commands do not call the network.\n",
    "  - Live execution requires env-only Alpaca secrets, explicit enablement, a fresh approval artifact, and --confirm-live.\n",
);

pub fn print_help() {
    println!("{HELP_TEXT}");
}

pub fn print_version() {
    println!("optionclaw {}", env!("CARGO_PKG_VERSION"));
}

pub fn print_check_config(report: &ConfigReport) {
    println!(
        "config ok: mode={} config={}",
        report.trading_mode,
        report.config_path.display()
    );
}

pub fn print_state_init(report: &StateReport) {
    println!(
        "state init ok root={} schema_version={} created={} verified={}",
        report.root.display(),
        report.schema_version,
        report.created,
        report.verified
    );
}

pub fn print_state_verify(report: &StateReport) {
    println!(
        "state verify ok root={} schema_version={} created={} verified={}",
        report.root.display(),
        report.schema_version,
        report.created,
        report.verified
    );
}

pub fn print_paper_run(report: &PaperRunReport) {
    let execution = match report.execution_status {
        PaperExecutionStatus::Executed => "executed",
        PaperExecutionStatus::Rejected => "rejected",
    };

    println!(
        "paper run-once ok mode={} intent={} decision={} execution={} audit_appended={} state_updated={}",
        report.trading_mode,
        report.order_intent_id,
        report.risk_decision,
        execution,
        report.audit_appended,
        report.state_updated
    );
}

pub fn print_risk_report(mode: TradingMode, report: &RiskReport) {
    println!(
        "risk explain ok mode={} intent={} decision={}",
        mode, report.order_intent_id, report.decision
    );
}

pub fn print_health_report(report: &HealthReport) {
    println!(
        "health ok mode={} config_ready={} data_ready={} audit_ready={} secrets_store_ready={} providers_ready={} kill_switch_active={}",
        report.trading_mode,
        report.config_ready,
        report.data_ready,
        report.audit_ready,
        report.secrets_store_ready,
        report.providers_ready,
        report.kill_switch_active
    );
}

pub fn print_backtest_report(report: &BacktestReport) {
    println!(
        "research backtest ok report={} strategy={} annualized_net_roi_bps={} forward_paper_roi_bps={} profit_factor_bps={} max_drawdown_bps={} backtest_trades={} forward_paper_trades={} risk_gate_bypasses={}",
        report.report_path.display(),
        report.evidence.strategy_id,
        report.evidence.annualized_net_roi_bps,
        report.evidence.forward_paper_roi_bps,
        report.evidence.profit_factor_bps,
        report.evidence.max_drawdown_bps,
        report.evidence.backtest_trades,
        report.evidence.forward_paper_trades,
        report.evidence.risk_gate_bypasses
    );
}

pub fn print_approval_report(report: &ApprovalReport) {
    println!(
        "research approve ok approval={} strategy={} expires_at_unix_seconds={} approved={}",
        report.approval_path.display(),
        report.artifact.strategy_id,
        report.artifact.expires_at_unix_seconds,
        report.artifact.approved
    );
}

pub fn print_live_check(report: &LiveCheckReport) {
    println!(
        "live check ok provider={} provider_environment={} account_status={} options_approved_level={} options_trading_level={} approval_fresh={} kill_switch_active={} approved={}",
        report.provider,
        report.provider_environment,
        report.account_status,
        report.options_approved_level,
        report.options_trading_level,
        report.approval_fresh,
        report.kill_switch_active,
        report.approved
    );
}

pub fn print_live_submit(report: &LiveSubmitReport) {
    println!(
        "live submit ok config={} order_intent={} intent={} provider_order_id={} provider_status={} submitted={}",
        report.config_path.display(),
        report.order_intent_path.display(),
        report.order_intent_id,
        report.provider_order_id,
        report.provider_status,
        report.submitted
    );
}

pub fn print_error(error: &CliError) {
    eprintln!("{error}");
}
