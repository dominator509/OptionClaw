use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use optionclaw::services::{
    check_config, explain_risk, health, init_state, run_paper_once, verify_state,
    PaperExecutionStatus,
};
use serde_json::json;

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be on or after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("optionclaw-ep004-{name}-{nanos}"));
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

fn copy_fixture_tree(root: &Path) -> PathBuf {
    let fixture_root = root.join("fixtures");
    fs::create_dir_all(fixture_root.join("market"))
        .expect("market fixture dir should be creatable");
    fs::create_dir_all(fixture_root.join("llm")).expect("llm fixture dir should be creatable");
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("market")
            .join("sample_snapshot.json"),
        fixture_root.join("market").join("sample_snapshot.json"),
    )
    .expect("market fixture should copy");
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("llm")
            .join("sample_advisory.json"),
        fixture_root.join("llm").join("sample_advisory.json"),
    )
    .expect("llm fixture should copy");
    fixture_root
}

#[test]
fn config_and_state_services_succeed() {
    let root = unique_temp_dir("service-config");
    let config_path = write_config(&root, "paper");
    let data_dir = root.join("var").join("dev");

    let config_report = check_config(&config_path).expect("config should validate");
    assert_eq!(config_report.trading_mode.to_string(), "paper");

    let init_report = init_state(&data_dir).expect("state init should succeed");
    assert!(init_report.created);
    let verify_report = verify_state(&data_dir).expect("state verify should succeed");
    assert!(verify_report.verified);

    let health_report = health(&config_path).expect("health should succeed");
    assert!(health_report.config_ready);
    assert!(health_report.data_ready);
    assert!(health_report.audit_ready);
    assert!(health_report.mock_providers_ready);
    assert!(!health_report.kill_switch_active);
}

#[test]
fn paper_run_once_records_audit_and_updates_state() {
    let root = unique_temp_dir("paper-success");
    let config_path = write_config(&root, "paper");
    let fixture_root = copy_fixture_tree(&root);

    let report = run_paper_once(&config_path, &fixture_root).expect("paper run should succeed");
    assert_eq!(report.execution_status, PaperExecutionStatus::Executed);
    assert!(report.state_updated);
    assert!(matches!(
        report.trading_mode,
        optionclaw::domain::TradingMode::Paper
    ));

    let data_dir = root.join("var").join("dev");
    let state_contents = fs::read_to_string(data_dir.join("paper").join("state.json"))
        .expect("state file should be readable");
    assert!(state_contents.contains("paper"));

    let audit_contents = fs::read_to_string(data_dir.join("audit").join("events.jsonl"))
        .expect("audit file should be readable");
    assert!(audit_contents.contains("PAPER_EXECUTED"));
}

#[test]
fn paper_run_once_rejects_risky_fixture() {
    let root = unique_temp_dir("paper-reject");
    let config_path = write_config(&root, "paper");
    let fixture_root = copy_fixture_tree(&root);

    let snapshot_path = fixture_root.join("market").join("sample_snapshot.json");
    let mut snapshot: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&snapshot_path).expect("snapshot should be readable"),
    )
    .expect("snapshot JSON should parse");
    snapshot["market_snapshot"]["option_ask_micros"] = json!(2_500_000);
    fs::write(
        &snapshot_path,
        serde_json::to_string_pretty(&snapshot).expect("snapshot should serialize"),
    )
    .expect("snapshot should write");

    let report = run_paper_once(&config_path, &fixture_root).expect("paper run should return");
    assert_eq!(report.execution_status, PaperExecutionStatus::Rejected);
    assert!(!report.state_updated);
}

#[test]
fn risk_explain_accepts_serialized_fixture() {
    let root = unique_temp_dir("risk");
    let explain_path = root.join("risk-explain.json");
    let fixture = json!({
        "order_intent": {
            "id": "intent-1",
            "mode": "paper",
            "contract": {
                "symbol": "AAPL260201C00100000",
                "underlying": "AAPL",
                "expiration": { "year": 2026, "month": 2, "day": 1 },
                "strike_micros": 1000000,
                "kind": "Call",
                "venue_id": null,
                "as_of": { "year": 2026, "month": 1, "day": 1 }
            },
            "side": "Buy",
            "quantity": 1,
            "order_type": "Limit",
            "limit_price_micros": 5000,
            "estimated_max_loss_micros": 5000,
            "strategy_id": "strategy-1",
            "risk_context_id": "risk-1",
            "created_at_unix_seconds": 1704067201
        },
        "account": {
            "account_id": "acct-1",
            "equity_micros": 10000000,
            "available_cash_micros": 7000000,
            "daily_loss_bps": 0
        },
        "limits": {
            "max_account_risk_bps": 100,
            "max_daily_loss_bps": 100,
            "max_contracts_per_order": 1,
            "allow_live": false
        },
        "kill_switch_active": false
    });
    fs::write(
        &explain_path,
        serde_json::to_string_pretty(&fixture).expect("risk fixture should serialize"),
    )
    .expect("risk fixture should write");

    let report = explain_risk(&explain_path).expect("risk explain should succeed");
    assert!(report.decision.is_accepted());
}

#[test]
fn health_reflects_missing_data_dir_when_not_initialized() {
    let root = unique_temp_dir("health-empty");
    let config_path = write_config(&root, "paper");

    let report = health(&config_path).expect("health should still parse config");
    assert!(report.config_ready);
    assert!(!report.data_ready);
    assert!(!report.audit_ready);
}
