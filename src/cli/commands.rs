use std::{
    env,
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use crate::errors::CliError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
    CheckConfig {
        config: PathBuf,
    },
    StateInit {
        data_dir: PathBuf,
    },
    StateVerify {
        data_dir: PathBuf,
    },
    PaperRunOnce {
        config: PathBuf,
        fixtures: PathBuf,
    },
    RiskExplain {
        config: PathBuf,
        order_intent: PathBuf,
    },
    Health {
        config: PathBuf,
    },
}

pub fn parse_command() -> Result<Command, CliError> {
    let mut args = env::args_os().skip(1);
    match args.next() {
        None => Ok(Command::Help),
        Some(flag) if is_help_flag(&flag) => Ok(Command::Help),
        Some(flag) if is_version_flag(&flag) => Ok(Command::Version),
        Some(command) if command == "check-config" => parse_check_config(args),
        Some(command) if command == "state" => parse_state(args),
        Some(command) if command == "paper" => parse_paper(args),
        Some(command) if command == "risk" => parse_risk(args),
        Some(command) if command == "health" => parse_health(args),
        Some(other) => Err(CliError::UnknownCommand {
            command: other.to_string_lossy().to_string(),
        }),
    }
}

fn parse_check_config(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(first) = args.next() else {
        return Err(CliError::MissingArgument {
            command: "check-config",
            argument: "--config <path>",
            hint: "run `optionclaw check-config --help`.",
        });
    };
    if is_help_flag(&first) {
        return Ok(Command::Help);
    }

    if first != "--config" {
        return Err(CliError::UnexpectedArgument {
            command: "check-config",
            argument: first.to_string_lossy().to_string(),
            hint: "use `--config <path>`.",
        });
    }

    let Some(path) = args.next() else {
        return Err(CliError::MissingArgument {
            command: "check-config",
            argument: "--config <path>",
            hint: "run `optionclaw check-config --help`.",
        });
    };

    if let Some(extra) = args.next() {
        return Err(CliError::UnexpectedArgument {
            command: "check-config",
            argument: extra.to_string_lossy().to_string(),
            hint: "remove the extra argument and try again.",
        });
    }

    Ok(Command::CheckConfig {
        config: PathBuf::from(path),
    })
}

fn parse_state(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(subcommand) = args.next() else {
        return Err(CliError::MissingArgument {
            command: "state",
            argument: "<init|verify>",
            hint: "run `optionclaw state --help`.",
        });
    };
    if is_help_flag(&subcommand) {
        return Ok(Command::Help);
    }
    if subcommand == "init" {
        return parse_state_init(args);
    }
    if subcommand == "verify" {
        return parse_state_verify(args);
    }
    Err(CliError::UnknownSubcommand {
        command: "state",
        subcommand: subcommand.to_string_lossy().to_string(),
    })
}

fn parse_state_init(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(first) = args.next() else {
        return Err(missing_path("state init", "--data-dir <path>"));
    };
    if is_help_flag(&first) {
        return Ok(Command::Help);
    }

    if first != "--data-dir" {
        return Err(unexpected_arg("state init", first));
    }

    let Some(data_dir) = args.next() else {
        return Err(missing_path("state init", "--data-dir <path>"));
    };
    expect_no_more_args("state init", args).map(|_| Command::StateInit {
        data_dir: PathBuf::from(data_dir),
    })
}

fn parse_state_verify(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(first) = args.next() else {
        return Err(missing_path("state verify", "--data-dir <path>"));
    };
    if is_help_flag(&first) {
        return Ok(Command::Help);
    }

    if first != "--data-dir" {
        return Err(unexpected_arg("state verify", first));
    }

    let Some(data_dir) = args.next() else {
        return Err(missing_path("state verify", "--data-dir <path>"));
    };
    expect_no_more_args("state verify", args).map(|_| Command::StateVerify {
        data_dir: PathBuf::from(data_dir),
    })
}

fn parse_paper(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(subcommand) = args.next() else {
        return Err(CliError::MissingArgument {
            command: "paper",
            argument: "<run-once>",
            hint: "run `optionclaw paper --help`.",
        });
    };
    if is_help_flag(&subcommand) {
        return Ok(Command::Help);
    }
    if subcommand == "run-once" {
        return parse_paper_run_once(args);
    }
    Err(CliError::UnknownSubcommand {
        command: "paper",
        subcommand: subcommand.to_string_lossy().to_string(),
    })
}

fn parse_paper_run_once(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(first) = args.next() else {
        return Err(missing_path("paper run-once", "--config <path>"));
    };
    if is_help_flag(&first) {
        return Ok(Command::Help);
    }

    if first != "--config" {
        return Err(unexpected_arg("paper run-once", first));
    }

    let Some(config) = args.next() else {
        return Err(missing_path("paper run-once", "--config <path>"));
    };

    let Some(fixtures_flag) = args.next() else {
        return Err(missing_path("paper run-once", "--fixtures <path>"));
    };
    if fixtures_flag != "--fixtures" {
        return Err(unexpected_arg("paper run-once", fixtures_flag));
    }

    let Some(fixtures) = args.next() else {
        return Err(missing_path("paper run-once", "--fixtures <path>"));
    };

    expect_no_more_args("paper run-once", args).map(|_| Command::PaperRunOnce {
        config: PathBuf::from(config),
        fixtures: PathBuf::from(fixtures),
    })
}

fn parse_risk(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(subcommand) = args.next() else {
        return Err(CliError::MissingArgument {
            command: "risk",
            argument: "<explain>",
            hint: "run `optionclaw risk --help`.",
        });
    };
    if is_help_flag(&subcommand) {
        return Ok(Command::Help);
    }
    if subcommand == "explain" {
        return parse_risk_explain(args);
    }
    Err(CliError::UnknownSubcommand {
        command: "risk",
        subcommand: subcommand.to_string_lossy().to_string(),
    })
}

fn parse_risk_explain(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(first) = args.next() else {
        return Err(missing_path("risk explain", "--config <path>"));
    };
    if is_help_flag(&first) {
        return Ok(Command::Help);
    }

    if first != "--config" {
        return Err(unexpected_arg("risk explain", first));
    }

    let Some(config) = args.next() else {
        return Err(missing_path("risk explain", "--config <path>"));
    };

    let Some(order_flag) = args.next() else {
        return Err(missing_path("risk explain", "--order-intent <path>"));
    };
    if order_flag != "--order-intent" {
        return Err(unexpected_arg("risk explain", order_flag));
    }

    let Some(order_intent) = args.next() else {
        return Err(missing_path("risk explain", "--order-intent <path>"));
    };

    expect_no_more_args("risk explain", args).map(|_| Command::RiskExplain {
        config: PathBuf::from(config),
        order_intent: PathBuf::from(order_intent),
    })
}

fn parse_health(mut args: impl Iterator<Item = OsString>) -> Result<Command, CliError> {
    let Some(first) = args.next() else {
        return Err(missing_path("health", "--config <path>"));
    };
    if is_help_flag(&first) {
        return Ok(Command::Help);
    }

    if first != "--config" {
        return Err(unexpected_arg("health", first));
    }

    let Some(config) = args.next() else {
        return Err(missing_path("health", "--config <path>"));
    };

    expect_no_more_args("health", args).map(|_| Command::Health {
        config: PathBuf::from(config),
    })
}

fn expect_no_more_args(
    command: &'static str,
    mut args: impl Iterator<Item = OsString>,
) -> Result<(), CliError> {
    if let Some(extra) = args.next() {
        return Err(CliError::UnexpectedArgument {
            command,
            argument: extra.to_string_lossy().to_string(),
            hint: "remove the extra argument and try again.",
        });
    }
    Ok(())
}

fn missing_path(command: &'static str, argument: &'static str) -> CliError {
    CliError::MissingArgument {
        command,
        argument,
        hint: "run the command with --help to see the required form.",
    }
}

fn unexpected_arg(command: &'static str, argument: OsString) -> CliError {
    CliError::UnexpectedArgument {
        command,
        argument: argument.to_string_lossy().to_string(),
        hint: "run the command with --help to see the required form.",
    }
}

fn is_help_flag(flag: &OsStr) -> bool {
    flag == "--help" || flag == "-h"
}

fn is_version_flag(flag: &OsStr) -> bool {
    flag == "--version" || flag == "-V"
}
