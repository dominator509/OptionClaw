use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::domain::TradingMode;
use crate::errors::PersistenceError;

use super::{read_text, schema::SCHEMA_VERSION, write_atomic_text};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct PaperState {
    pub schema_version: u32,
    pub mode: String,
    pub account_id: String,
    pub equity_micros: Option<i64>,
    pub available_cash_micros: Option<i64>,
    pub daily_loss_bps: u32,
    pub positions: Vec<StoredPosition>,
    pub last_updated_unix_seconds: Option<u64>,
}

impl Default for PaperState {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            mode: TradingMode::Paper.to_string(),
            account_id: "paper".to_string(),
            equity_micros: None,
            available_cash_micros: None,
            daily_loss_bps: 0,
            positions: Vec::new(),
            last_updated_unix_seconds: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(default)]
pub struct StoredPosition {
    pub symbol: String,
    pub underlying: String,
    pub expiration: String,
    pub strike_micros: i64,
    pub kind: String,
    pub quantity: u32,
    pub average_entry_price_micros: i64,
}

pub fn write_state_atomic(
    path: impl AsRef<Path>,
    state: &PaperState,
) -> Result<(), PersistenceError> {
    let path = path.as_ref();
    if state.schema_version != SCHEMA_VERSION {
        return Err(PersistenceError::UnsupportedSchema {
            path: path.to_path_buf(),
            found: state.schema_version,
            expected: SCHEMA_VERSION,
        });
    }

    let json = serde_json::to_string_pretty(state).map_err(|source| PersistenceError::Corrupt {
        path: path.to_path_buf(),
        detail: source.to_string(),
    })?;
    write_atomic_text(path, &json)
}

pub fn read_state(path: impl AsRef<Path>) -> Result<PaperState, PersistenceError> {
    let path = path.as_ref();
    let contents = read_text(path)?;
    let state: PaperState =
        serde_json::from_str(&contents).map_err(|source| PersistenceError::Corrupt {
            path: path.to_path_buf(),
            detail: source.to_string(),
        })?;

    if state.schema_version != SCHEMA_VERSION {
        return Err(PersistenceError::UnsupportedSchema {
            path: path.to_path_buf(),
            found: state.schema_version,
            expected: SCHEMA_VERSION,
        });
    }

    if TradingMode::parse(&state.mode).is_err() {
        return Err(PersistenceError::Corrupt {
            path: path.to_path_buf(),
            detail: "invalid trading mode".to_string(),
        });
    }

    Ok(state)
}
