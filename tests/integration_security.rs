use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use optionclaw::{
    config::{AppConfig, TradingMode},
    errors::{SecurityError, SecurityErrorCode},
    execution::authorize_live_mode,
    risk::{authorize_execution, ExecutionGates},
    secrets::{reject_plaintext_secret_file, DisabledSecretStore, MemorySecretStore, SecretStore},
};

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be on or after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("optionclaw-ep006-{name}-{nanos}"));
    if path.exists() {
        let _ = fs::remove_dir_all(&path);
    }
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

#[test]
fn redaction_hides_sensitive_values() {
    let secret = optionclaw::secrets::Redacted::new("super-secret".to_string());
    assert_eq!(format!("{secret}"), "<redacted>");
    assert!(!format!("{secret:?}").contains("super-secret"));
}

#[test]
fn missing_secret_fails_closed() {
    let store = DisabledSecretStore;
    let err = store
        .load("broker_api_key")
        .expect_err("missing secret should fail");
    assert!(format!("{err}").contains("SECRET_MISSING"));
}

#[test]
fn paper_mode_does_not_require_secrets() {
    let config = AppConfig {
        trading_mode: TradingMode::Paper,
        ..AppConfig::default()
    };
    assert!(config.validate_security().is_ok());
    assert!(authorize_execution(TradingMode::Paper, ExecutionGates::default()).is_ok());
}

#[test]
fn live_mode_fails_closed_without_gates() {
    let err = authorize_execution(TradingMode::Live, ExecutionGates::default())
        .expect_err("live mode should fail closed");
    assert!(err.to_string().contains("LIVE_TRADING_DISABLED"));
    assert!(authorize_live_mode().is_err());
}

#[test]
fn kill_switch_active_blocks_execution() {
    let err = authorize_execution(
        TradingMode::Paper,
        ExecutionGates {
            kill_switch_active: true,
            ..ExecutionGates::default()
        },
    )
    .expect_err("kill switch should block execution");
    assert!(format!("{err}").contains("KILL_SWITCH_ACTIVE"));
}

#[test]
fn plaintext_secret_file_is_rejected() {
    let root = unique_temp_dir("plaintext");
    let secret_path = root.join("secret.txt");
    fs::write(&secret_path, "api_key = \"super-secret\"").expect("secret file should write");

    let err = reject_plaintext_secret_file(&secret_path)
        .expect_err("plaintext secret file should be rejected");
    assert!(format!("{err}").contains("SECRET_PLAINTEXT_REJECTED"));
}

#[test]
fn memory_secret_store_round_trips_without_plaintext_printing() {
    let store = MemorySecretStore::new();
    store
        .store("llm_api_key", "opclaw_fake_key".into())
        .expect("store should succeed");

    let secret = store.load("llm_api_key").expect("load should succeed");
    assert_eq!(secret.expose(), "opclaw_fake_key");
    assert_eq!(format!("{secret}"), "<redacted>");
}

#[test]
fn security_error_codes_are_stable() {
    let err = SecurityError::SecretStorageDisabled { path: None };
    assert_eq!(
        err.code().as_str(),
        SecurityErrorCode::SecretStorageDisabled.as_str()
    );
}
