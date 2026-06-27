use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

use optionclaw::services::{approve_research, run_backtest};
use serde_json::{json, Value};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_optionclaw"))
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be on or after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("optionclaw-ep011-e2e-{name}-{nanos}"));
    if path.exists() {
        let _ = fs::remove_dir_all(&path);
    }
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn toml_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn write_mode_config(root: &Path, mode: &str) -> PathBuf {
    let config_dir = root.join("config");
    fs::create_dir_all(&config_dir).expect("config dir should be creatable");
    let config_path = config_dir.join(format!("{mode}.toml"));
    fs::write(&config_path, format!("trading_mode = \"{mode}\"")).expect("config should write");
    config_path
}

fn write_live_config(root: &Path) -> (PathBuf, PathBuf) {
    let config_dir = root.join("config");
    fs::create_dir_all(&config_dir).expect("config dir should be creatable");
    let approval_path = root
        .join("var")
        .join("dev")
        .join("live")
        .join("approval.json");
    let kill_switch_path = root
        .join("var")
        .join("dev")
        .join("live")
        .join("KILL_SWITCH");
    let config_path = config_dir.join("live.toml");
    fs::write(
        &config_path,
        format!(
            r#"
trading_mode = "live"
provider = "alpaca"
provider_environment = "sandbox"
strategy_id = "aggressive-growth-v1"
risk_profile_id = "aggressive-growth-risk-v1"
max_account_risk_bps = 100
max_daily_loss_bps = 300
max_contracts_per_order = 1
kill_switch_file = "{}"
approval_artifact = "{}"
alpaca_base_url = "http://127.0.0.1:9"
"#,
            toml_path(&kill_switch_path),
            toml_path(&approval_path)
        ),
    )
    .expect("config should write");
    (config_path, approval_path)
}

fn write_research_fixture(root: &Path) -> PathBuf {
    let path = root.join("research.json");
    fs::write(
        &path,
        serde_json::to_string_pretty(&json!({
            "annualized_net_roi_bps": 2500,
            "forward_paper_roi_bps": 800,
            "profit_factor_bps": 13500,
            "max_drawdown_bps": 2000,
            "backtest_trades": 200,
            "forward_paper_trades": 30,
            "risk_gate_bypasses": 0
        }))
        .expect("fixture should serialize"),
    )
    .expect("fixture should write");
    path
}

fn write_live_order(root: &Path) -> PathBuf {
    let path = root.join("long-call.json");
    fs::write(
        &path,
        serde_json::to_string_pretty(&json!({
            "id": "live-intent-1",
            "mode": "live",
            "contract": {
                "symbol": "AAPL260201C00100000",
                "underlying": "AAPL",
                "expiration": { "year": 2026, "month": 2, "day": 1 },
                "strike_micros": 100000000,
                "kind": "Call",
                "venue_id": null,
                "as_of": { "year": 2026, "month": 1, "day": 1 }
            },
            "side": "Buy",
            "quantity": 1,
            "order_type": "Limit",
            "limit_price_micros": 1250000,
            "estimated_max_loss_micros": 1250000,
            "strategy_id": "aggressive-growth-v1",
            "risk_context_id": "aggressive-growth-risk-v1",
            "created_at_unix_seconds": 1704067201
        }))
        .expect("order should serialize"),
    )
    .expect("order should write");
    path
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
fn live_submit_refuses_without_confirm_live() {
    let root = unique_temp_dir("no-confirm");
    let config_path = write_mode_config(&root, "paper");
    let order_path = root.join("missing-order.json");

    let output = binary()
        .args(["live", "submit", "--config"])
        .arg(&config_path)
        .args(["--order-intent"])
        .arg(&order_path)
        .output()
        .expect("binary should run");

    assert!(!output.status.success(), "live submit should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--confirm-live"));
    assert_plain_text(&output);
}

#[test]
fn live_submit_refuses_paper_and_sandbox_modes() {
    for mode in ["paper", "sandbox"] {
        let root = unique_temp_dir(mode);
        let config_path = write_mode_config(&root, mode);
        let order_path = root.join("missing-order.json");

        let output = binary()
            .args(["live", "submit", "--config"])
            .arg(&config_path)
            .args(["--order-intent"])
            .arg(&order_path)
            .arg("--confirm-live")
            .output()
            .expect("binary should run");

        assert!(!output.status.success(), "{mode} mode should fail");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("trading_mode = \"live\""));
        assert_plain_text(&output);
    }
}

#[test]
fn live_submit_refuses_stale_roi_evidence_and_redacts_env_secrets() {
    let root = unique_temp_dir("stale");
    let (config_path, approval_path) = write_live_config(&root);
    let report = run_backtest(&config_path, write_research_fixture(&root))
        .expect("backtest should produce evidence");
    approve_research(&config_path, &report.report_path).expect("approval should write");
    let mut artifact: Value = serde_json::from_str(
        &fs::read_to_string(&approval_path).expect("approval should be readable"),
    )
    .expect("approval should parse");
    artifact["expires_at_unix_seconds"] = json!(1);
    fs::write(
        &approval_path,
        serde_json::to_string_pretty(&artifact).expect("artifact should serialize"),
    )
    .expect("artifact should write");
    let order_path = write_live_order(&root);

    let output = binary()
        .env("OPTIONCLAW_ENABLE_LIVE_TRADING", "true")
        .env("OPTIONCLAW_ALPACA_API_KEY", "super-secret-key")
        .env("OPTIONCLAW_ALPACA_API_SECRET", "super-secret-secret")
        .args(["live", "submit", "--config"])
        .arg(&config_path)
        .args(["--order-intent"])
        .arg(&order_path)
        .arg("--confirm-live")
        .output()
        .expect("binary should run");

    assert!(!output.status.success(), "stale ROI evidence should fail");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("stale"));
    assert!(!stdout.contains("super-secret-key"));
    assert!(!stdout.contains("super-secret-secret"));
    assert!(!stderr.contains("super-secret-key"));
    assert!(!stderr.contains("super-secret-secret"));
    assert_plain_text(&output);
}
