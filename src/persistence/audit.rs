use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{domain::TradingMode, errors::PersistenceError};

use super::{read_text, schema::AUDIT_FILE_NAME};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    InitDataDir,
    VerifyDataDir,
    StateWrite,
    StateRead,
    Backup,
    MigrationDryRun,
    RiskDecision,
    OrderIntent,
}

impl AuditEventType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitDataDir => "init_data_dir",
            Self::VerifyDataDir => "verify_data_dir",
            Self::StateWrite => "state_write",
            Self::StateRead => "state_read",
            Self::Backup => "backup",
            Self::MigrationDryRun => "migration_dry_run",
            Self::RiskDecision => "risk_decision",
            Self::OrderIntent => "order_intent",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub timestamp_unix_seconds: u64,
    pub event_type: AuditEventType,
    pub mode: String,
    pub order_intent_id: Option<String>,
    pub risk_decision: Option<String>,
    pub redacted_context: String,
}

impl AuditEvent {
    pub fn new(
        event_type: AuditEventType,
        mode: TradingMode,
        order_intent_id: Option<String>,
        risk_decision: Option<String>,
        redacted_context: impl Into<String>,
    ) -> Result<Self, PersistenceError> {
        let redacted_context = redacted_context.into();
        if redacted_context.trim().is_empty() {
            return Err(PersistenceError::Corrupt {
                path: Path::new(AUDIT_FILE_NAME).to_path_buf(),
                detail: "redacted context must not be empty".to_string(),
            });
        }

        Ok(Self {
            timestamp_unix_seconds: now_unix_seconds()?,
            event_type,
            mode: mode.to_string(),
            order_intent_id,
            risk_decision,
            redacted_context,
        })
    }
}

pub fn append_audit(path: impl AsRef<Path>, event: &AuditEvent) -> Result<(), PersistenceError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| PersistenceError::Unavailable {
            path: parent.to_path_buf(),
            source: Box::new(source),
        })?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|source| PersistenceError::AuditAppendFailed {
            path: path.to_path_buf(),
            source: Box::new(source),
        })?;

    let line = serde_json::to_string(event).map_err(|source| PersistenceError::Corrupt {
        path: path.to_path_buf(),
        detail: source.to_string(),
    })?;
    file.write_all(line.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.sync_all())
        .map_err(|source| PersistenceError::AuditAppendFailed {
            path: path.to_path_buf(),
            source: Box::new(source),
        })?;
    Ok(())
}

pub fn read_audit_events(path: impl AsRef<Path>) -> Result<Vec<AuditEvent>, PersistenceError> {
    let path = path.as_ref();
    let contents = read_text(path)?;
    let mut events = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let event = serde_json::from_str::<AuditEvent>(line).map_err(|source| {
            PersistenceError::Corrupt {
                path: path.to_path_buf(),
                detail: format!("line {}: {}", line_index + 1, source),
            }
        })?;
        events.push(event);
    }
    Ok(events)
}

fn now_unix_seconds() -> Result<u64, PersistenceError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| PersistenceError::Corrupt {
            path: Path::new(AUDIT_FILE_NAME).to_path_buf(),
            detail: "system clock is before Unix epoch".to_string(),
        })
}
