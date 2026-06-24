use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_optionclaw"))
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be on or after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("optionclaw-ep005-{name}-{nanos}"));
    if path.exists() {
        let _ = fs::remove_dir_all(&path);
    }
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn write_config(root: &Path, trading_mode: &str) -> PathBuf {
    let config_dir = root.join("config");
    fs::create_dir_all(&config_dir).expect("config dir should be creatable");
    let config_path = config_dir.join("example.toml");
    fs::write(&config_path, format!("trading_mode = \"{trading_mode}\""))
        .expect("config file should be writable");
    config_path
}

fn repo_fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

fn invalid_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("config")
        .join("invalid_config.toml")
}

fn order_intent_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("orders")
        .join("sample_order_intent.json")
}

fn assert_plain_text(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stdout.contains('\u{1b}'),
        "stdout should not require color"
    );
    assert!(
        !stderr.contains('\u{1b}'),
        "stderr should not require color"
    );
}

#[test]
fn help_lists_required_commands_and_defaults() {
    let output = binary().arg("--help").output().expect("binary should run");

    assert!(output.status.success(), "help should exit successfully");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("check-config"));
    assert!(stdout.contains("state init"));
    assert!(stdout.contains("state verify"));
    assert!(stdout.contains("paper run-once"));
    assert!(stdout.contains("risk explain"));
    assert!(stdout.contains("health"));
    assert!(stdout.contains("paper mode is the default"));
    assert_plain_text(&output);
}

#[test]
fn version_reports_binary_version() {
    let output = binary()
        .arg("--version")
        .output()
        .expect("binary should run");

    assert!(output.status.success(), "version should exit successfully");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        format!("optionclaw {}", env!("CARGO_PKG_VERSION"))
    );
    assert_plain_text(&output);
}

#[test]
fn check_config_reports_mode_and_rejects_invalid_files() {
    let success = binary()
        .args(["check-config", "--config"])
        .arg(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("config")
                .join("example.toml"),
        )
        .output()
        .expect("binary should run");

    assert!(success.status.success(), "check-config should succeed");
    let stdout = String::from_utf8_lossy(&success.stdout);
    assert!(stdout.contains("config ok: mode=paper"));
    assert_plain_text(&success);

    let failure = binary()
        .args(["check-config", "--config"])
        .arg(invalid_config_path())
        .output()
        .expect("binary should run");

    assert!(!failure.status.success(), "invalid config should fail");
    let stderr = String::from_utf8_lossy(&failure.stderr);
    assert!(stderr.contains("E1002"));
    assert!(stderr.contains("start from config/example.toml"));
    assert_plain_text(&failure);
}

#[test]
fn check_config_refuses_live_mode_without_gates() {
    let root = unique_temp_dir("live-config");
    let config_path = write_config(&root, "live");

    let output = binary()
        .args(["check-config", "--config"])
        .arg(&config_path)
        .output()
        .expect("binary should run");

    assert!(!output.status.success(), "live config should fail closed");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("LIVE_TRADING_DISABLED"));
    assert!(stderr.contains("paper mode until production approval is complete"));
    assert_plain_text(&output);
}

#[test]
fn state_init_and_verify_report_readiness() {
    let root = unique_temp_dir("state");
    let data_dir = root.join("var").join("dev");

    let init = binary()
        .args(["state", "init", "--data-dir"])
        .arg(&data_dir)
        .output()
        .expect("binary should run");
    assert!(init.status.success(), "state init should succeed");
    let init_stdout = String::from_utf8_lossy(&init.stdout);
    assert!(init_stdout.contains("state init ok"));
    assert!(init_stdout.contains("created=true"));
    assert_plain_text(&init);

    let verify = binary()
        .args(["state", "verify", "--data-dir"])
        .arg(&data_dir)
        .output()
        .expect("binary should run");
    assert!(verify.status.success(), "state verify should succeed");
    let verify_stdout = String::from_utf8_lossy(&verify.stdout);
    assert!(verify_stdout.contains("state verify ok"));
    assert!(verify_stdout.contains("verified=true"));
    assert_plain_text(&verify);
}

#[test]
fn paper_run_once_executes_and_updates_state() {
    let root = unique_temp_dir("paper");
    let config_path = write_config(&root, "paper");
    let fixture_root = repo_fixture_root();

    let output = binary()
        .args(["paper", "run-once", "--config"])
        .arg(&config_path)
        .args(["--fixtures"])
        .arg(&fixture_root)
        .output()
        .expect("binary should run");

    assert!(output.status.success(), "paper run-once should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("paper run-once ok"));
    assert!(stdout.contains("mode=paper"));
    assert!(stdout.contains("decision=RISK_ACCEPTED"));
    assert!(stdout.contains("execution=executed"));
    assert_plain_text(&output);

    let data_dir = root.join("var").join("dev");
    assert!(data_dir.join("paper").join("state.json").exists());
    assert!(data_dir.join("audit").join("events.jsonl").exists());
}

#[test]
fn paper_run_once_refuses_live_mode_safely() {
    let root = unique_temp_dir("paper-live");
    let config_path = write_config(&root, "live");
    let fixture_root = repo_fixture_root();

    let output = binary()
        .args(["paper", "run-once", "--config"])
        .arg(&config_path)
        .args(["--fixtures"])
        .arg(&fixture_root)
        .output()
        .expect("binary should run");

    assert!(!output.status.success(), "live mode should be refused");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("LIVE_TRADING_DISABLED"));
    assert!(stderr.contains("live trading is disabled for `live`"));
    assert_plain_text(&output);
}

#[test]
fn risk_explain_and_health_report_mode_and_readiness() {
    let root = unique_temp_dir("risk-health");
    let config_path = write_config(&root, "paper");
    let data_dir = root.join("var").join("dev");

    let state_init = binary()
        .args(["state", "init", "--data-dir"])
        .arg(&data_dir)
        .output()
        .expect("binary should run");
    assert!(state_init.status.success(), "state init should succeed");

    let risk_output = binary()
        .args(["risk", "explain", "--config"])
        .arg(&config_path)
        .args(["--order-intent"])
        .arg(order_intent_path())
        .output()
        .expect("binary should run");

    assert!(risk_output.status.success(), "risk explain should succeed");
    let risk_stdout = String::from_utf8_lossy(&risk_output.stdout);
    assert!(risk_stdout.contains("risk explain ok"));
    assert!(risk_stdout.contains("mode=paper"));
    assert!(risk_stdout.contains("decision=RISK_ACCEPTED"));
    assert_plain_text(&risk_output);

    let health_output = binary()
        .args(["health", "--config"])
        .arg(&config_path)
        .output()
        .expect("binary should run");

    assert!(health_output.status.success(), "health should succeed");
    let health_stdout = String::from_utf8_lossy(&health_output.stdout);
    assert!(health_stdout.contains("health ok"));
    assert!(health_stdout.contains("config_ready=true"));
    assert!(health_stdout.contains("data_ready=true"));
    assert!(health_stdout.contains("audit_ready=true"));
    assert!(health_stdout.contains("secrets_store_ready=true"));
    assert!(health_stdout.contains("providers_ready=true"));
    assert_plain_text(&health_output);
}
