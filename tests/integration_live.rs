use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use httpmock::prelude::*;
use httpmock::Mock;
use optionclaw::services::{approve_research, live_check, live_submit, run_backtest};
use serde_json::{json, Value};

static ENV_LOCK: Mutex<()> = Mutex::new(());

struct EnvGuard;

impl EnvGuard {
    fn set_live() -> Self {
        std::env::set_var("OPTIONCLAW_ENABLE_LIVE_TRADING", "true");
        std::env::set_var("OPTIONCLAW_ALPACA_API_KEY", "integration-key");
        std::env::set_var("OPTIONCLAW_ALPACA_API_SECRET", "integration-secret");
        Self
    }

    fn clear() -> Self {
        std::env::remove_var("OPTIONCLAW_ENABLE_LIVE_TRADING");
        std::env::remove_var("OPTIONCLAW_ALPACA_API_KEY");
        std::env::remove_var("OPTIONCLAW_ALPACA_API_SECRET");
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        std::env::remove_var("OPTIONCLAW_ENABLE_LIVE_TRADING");
        std::env::remove_var("OPTIONCLAW_ALPACA_API_KEY");
        std::env::remove_var("OPTIONCLAW_ALPACA_API_SECRET");
    }
}

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be on or after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("optionclaw-ep011-{name}-{nanos}"));
    if path.exists() {
        let _ = fs::remove_dir_all(&path);
    }
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn toml_path(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn write_live_config(root: &Path, base_url: &str) -> (PathBuf, PathBuf, PathBuf) {
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
alpaca_base_url = "{}"
"#,
            toml_path(&kill_switch_path),
            toml_path(&approval_path),
            base_url
        ),
    )
    .expect("config should write");
    (config_path, approval_path, kill_switch_path)
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

fn approve_fixture(config_path: &Path, root: &Path) -> PathBuf {
    let report = run_backtest(config_path, write_research_fixture(root))
        .expect("backtest should produce evidence");
    approve_research(config_path, &report.report_path)
        .expect("approval should write")
        .approval_path
}

fn mock_account(server: &MockServer, options_level: u8) -> Mock<'_> {
    server.mock(move |when, then| {
        when.method(GET).path("/v2/account");
        then.status(200).json_body(json!({
            "id": "acct-live",
            "status": "ACTIVE",
            "options_approved_level": options_level,
            "options_trading_level": options_level,
            "trading_blocked": false,
            "equity": "100000.00",
            "buying_power": "50000.00"
        }));
    })
}

#[test]
fn live_check_passes_with_mocked_alpaca_and_fresh_approval() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::set_live();
    let root = unique_temp_dir("check-pass");
    let server = MockServer::start();
    let account = mock_account(&server, 2);
    let (config_path, _, _) = write_live_config(&root, &server.base_url());
    approve_fixture(&config_path, &root);

    let report = live_check(&config_path).expect("live check should pass");

    assert!(report.approved);
    assert!(report.approval_fresh);
    assert_eq!(report.options_approved_level, 2);
    account.assert_calls(1);
}

#[test]
fn live_check_fails_for_missing_secrets() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::clear();
    let root = unique_temp_dir("missing-secrets");
    let server = MockServer::start();
    let (config_path, _, _) = write_live_config(&root, &server.base_url());

    let err = live_check(&config_path).expect_err("missing secrets should fail");

    assert!(format!("{err}").contains("LIVE_TRADING_DISABLED"));
}

#[test]
fn live_check_fails_for_stale_approval() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::set_live();
    let root = unique_temp_dir("stale-approval");
    let server = MockServer::start();
    let (config_path, approval_path, _) = write_live_config(&root, &server.base_url());
    approve_fixture(&config_path, &root);
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

    let err = live_check(&config_path).expect_err("stale approval should fail");

    assert!(format!("{err}").contains("stale"));
}

#[test]
fn live_check_fails_for_mismatched_config_hash() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::set_live();
    let root = unique_temp_dir("hash-mismatch");
    let server = MockServer::start();
    let (config_path, approval_path, _) = write_live_config(&root, &server.base_url());
    approve_fixture(&config_path, &root);
    let mut artifact: Value = serde_json::from_str(
        &fs::read_to_string(&approval_path).expect("approval should be readable"),
    )
    .expect("approval should parse");
    artifact["config_hash"] = json!("mismatched-config-hash");
    fs::write(
        &approval_path,
        serde_json::to_string_pretty(&artifact).expect("artifact should serialize"),
    )
    .expect("artifact should write");

    let err = live_check(&config_path).expect_err("mismatch should fail");

    assert!(format!("{err}").contains("config hash"));
}

#[test]
fn live_check_fails_for_insufficient_options_level_and_provider_errors() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::set_live();
    let root = unique_temp_dir("insufficient-options");
    let server = MockServer::start();
    let account = mock_account(&server, 1);
    let (config_path, _, _) = write_live_config(&root, &server.base_url());
    approve_fixture(&config_path, &root);

    let err = live_check(&config_path).expect_err("low options level should fail");

    assert!(format!("{err}").contains("options approved/trading level 2"));
    account.assert_calls(1);

    let provider_root = unique_temp_dir("provider-down");
    let provider_server = MockServer::start();
    provider_server.mock(|when, then| {
        when.method(GET).path("/v2/account");
        then.status(503)
            .json_body(json!({"message": "unavailable"}));
    });
    let (provider_config, _, _) = write_live_config(&provider_root, &provider_server.base_url());
    approve_fixture(&provider_config, &provider_root);
    let err = live_check(&provider_config).expect_err("provider error should fail");
    assert!(format!("{err}").contains("Alpaca account status failed"));
}

#[test]
fn live_check_fails_when_kill_switch_is_active() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::set_live();
    let root = unique_temp_dir("kill-switch");
    let server = MockServer::start();
    let (config_path, _, kill_switch_path) = write_live_config(&root, &server.base_url());
    fs::create_dir_all(kill_switch_path.parent().expect("kill switch parent"))
        .expect("kill switch dir should exist");
    fs::write(&kill_switch_path, "STOP").expect("kill switch should write");

    let err = live_check(&config_path).expect_err("kill switch should fail");

    assert!(format!("{err}").contains("KILL_SWITCH_ACTIVE"));
}

#[test]
fn live_submit_submits_once_after_all_gates_pass() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::set_live();
    let root = unique_temp_dir("submit-once");
    let server = MockServer::start();
    let account = mock_account(&server, 2);
    let order = server.mock(|when, then| {
        when.method(POST).path("/v2/orders");
        then.status(200).json_body(json!({
            "id": "alpaca-order-1",
            "status": "accepted",
            "filled_qty": "0",
            "filled_avg_price": null
        }));
    });
    let (config_path, _, _) = write_live_config(&root, &server.base_url());
    approve_fixture(&config_path, &root);
    let order_path = write_live_order(&root);

    let report = live_submit(&config_path, &order_path, true).expect("submit should pass");

    assert!(report.submitted);
    assert_eq!(report.provider_order_id, "alpaca-order-1");
    account.assert_calls(1);
    order.assert_calls(1);
    let audit_path = root
        .join("var")
        .join("dev")
        .join("audit")
        .join("events.jsonl");
    let audit = fs::read_to_string(audit_path).expect("audit should be readable");
    assert!(audit.contains("LIVE_SUBMITTED"));
}
