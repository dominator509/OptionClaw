use crate::{
    domain::TradingMode,
    errors::CliError,
    persistence::StateReport,
    services::ConfigReport,
    services::{HealthReport, PaperExecutionStatus, PaperRunReport, RiskReport},
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
    "\n",
    "Commands:\n",
    "  check-config  Validate a config file without contacting external services.\n",
    "  state init    Create the local data directory layout.\n",
    "  state verify  Verify the local data directory layout.\n",
    "  paper run-once  Run one fixture-backed paper workflow.\n",
    "  risk explain  Evaluate a serialized order intent and print the decision.\n",
    "  health        Report local readiness.\n",
    "\n",
    "Safety defaults:\n",
    "  - paper mode is the default.\n",
    "  - Help and validation commands do not call the network.\n",
    "  - Live execution remains disabled until a later ExecPlan enables it.\n",
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

pub fn print_error(error: &CliError) {
    eprintln!("{error}");
}
