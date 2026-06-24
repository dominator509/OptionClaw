use std::process::ExitCode;

use crate::{
    errors::AppError,
    services::{check_config, explain_risk, health, init_state, run_paper_once, verify_state},
};

mod commands;
mod output;

pub fn run() -> ExitCode {
    match commands::parse_command() {
        Ok(commands::Command::Help) => {
            output::print_help();
            ExitCode::SUCCESS
        }
        Ok(commands::Command::Version) => {
            output::print_version();
            ExitCode::SUCCESS
        }
        Ok(commands::Command::CheckConfig { config }) => match check_config(&config) {
            Ok(report) => {
                output::print_check_config(&report);
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure(err),
        },
        Ok(commands::Command::StateInit { data_dir }) => match init_state(&data_dir) {
            Ok(report) => {
                output::print_state_init(&report);
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure(err),
        },
        Ok(commands::Command::StateVerify { data_dir }) => match verify_state(&data_dir) {
            Ok(report) => {
                output::print_state_verify(&report);
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure(err),
        },
        Ok(commands::Command::PaperRunOnce { config, fixtures }) => {
            match run_paper_once(&config, &fixtures) {
                Ok(report) => {
                    output::print_paper_run(&report);
                    ExitCode::SUCCESS
                }
                Err(err) => report_cli_failure(err),
            }
        }
        Ok(commands::Command::RiskExplain {
            config,
            order_intent,
        }) => match explain_risk_command(&config, &order_intent) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => report_cli_failure(err),
        },
        Ok(commands::Command::Health { config }) => match health(&config) {
            Ok(report) => {
                output::print_health_report(&report);
                ExitCode::SUCCESS
            }
            Err(err) => report_cli_failure(err),
        },
        Err(err) => {
            output::print_error(&err);
            output::print_help();
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

fn report_cli_failure(err: AppError) -> ExitCode {
    eprintln!("{err}");
    ExitCode::from(1)
}
