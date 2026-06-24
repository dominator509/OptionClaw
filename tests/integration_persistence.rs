use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use optionclaw::domain::TradingMode;
use optionclaw::persistence::{
    append_audit, backup_data_dir, init_data_dir, migrate_dry_run, read_audit_events, read_state,
    verify_data_dir, write_state_atomic, AuditEvent, AuditEventType, PaperState, StoredPosition,
};

fn unique_temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be on or after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("optionclaw-ep003-{name}-{nanos}"));
    if path.exists() {
        let _ = fs::remove_dir_all(&path);
    }
    fs::create_dir_all(&path).expect("temp dir should be creatable");
    path
}

fn write_invalid_schema(root: &Path, schema_version: u32) {
    let schema_path = root.join("schema.json");
    fs::write(
        schema_path,
        format!("{{\"schema_version\":{schema_version}}}"),
    )
    .expect("schema file should be writable");
}

fn copy_dir(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination dir should be creatable");
    for entry in fs::read_dir(source).expect("source dir should be readable") {
        let entry = entry.expect("entry should be readable");
        let target = destination.join(entry.file_name());
        let file_type = entry.file_type().expect("file type should be readable");
        if file_type.is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).expect("file should copy");
        }
    }
}

#[test]
fn init_is_idempotent_and_verify_succeeds() {
    let root = unique_temp_dir("init");

    let first = init_data_dir(&root).expect("initial init should succeed");
    assert!(first.created);

    let second = init_data_dir(&root).expect("re-init should succeed");
    assert!(!second.created);

    let verified = verify_data_dir(&root).expect("verify should succeed");
    assert!(verified.verified);
    assert_eq!(verified.schema_version, 1);

    assert!(root.join("schema.json").exists());
    assert!(root.join("audit").join("events.jsonl").exists());
    assert!(root.join("paper").join("state.json").exists());
    assert!(root.join("backups").exists());
}

#[test]
fn audit_append_is_readable_and_redacted() {
    let root = unique_temp_dir("audit");
    init_data_dir(&root).expect("init should succeed");

    let audit_path = root.join("audit").join("events.jsonl");
    let event = AuditEvent::new(
        AuditEventType::RiskDecision,
        TradingMode::Paper,
        Some("intent-1".to_string()),
        Some("RISK_ACCEPTED".to_string()),
        "redacted context only",
    )
    .expect("audit event should be valid");

    append_audit(&audit_path, &event).expect("audit append should succeed");
    let events = read_audit_events(&audit_path).expect("audit readback should succeed");
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].order_intent_id.as_deref(), Some("intent-1"));
    assert_eq!(events[0].risk_decision.as_deref(), Some("RISK_ACCEPTED"));

    let contents = fs::read_to_string(&audit_path).expect("audit file should be readable");
    assert!(contents.contains("redacted context only"));
    assert!(!contents.contains("secret"));
}

#[test]
fn write_state_is_atomic_and_round_trips() {
    let root = unique_temp_dir("state");
    let state_dir = root.join("paper");
    fs::create_dir_all(&state_dir).expect("state dir should be creatable");
    let state_path = state_dir.join("state.json");

    let state = PaperState {
        positions: vec![StoredPosition {
            symbol: "AAPL260201C00100000".to_string(),
            underlying: "AAPL".to_string(),
            expiration: "2026-02-01".to_string(),
            strike_micros: 1_000_000,
            kind: "call".to_string(),
            quantity: 1,
            average_entry_price_micros: 1_200_000,
        }],
        available_cash_micros: Some(5_000_000),
        equity_micros: Some(7_500_000),
        ..PaperState::default()
    };

    write_state_atomic(&state_path, &state).expect("state write should succeed");
    let round_trip = read_state(&state_path).expect("state readback should succeed");
    assert_eq!(round_trip.account_id, "paper");
    assert_eq!(round_trip.positions.len(), 1);
    assert_eq!(round_trip.positions[0].symbol, "AAPL260201C00100000");

    let tmp_files = fs::read_dir(&state_dir)
        .expect("state dir should be readable")
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_string_lossy().contains(".tmp"))
        .count();
    assert_eq!(tmp_files, 0);
}

#[test]
fn verify_rejects_corrupt_state_and_unsupported_schema() {
    let root = unique_temp_dir("corrupt");
    init_data_dir(&root).expect("init should succeed");

    fs::write(root.join("paper").join("state.json"), "{not json}")
        .expect("state should be writable");
    let corrupt = verify_data_dir(&root).expect_err("corrupt state must fail");
    assert!(format!("{corrupt}").contains("PERSISTENCE_CORRUPT"));

    let unsupported_root = unique_temp_dir("unsupported");
    init_data_dir(&unsupported_root).expect("init should succeed");
    write_invalid_schema(&unsupported_root, 2);
    let unsupported = verify_data_dir(&unsupported_root).expect_err("unsupported schema must fail");
    assert!(format!("{unsupported}").contains("SCHEMA_UNSUPPORTED"));
}

#[test]
fn backup_can_be_restored_from_copy() {
    let source = unique_temp_dir("backup-source");
    init_data_dir(&source).expect("init should succeed");

    let state_path = source.join("paper").join("state.json");
    let mut state = read_state(&state_path).expect("state should be readable");
    state.available_cash_micros = Some(1_000_000);
    write_state_atomic(&state_path, &state).expect("state update should succeed");

    let backup = unique_temp_dir("backup-target");
    fs::remove_dir_all(&backup).expect("backup placeholder dir should be removable");
    let report = backup_data_dir(&source, &backup).expect("backup should succeed");
    assert_eq!(report.files_copied, 3);
    assert!(backup.join("schema.json").exists());
    assert!(backup.join("paper").join("state.json").exists());

    let restored = unique_temp_dir("backup-restored");
    copy_dir(&backup, &restored);
    let verified = verify_data_dir(&restored).expect("restored backup should verify");
    assert_eq!(verified.schema_version, 1);
}

#[test]
fn dry_run_migration_is_noop_for_schema_v1() {
    let root = unique_temp_dir("migrate");
    init_data_dir(&root).expect("init should succeed");

    let report = migrate_dry_run(&root).expect("dry run should succeed");
    assert!(report.dry_run);
    assert!(!report.would_modify);
    assert_eq!(report.from_version, 1);
    assert_eq!(report.to_version, 1);
}
