use std::process::ExitCode;

use crate::{
    errors::AppError,
    observability::{error_code_from_display, init_logging, record_metric, LogLevel, MetricEvent},
    services::{
        approve_research, check_config, explain_risk, health, init_state, live_check, live_submit,
        run_backtest, run_paper_once, verify_state,
    },
};

mod commands;
mod output;

pub fn run() -> ExitCode {
    init_logging(LogLevel::Info);
    match commands::parse_command() {
        Ok(commands::Command::Help) => {
            output::print_help();
            record_metric(MetricEvent::command_success("help"));
            ExitCode::SUCCESS
        }
        Ok(commands::Command::Version) => {
            output::print_version();
            record_metric(MetricEvent::command_success("version"));
            ExitCode::SUCCESS
        }
        Ok(commands::Command::CheckConfig { config }) => match check_config(&config) {
            Ok(report) => {
                output::print_check_config(&report);
                record_metric(MetricEvent::command_success("check-config"));
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure("check-config", err),
        },
        Ok(commands::Command::StateInit { data_dir }) => match init_state(&data_dir) {
            Ok(report) => {
                output::print_state_init(&report);
                record_metric(MetricEvent::command_success("state init"));
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure("state init", err),
        },
        Ok(commands::Command::StateVerify { data_dir }) => match verify_state(&data_dir) {
            Ok(report) => {
                output::print_state_verify(&report);
                record_metric(MetricEvent::command_success("state verify"));
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure("state verify", err),
        },
        Ok(commands::Command::PaperRunOnce { config, fixtures }) => {
            match run_paper_once(&config, &fixtures) {
                Ok(report) => {
                    output::print_paper_run(&report);
                    record_metric(MetricEvent::command_success("paper run-once"));
                    ExitCode::SUCCESS
                }
                Err(err) => report_cli_failure("paper run-once", err),
            }
        }
        Ok(commands::Command::RiskExplain {
            config,
            order_intent,
        }) => match explain_risk_command(&config, &order_intent) {
            Ok(()) => {
                record_metric(MetricEvent::command_success("risk explain"));
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure("risk explain", err),
        },
        Ok(commands::Command::Health { config }) => match health(&config) {
            Ok(report) => {
                output::print_health_report(&report);
                record_metric(MetricEvent::command_success("health"));
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure("health", err),
        },
        Ok(commands::Command::LiveCheck { config }) => match live_check(&config) {
            Ok(report) => {
                output::print_live_check(&report);
                record_metric(MetricEvent::command_success("live check"));
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure("live check", err),
        },
        Ok(commands::Command::LiveSubmit {
            config,
            order_intent,
            confirm_live,
        }) => match live_submit(&config, &order_intent, confirm_live) {
            Ok(report) => {
                output::print_live_submit(&report);
                record_metric(MetricEvent::command_success("live submit"));
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure("live submit", err),
        },
        Ok(commands::Command::ResearchBacktest { config, fixtures }) => {
            match run_backtest(&config, &fixtures) {
                Ok(report) => {
                    output::print_backtest_report(&report);
                    record_metric(MetricEvent::command_success("research backtest"));
                    ExitCode::SUCCESS
                }
                Err(err) => report_cli_failure("research backtest", err),
            }
        }
        Ok(commands::Command::ResearchApprove { config, report }) => {
            match approve_research(&config, &report) {
                Ok(report) => {
                    output::print_approval_report(&report);
                    record_metric(MetricEvent::command_success("research approve"));
                    ExitCode::SUCCESS
                }
                Err(err) => report_cli_failure("research approve", err),
            }
        }
        Err(err) => {
            output::print_error(&err);
            output::print_help();
            record_metric(MetricEvent::command_failure(
                "cli",
                err.code().as_str().to_string(),
            ));
            ExitCode::from(2)
        }
    }
}

fn explain_risk_command(
    config: &std::path::PathBuf,
    order_intent: &std::path::PathBuf,
) -> Result<(), AppError> {
    let config_report = check_config(config)?;
    let report = explain_risk(order_intent)?;
    output::print_risk_report(config_report.trading_mode, &report);
    Ok(())
}

fn report_cli_failure(command: &str, err: AppError) -> ExitCode {
    record_metric(MetricEvent::command_failure(
        command,
        error_code_from_display(&err),
    ));
    eprintln!("{err}");
    ExitCode::from(1)
}
