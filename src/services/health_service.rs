use std::path::{Path, PathBuf};

use crate::{
    config::AppConfig,
    errors::AppError,
    persistence::verify_data_dir,
    services::{derive_data_dir, state_service::verify_state},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthReport {
    pub config_path: PathBuf,
    pub data_dir: PathBuf,
    pub trading_mode: crate::domain::TradingMode,
    pub config_ready: bool,
    pub data_ready: bool,
    pub audit_ready: bool,
    pub mock_providers_ready: bool,
    pub kill_switch_active: bool,
}

pub fn health(config_path: impl AsRef<Path>) -> Result<HealthReport, AppError> {
    let config_path = config_path.as_ref().to_path_buf();
    let config = AppConfig::load_from_path(&config_path)?;
    let data_dir = derive_data_dir(&config_path);

    let mut data_ready = false;
    let mut audit_ready = false;
    let config_ready = true;

    if data_dir.exists() {
        if verify_state(&data_dir).is_ok() {
            data_ready = true;
        }
        audit_ready = verify_data_dir(&data_dir).is_ok();
    }

    Ok(HealthReport {
        config_path,
        data_dir,
        trading_mode: config.trading_mode,
        config_ready,
        data_ready,
        audit_ready,
        mock_providers_ready: true,
        kill_switch_active: false,
    })
}
