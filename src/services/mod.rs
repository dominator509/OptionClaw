use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::de::DeserializeOwned;

use crate::errors::{AppError, InputError};

pub mod config_service;
pub mod health_service;
pub mod paper_service;
pub mod risk_service;
pub mod state_service;

pub use config_service::{check_config, ConfigReport};
pub use health_service::{health, HealthReport};
pub use paper_service::{run_paper_once, PaperExecutionStatus, PaperRunReport};
pub use risk_service::{explain_risk, RiskExplainFixture, RiskReport};
pub use state_service::{init_state, verify_state};

pub(crate) fn load_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, AppError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| {
        AppError::from(InputError::Invalid {
            path: path.to_path_buf(),
            detail: source.to_string(),
        })
    })?;

    serde_json::from_str(&contents).map_err(|source| {
        AppError::from(InputError::Invalid {
            path: path.to_path_buf(),
            detail: source.to_string(),
        })
    })
}

pub(crate) fn derive_data_dir(config_path: impl AsRef<Path>) -> PathBuf {
    let config_path = config_path.as_ref();
    let config_dir = config_path.parent().unwrap_or_else(|| Path::new("."));
    let repo_root = config_dir.parent().unwrap_or(config_dir);
    repo_root.join("var").join("dev")
}
