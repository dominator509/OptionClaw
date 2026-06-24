use std::{path::PathBuf, process::Command};

use optionclaw::config::{AppConfig, TradingMode};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_optionclaw"))
}

#[test]
fn loads_example_config_with_paper_mode() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("config/example.toml");

    let config = AppConfig::load_from_path(&path).expect("example config should parse");

    assert_eq!(config.trading_mode, TradingMode::Paper);
}

#[test]
fn health_command_reports_readiness() {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("config/example.toml");

    let output = binary()
        .args(["health", "--config"])
        .arg(&config_path)
        .output()
        .expect("binary should run");

    assert!(output.status.success(), "health should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("health ok"));
    assert!(stdout.contains("secrets_store_ready=true"));
    assert!(stdout.contains("providers_ready=true"));
}
