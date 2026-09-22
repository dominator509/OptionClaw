use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use optionclaw::{
    observability::{
        drain_structured_logs_for_test, init_logging, record_metric, record_structured_log,
        reset_metrics_for_test, snapshot_metrics, LogLevel, MetricEvent, StructuredField,
        StructuredLogEvent,
    },
    services::{health, init_state},
};

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be on or after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("optionclaw-observability-{name}-{nanos}"));
    if path.exists() {
        let _ = fs::remove_dir_all(&path);
    }
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn write_paper_config(root: &Path) -> PathBuf {
    let config_dir = root.join("config");
    fs::create_dir_all(&config_dir).expect("config dir should be creatable");
    let config_path = config_dir.join("example.toml");
    fs::write(&config_path, "trading_mode = \"paper\"\n").expect("config should write");
    config_path
}

fn invalid_config_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join("config")
        .join("invalid_config.toml")
}

#[test]
fn structured_logs_redact_sensitive_fields() {
    drain_structured_logs_for_test();
    init_logging(LogLevel::Info);

    let line = record_structured_log(
        StructuredLogEvent::new(LogLevel::Info, "check-config", "load_config", "success")
            .with_mode("paper")
            .with_field(StructuredField::plain("config_path", "config/example.toml"))
            .with_field(StructuredField::redacted("api_key", "super-secret")),
    );

    let value: serde_json::Value =
        serde_json::from_str(&line).expect("structured log should parse");
    assert_eq!(value["fields"][1]["value"], "<redacted>");
    assert_ne!(value["fields"][1]["value"], "super-secret");
    assert_eq!(value["command"], "check-config");
}

#[test]
fn metrics_record_local_operational_signals() {
    reset_metrics_for_test();

    record_metric(MetricEvent::command_success("health"));
    record_metric(MetricEvent::command_failure("health", "E1001"));
    record_metric(MetricEvent::health_status(
        true, true, true, true, true, false,
    ));

    let snapshot = snapshot_metrics();
    assert_eq!(
        snapshot.get("command_success{command=health}"),
        Some(&1_u64)
    );
    assert_eq!(
        snapshot.get("command_failure{command=health,error_code=E1001}"),
        Some(&1_u64)
    );
    assert_eq!(
        snapshot.get(
            "health_status{audit_ready=true,config_ready=true,data_ready=true,kill_switch_active=false,providers_ready=true,secrets_store_ready=true}"
        ),
        Some(&1_u64)
    );
}

#[test]
fn health_fails_for_invalid_config() {
    let err = health(invalid_config_path()).expect_err("invalid config should fail health");
    assert!(err.to_string().contains("E1002"));
}

#[test]
fn health_success_emits_structured_signal() {
    drain_structured_logs_for_test();
    reset_metrics_for_test();

    // health() derives the data dir (<root>/var/dev) from the config path, so
    // build an isolated root and initialize state there: a fresh checkout
    // (including CI) has no <repo>/var/dev, which previously made this test
    // depend on local machine state.
    let root = unique_temp_dir("health");
    let config_path = write_paper_config(&root);
    init_state(root.join("var").join("dev")).expect("state should initialize");

    let report = health(&config_path).expect("health should succeed");
    assert!(report.config_ready);

    let snapshot = snapshot_metrics();
    assert!(snapshot.contains_key(
        "health_status{audit_ready=true,config_ready=true,data_ready=true,kill_switch_active=false,providers_ready=true,secrets_store_ready=true}"
    ));

    let logs = drain_structured_logs_for_test();
    assert!(logs
        .iter()
        .any(|line| line.contains("\"command\":\"health\"")
            && line.contains("\"result\":\"success\"")));

    let _ = fs::remove_dir_all(&root);
}
